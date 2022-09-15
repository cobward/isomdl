use crate::{
    definitions::{
        device_engagement::RetrievalOptions,
        helpers::{ByteStr, NonEmptyMap, NonEmptyVec, Tag24},
        issuer_signed::{IssuerNamespaces, IssuerSignedItemBytes},
        session::{Curves, EncodedPoints},
        DeviceEngagement, DeviceKeyInfo, DigestAlgorithm, DigestId, DigestIds, IssuerSignedItem,
        Mso, ValidityInfo,
    },
    issuance::x5chain::{X5Chain, X5CHAIN_HEADER_LABEL},
};
use anyhow::{anyhow, Result};
use aws_nitro_enclaves_cose::{
    crypto::{Openssl, SignatureAlgorithm, SigningPrivateKey, SigningPublicKey},
    header_map::HeaderMap,
    CoseSign1,
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_cbor::Value as CborValue;
use std::collections::{HashMap, HashSet};

pub type Namespaces = HashMap<String, HashMap<String, CborValue>>;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Representation of an issued mdoc.
pub struct Mdoc {
    doc_type: String,
    namespaces: IssuerNamespaces,
    issuer_auth: CoseSign1,
}

#[derive(Debug, Clone)]
pub struct MdocPreparation {
    doc_type: String,
    namespaces: IssuerNamespaces,
    mso: Mso,
    x5chain: X5Chain,
}

impl Mdoc {
    pub fn prepare_mdoc(
        doc_type: String,
        namespaces: Namespaces,
        x5chain: X5Chain,
        validity_info: ValidityInfo,
        digest_algorithm: DigestAlgorithm,
        device_key_info: DeviceKeyInfo,
    ) -> Result<MdocPreparation> {
        if let Some(authorizations) = &device_key_info.key_authorizations {
            authorizations.validate()?;
        }

        let issuer_namespaces = to_issuer_namespaces(namespaces)?;
        let value_digests = digest_namespaces(&issuer_namespaces, digest_algorithm)?;

        let mso = Mso {
            version: "1.0".to_string(),
            digest_algorithm,
            value_digests,
            device_key_info,
            doc_type: doc_type.clone(),
            validity_info,
        };

        let preparation_mdoc = MdocPreparation {
            doc_type,
            namespaces: issuer_namespaces,
            mso,
            x5chain,
        };

        Ok(preparation_mdoc)
    }

    pub fn issue<T: SigningPrivateKey + SigningPublicKey>(
        doc_type: String,
        namespaces: Namespaces,
        x5chain: X5Chain,
        validity_info: ValidityInfo,
        digest_algorithm: DigestAlgorithm,
        device_key_info: DeviceKeyInfo,
        signer: T,
    ) -> Result<Mdoc> {
        Self::prepare_mdoc(
            doc_type,
            namespaces,
            x5chain,
            validity_info,
            digest_algorithm,
            device_key_info,
        )?
        .complete(signer)
    }
}

impl MdocPreparation {
    pub fn complete<T: SigningPrivateKey + SigningPublicKey>(self, signer: T) -> Result<Mdoc> {
        let MdocPreparation {
            doc_type,
            namespaces,
            mso,
            x5chain,
        } = self;

        let signer_algorithm = signer
            .get_parameters()
            .map_err(|error| anyhow!("error getting signer parameters: {error}"))?
            .0;

        // Should/can we assert that the signer is the key identified by the x5chain?
        match (x5chain.key_algorithm()?, signer_algorithm) {
            (SignatureAlgorithm::ES256, SignatureAlgorithm::ES256) => (),
            (SignatureAlgorithm::ES384, SignatureAlgorithm::ES384) => (),
            (SignatureAlgorithm::ES512, SignatureAlgorithm::ES512) => (),
            (chain_alg, signer_alg) => Err(anyhow!(
                "signature algorithm does not match: expected '{:?}' (from x5chain), found '{:?}' (from signer)"
                , chain_alg, signer_alg
            ))?,
        }

        // encode mso to cbor
        let mso_bytes = serde_cbor::to_vec(&Tag24::new(mso)?)?;

        //headermap should contain alg header and x5chain header
        let protected_headers: HeaderMap = signer_algorithm.into();

        let mut unprotected_headers = HeaderMap::new();
        unprotected_headers.insert(
            serde_cbor::Value::Integer(X5CHAIN_HEADER_LABEL),
            x5chain.into_cbor()?,
        );

        let cose_sign1 = CoseSign1::new_with_protected::<Openssl>(
            &mso_bytes,
            &protected_headers,
            &unprotected_headers,
            &signer,
        )
        .map_err(|error| anyhow!("error signing mso: {error}"))?;

        let mdoc = Mdoc {
            doc_type,
            namespaces,
            issuer_auth: cose_sign1,
        };

        Ok(mdoc)
    }
}

fn to_issuer_namespaces(namespaces: Namespaces) -> Result<IssuerNamespaces> {
    namespaces
        .into_iter()
        .map(|(name, elements)| {
            to_issuer_signed_items(elements)
                .map(Tag24::new)
                .collect::<Result<Vec<Tag24<IssuerSignedItem>>, _>>()
                .map_err(|err| anyhow!("unable to encode IssuerSignedItem as cbor: {}", err))
                .and_then(|items| {
                    NonEmptyVec::try_from(items)
                        .map_err(|_| anyhow!("at least one element required in each namespace"))
                })
                .map(|elems| (name, elems))
        })
        .collect::<Result<HashMap<String, NonEmptyVec<Tag24<IssuerSignedItem>>>>>()
        .and_then(|namespaces| {
            NonEmptyMap::try_from(namespaces)
                .map_err(|_| anyhow!("at least one namespace required"))
        })
}

fn to_issuer_signed_items(
    elements: HashMap<String, CborValue>,
) -> impl Iterator<Item = IssuerSignedItem> {
    let mut used_ids = HashSet::new();
    elements.into_iter().map(move |(key, value)| {
        let digest_id = generate_digest_id(&mut used_ids);
        let random = Vec::from(rand::thread_rng().gen::<[u8; 16]>()).into();
        IssuerSignedItem {
            digest_id,
            random,
            element_identifier: key,
            element_value: value,
        }
    })
}

fn digest_namespaces(
    namespaces: &IssuerNamespaces,
    digest_algorithm: DigestAlgorithm,
) -> Result<HashMap<String, DigestIds>> {
    namespaces
        .iter()
        .map(|(name, elements)| Ok((name.clone(), digest_namespace(elements, digest_algorithm)?)))
        .collect()
}

fn digest_namespace(
    elements: &[IssuerSignedItemBytes],
    digest_algorithm: DigestAlgorithm,
) -> Result<DigestIds> {
    let ring_alg = match digest_algorithm {
        DigestAlgorithm::SHA256 => &ring::digest::SHA256,
        DigestAlgorithm::SHA384 => &ring::digest::SHA384,
        DigestAlgorithm::SHA512 => &ring::digest::SHA512,
    };
    let mut used_ids = elements
        .iter()
        .map(|item| item.as_ref().digest_id)
        .collect();

    // Generate X random digests to avoid leaking information.
    let random_ids = std::iter::repeat_with(|| generate_digest_id(&mut used_ids));
    let random_bytes = std::iter::repeat_with(|| {
        std::iter::repeat_with(|| rand::thread_rng().gen::<u8>())
            .take(512)
            .collect()
    });
    let random_digests = random_ids
        .zip(random_bytes)
        .map(Result::<_, anyhow::Error>::Ok)
        .take(rand::thread_rng().gen_range(5..10));

    elements
        .iter()
        .map(|item| Ok((item.as_ref().digest_id, serde_cbor::to_vec(item)?)))
        .chain(random_digests)
        .map(|result| {
            let (digest_id, bytes) = result?;
            let digest = ring::digest::digest(ring_alg, &bytes);
            return Ok((digest_id, digest.as_ref().to_vec().into()));
        })
        .collect()
}

fn generate_digest_id(used_ids: &mut HashSet<DigestId>) -> DigestId {
    let mut digest_id;
    loop {
        digest_id = rand::thread_rng().gen();
        if used_ids.insert(digest_id) {
            break;
        }
    }
    digest_id
}

fn prepare_device_engagement(
    crv: Curves,
    retrieval_option: RetrievalOptions,
    encoded_point: EncodedPoints,
) -> Result<DeviceEngagement> {
    let cipher_suite_identifier = get_cypher_suite_identifier(crv);
    let type_and_version = get_transport_type_and_version(retrieval_option.clone())?;

    let device_retrieval_option = (type_and_version.0, type_and_version.1, retrieval_option);
    let device_retrieval_options = vec![device_retrieval_option];

    let device_engagement = DeviceEngagement {
        version: "1.0".to_string(),
        security: (cipher_suite_identifier, encoded_point.into()),
        device_retrieval_methods: Some(device_retrieval_options),
        //server_retrieval is not implemented
        server_retrieval_methods: None,
        //protocol_info is not implemented
        protocol_info: None,
    };

    Ok(device_engagement)
}

fn get_cypher_suite_identifier(crv: Curves) -> u64 {
    match crv {
        P256 => 1,
        P384 => 2,
        P521 => 3,
        X25519 => 4,
        X448 => 5,
        Ed25519 => 6,
        Ed448 => 7,
    }
}

fn get_transport_type_and_version(retrieval_option: RetrievalOptions) -> Result<(u64, u64)> {
    match retrieval_option {
        NFCOPTIONS => Ok((1, 1)),
        BLEOPTIONS => Ok((2, 1)),
        WIFIOPTIONS => Ok((3, 1)),
        _ => Err(anyhow!("retrieval option not recognized")),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::definitions::{session::generate_ephemeral_keys, BleOptions, KeyAuthorizations};
    use hex::FromHex;

    static ISSUER_CERT: &[u8] = include_bytes!("../../test/issuance/256-cert.pem");
    static ISSUER_KEY: &[u8] = include_bytes!("../../test/issuance/256-key.pem");
    static COSE_KEY: &str = include_str!("../../test/definitions/cose_key/ec_p256.cbor");

    #[test]
    fn issue_minimal_mdoc() {
        let doc_type = String::from("org.iso.18013.5.1.mDL");

        let mdl_namespace = String::from("org.iso.18013.5.1");
        let key = String::from("family_name");
        let value = String::from("Smith").into();
        let mdl_elements = [(key, value)].into_iter().collect();
        let namespaces = [(mdl_namespace.clone(), mdl_elements)]
            .into_iter()
            .collect();

        let x5chain = X5Chain::builder()
            .with_pem(ISSUER_CERT)
            .unwrap()
            .build()
            .unwrap();

        let validity_info = ValidityInfo {
            signed: Default::default(),
            valid_from: Default::default(),
            valid_until: Default::default(),
            expected_update: None,
        };

        let digest_algorithm = DigestAlgorithm::SHA256;

        let device_key_bytes =
            <Vec<u8>>::from_hex(COSE_KEY).expect("unable to convert cbor hex to bytes");
        let device_key = serde_cbor::from_slice(&device_key_bytes).unwrap();
        let device_key_info = DeviceKeyInfo {
            device_key,
            key_authorizations: Some(KeyAuthorizations {
                namespaces: Some(NonEmptyVec::new(mdl_namespace)),
                data_elements: None,
            }),
            key_info: None,
        };

        let signer = openssl::pkey::PKey::private_key_from_pem(ISSUER_KEY).unwrap();

        Mdoc::issue(
            doc_type,
            namespaces,
            x5chain,
            validity_info,
            digest_algorithm,
            device_key_info,
            signer,
        )
        .expect("failed to issue mdoc");
    }

    #[test]
    fn device_engagement() {
        let crv = Curves::P256;
        let key_pair = generate_ephemeral_keys(crv.clone());
        let encoded_point = key_pair.unwrap().1;

        let uuid_bytes: Vec<u8> = vec![1, 2, 3, 4, 5];
        let address_bytes: Vec<u8> = vec![6, 7, 8, 9, 0];

        let ble_option = BleOptions {
            peripheral_server_mode: false,
            central_client_mode: true,
            peripheral_server_uuid: None,
            client_central_uuid: Some(ByteStr::from(uuid_bytes)),
            mdoc_ble_device_address_peripheral_server: Some(ByteStr::from(address_bytes)),
        };

        prepare_device_engagement(crv, RetrievalOptions::BLEOPTIONS(ble_option), encoded_point)
            .expect("failed to prepare for device engagement");
    }
}
