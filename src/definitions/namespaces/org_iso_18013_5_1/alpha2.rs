use crate::definitions::traits::{FromJson, FromJsonError};
use serde_cbor::Value as Cbor;
use serde_json::Value as Json;
use std::str::FromStr;

/// ISO 3166-1 alpha-2 country code.
#[derive(Clone, Debug)]
pub enum Alpha2 {
    AD,
    AE,
    AF,
    AG,
    AI,
    AL,
    AM,
    AO,
    AQ,
    AR,
    AS,
    AT,
    AU,
    AW,
    AX,
    AZ,
    BA,
    BB,
    BD,
    BE,
    BF,
    BG,
    BH,
    BI,
    BJ,
    BL,
    BM,
    BN,
    BO,
    BQ,
    BR,
    BS,
    BT,
    BV,
    BW,
    BY,
    BZ,
    CA,
    CC,
    CD,
    CF,
    CG,
    CH,
    CI,
    CK,
    CL,
    CM,
    CN,
    CO,
    CR,
    CU,
    CV,
    CW,
    CX,
    CY,
    CZ,
    DE,
    DJ,
    DK,
    DM,
    DO,
    DZ,
    EC,
    EE,
    EG,
    EH,
    ER,
    ES,
    ET,
    FI,
    FJ,
    FK,
    FM,
    FO,
    FR,
    GA,
    GB,
    GD,
    GE,
    GF,
    GG,
    GH,
    GI,
    GL,
    GM,
    GN,
    GP,
    GQ,
    GR,
    GS,
    GT,
    GU,
    GW,
    GY,
    HK,
    HM,
    HN,
    HR,
    HT,
    HU,
    ID,
    IE,
    IL,
    IM,
    IN,
    IO,
    IQ,
    IR,
    IS,
    IT,
    JE,
    JM,
    JO,
    JP,
    KE,
    KG,
    KH,
    KI,
    KM,
    KN,
    KP,
    KR,
    KW,
    KY,
    KZ,
    LA,
    LB,
    LC,
    LI,
    LK,
    LR,
    LS,
    LT,
    LU,
    LV,
    LY,
    MA,
    MC,
    MD,
    ME,
    MF,
    MG,
    MH,
    MK,
    ML,
    MM,
    MN,
    MO,
    MP,
    MQ,
    MR,
    MS,
    MT,
    MU,
    MV,
    MW,
    MX,
    MY,
    MZ,
    NA,
    NC,
    NE,
    NF,
    NG,
    NI,
    NL,
    NO,
    NP,
    NR,
    NU,
    NZ,
    OM,
    PA,
    PE,
    PF,
    PG,
    PH,
    PK,
    PL,
    PM,
    PN,
    PR,
    PS,
    PT,
    PW,
    PY,
    QA,
    RE,
    RO,
    RS,
    RU,
    RW,
    SA,
    SB,
    SC,
    SD,
    SE,
    SG,
    SH,
    SI,
    SJ,
    SK,
    SL,
    SM,
    SN,
    SO,
    SR,
    SS,
    ST,
    SV,
    SX,
    SY,
    SZ,
    TC,
    TD,
    TF,
    TG,
    TH,
    TJ,
    TK,
    TL,
    TM,
    TN,
    TO,
    TR,
    TT,
    TV,
    TW,
    TZ,
    UA,
    UG,
    UM,
    US,
    UY,
    UZ,
    VA,
    VC,
    VE,
    VG,
    VI,
    VN,
    VU,
    WF,
    WS,
    YE,
    YT,
    ZA,
    ZM,
    ZW,
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("unrecognized country code: {0}")]
    Unrecognized(String),
}

impl From<Alpha2> for Cbor {
    fn from(a: Alpha2) -> Cbor {
        a.as_str().to_string().into()
    }
}

impl Alpha2 {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AD => "AD",
            Self::AE => "AE",
            Self::AF => "AF",
            Self::AG => "AG",
            Self::AI => "AI",
            Self::AL => "AL",
            Self::AM => "AM",
            Self::AO => "AO",
            Self::AQ => "AQ",
            Self::AR => "AR",
            Self::AS => "AS",
            Self::AT => "AT",
            Self::AU => "AU",
            Self::AW => "AW",
            Self::AX => "AX",
            Self::AZ => "AZ",
            Self::BA => "BA",
            Self::BB => "BB",
            Self::BD => "BD",
            Self::BE => "BE",
            Self::BF => "BF",
            Self::BG => "BG",
            Self::BH => "BH",
            Self::BI => "BI",
            Self::BJ => "BJ",
            Self::BL => "BL",
            Self::BM => "BM",
            Self::BN => "BN",
            Self::BO => "BO",
            Self::BQ => "BQ",
            Self::BR => "BR",
            Self::BS => "BS",
            Self::BT => "BT",
            Self::BV => "BV",
            Self::BW => "BW",
            Self::BY => "BY",
            Self::BZ => "BZ",
            Self::CA => "CA",
            Self::CC => "CC",
            Self::CD => "CD",
            Self::CF => "CF",
            Self::CG => "CG",
            Self::CH => "CH",
            Self::CI => "CI",
            Self::CK => "CK",
            Self::CL => "CL",
            Self::CM => "CM",
            Self::CN => "CN",
            Self::CO => "CO",
            Self::CR => "CR",
            Self::CU => "CU",
            Self::CV => "CV",
            Self::CW => "CW",
            Self::CX => "CX",
            Self::CY => "CY",
            Self::CZ => "CZ",
            Self::DE => "DE",
            Self::DJ => "DJ",
            Self::DK => "DK",
            Self::DM => "DM",
            Self::DO => "DO",
            Self::DZ => "DZ",
            Self::EC => "EC",
            Self::EE => "EE",
            Self::EG => "EG",
            Self::EH => "EH",
            Self::ER => "ER",
            Self::ES => "ES",
            Self::ET => "ET",
            Self::FI => "FI",
            Self::FJ => "FJ",
            Self::FK => "FK",
            Self::FM => "FM",
            Self::FO => "FO",
            Self::FR => "FR",
            Self::GA => "GA",
            Self::GB => "GB",
            Self::GD => "GD",
            Self::GE => "GE",
            Self::GF => "GF",
            Self::GG => "GG",
            Self::GH => "GH",
            Self::GI => "GI",
            Self::GL => "GL",
            Self::GM => "GM",
            Self::GN => "GN",
            Self::GP => "GP",
            Self::GQ => "GQ",
            Self::GR => "GR",
            Self::GS => "GS",
            Self::GT => "GT",
            Self::GU => "GU",
            Self::GW => "GW",
            Self::GY => "GY",
            Self::HK => "HK",
            Self::HM => "HM",
            Self::HN => "HN",
            Self::HR => "HR",
            Self::HT => "HT",
            Self::HU => "HU",
            Self::ID => "ID",
            Self::IE => "IE",
            Self::IL => "IL",
            Self::IM => "IM",
            Self::IN => "IN",
            Self::IO => "IO",
            Self::IQ => "IQ",
            Self::IR => "IR",
            Self::IS => "IS",
            Self::IT => "IT",
            Self::JE => "JE",
            Self::JM => "JM",
            Self::JO => "JO",
            Self::JP => "JP",
            Self::KE => "KE",
            Self::KG => "KG",
            Self::KH => "KH",
            Self::KI => "KI",
            Self::KM => "KM",
            Self::KN => "KN",
            Self::KP => "KP",
            Self::KR => "KR",
            Self::KW => "KW",
            Self::KY => "KY",
            Self::KZ => "KZ",
            Self::LA => "LA",
            Self::LB => "LB",
            Self::LC => "LC",
            Self::LI => "LI",
            Self::LK => "LK",
            Self::LR => "LR",
            Self::LS => "LS",
            Self::LT => "LT",
            Self::LU => "LU",
            Self::LV => "LV",
            Self::LY => "LY",
            Self::MA => "MA",
            Self::MC => "MC",
            Self::MD => "MD",
            Self::ME => "ME",
            Self::MF => "MF",
            Self::MG => "MG",
            Self::MH => "MH",
            Self::MK => "MK",
            Self::ML => "ML",
            Self::MM => "MM",
            Self::MN => "MN",
            Self::MO => "MO",
            Self::MP => "MP",
            Self::MQ => "MQ",
            Self::MR => "MR",
            Self::MS => "MS",
            Self::MT => "MT",
            Self::MU => "MU",
            Self::MV => "MV",
            Self::MW => "MW",
            Self::MX => "MX",
            Self::MY => "MY",
            Self::MZ => "MZ",
            Self::NA => "NA",
            Self::NC => "NC",
            Self::NE => "NE",
            Self::NF => "NF",
            Self::NG => "NG",
            Self::NI => "NI",
            Self::NL => "NL",
            Self::NO => "NO",
            Self::NP => "NP",
            Self::NR => "NR",
            Self::NU => "NU",
            Self::NZ => "NZ",
            Self::OM => "OM",
            Self::PA => "PA",
            Self::PE => "PE",
            Self::PF => "PF",
            Self::PG => "PG",
            Self::PH => "PH",
            Self::PK => "PK",
            Self::PL => "PL",
            Self::PM => "PM",
            Self::PN => "PN",
            Self::PR => "PR",
            Self::PS => "PS",
            Self::PT => "PT",
            Self::PW => "PW",
            Self::PY => "PY",
            Self::QA => "QA",
            Self::RE => "RE",
            Self::RO => "RO",
            Self::RS => "RS",
            Self::RU => "RU",
            Self::RW => "RW",
            Self::SA => "SA",
            Self::SB => "SB",
            Self::SC => "SC",
            Self::SD => "SD",
            Self::SE => "SE",
            Self::SG => "SG",
            Self::SH => "SH",
            Self::SI => "SI",
            Self::SJ => "SJ",
            Self::SK => "SK",
            Self::SL => "SL",
            Self::SM => "SM",
            Self::SN => "SN",
            Self::SO => "SO",
            Self::SR => "SR",
            Self::SS => "SS",
            Self::ST => "ST",
            Self::SV => "SV",
            Self::SX => "SX",
            Self::SY => "SY",
            Self::SZ => "SZ",
            Self::TC => "TC",
            Self::TD => "TD",
            Self::TF => "TF",
            Self::TG => "TG",
            Self::TH => "TH",
            Self::TJ => "TJ",
            Self::TK => "TK",
            Self::TL => "TL",
            Self::TM => "TM",
            Self::TN => "TN",
            Self::TO => "TO",
            Self::TR => "TR",
            Self::TT => "TT",
            Self::TV => "TV",
            Self::TW => "TW",
            Self::TZ => "TZ",
            Self::UA => "UA",
            Self::UG => "UG",
            Self::UM => "UM",
            Self::US => "US",
            Self::UY => "UY",
            Self::UZ => "UZ",
            Self::VA => "VA",
            Self::VC => "VC",
            Self::VE => "VE",
            Self::VG => "VG",
            Self::VI => "VI",
            Self::VN => "VN",
            Self::VU => "VU",
            Self::WF => "WF",
            Self::WS => "WS",
            Self::YE => "YE",
            Self::YT => "YT",
            Self::ZA => "ZA",
            Self::ZM => "ZM",
            Self::ZW => "ZW",
        }
    }
}

impl FromStr for Alpha2 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "AD" => Ok(Self::AD),
            "AE" => Ok(Self::AE),
            "AF" => Ok(Self::AF),
            "AG" => Ok(Self::AG),
            "AI" => Ok(Self::AI),
            "AL" => Ok(Self::AL),
            "AM" => Ok(Self::AM),
            "AO" => Ok(Self::AO),
            "AQ" => Ok(Self::AQ),
            "AR" => Ok(Self::AR),
            "AS" => Ok(Self::AS),
            "AT" => Ok(Self::AT),
            "AU" => Ok(Self::AU),
            "AW" => Ok(Self::AW),
            "AX" => Ok(Self::AX),
            "AZ" => Ok(Self::AZ),
            "BA" => Ok(Self::BA),
            "BB" => Ok(Self::BB),
            "BD" => Ok(Self::BD),
            "BE" => Ok(Self::BE),
            "BF" => Ok(Self::BF),
            "BG" => Ok(Self::BG),
            "BH" => Ok(Self::BH),
            "BI" => Ok(Self::BI),
            "BJ" => Ok(Self::BJ),
            "BL" => Ok(Self::BL),
            "BM" => Ok(Self::BM),
            "BN" => Ok(Self::BN),
            "BO" => Ok(Self::BO),
            "BQ" => Ok(Self::BQ),
            "BR" => Ok(Self::BR),
            "BS" => Ok(Self::BS),
            "BT" => Ok(Self::BT),
            "BV" => Ok(Self::BV),
            "BW" => Ok(Self::BW),
            "BY" => Ok(Self::BY),
            "BZ" => Ok(Self::BZ),
            "CA" => Ok(Self::CA),
            "CC" => Ok(Self::CC),
            "CD" => Ok(Self::CD),
            "CF" => Ok(Self::CF),
            "CG" => Ok(Self::CG),
            "CH" => Ok(Self::CH),
            "CI" => Ok(Self::CI),
            "CK" => Ok(Self::CK),
            "CL" => Ok(Self::CL),
            "CM" => Ok(Self::CM),
            "CN" => Ok(Self::CN),
            "CO" => Ok(Self::CO),
            "CR" => Ok(Self::CR),
            "CU" => Ok(Self::CU),
            "CV" => Ok(Self::CV),
            "CW" => Ok(Self::CW),
            "CX" => Ok(Self::CX),
            "CY" => Ok(Self::CY),
            "CZ" => Ok(Self::CZ),
            "DE" => Ok(Self::DE),
            "DJ" => Ok(Self::DJ),
            "DK" => Ok(Self::DK),
            "DM" => Ok(Self::DM),
            "DO" => Ok(Self::DO),
            "DZ" => Ok(Self::DZ),
            "EC" => Ok(Self::EC),
            "EE" => Ok(Self::EE),
            "EG" => Ok(Self::EG),
            "EH" => Ok(Self::EH),
            "ER" => Ok(Self::ER),
            "ES" => Ok(Self::ES),
            "ET" => Ok(Self::ET),
            "FI" => Ok(Self::FI),
            "FJ" => Ok(Self::FJ),
            "FK" => Ok(Self::FK),
            "FM" => Ok(Self::FM),
            "FO" => Ok(Self::FO),
            "FR" => Ok(Self::FR),
            "GA" => Ok(Self::GA),
            "GB" => Ok(Self::GB),
            "GD" => Ok(Self::GD),
            "GE" => Ok(Self::GE),
            "GF" => Ok(Self::GF),
            "GG" => Ok(Self::GG),
            "GH" => Ok(Self::GH),
            "GI" => Ok(Self::GI),
            "GL" => Ok(Self::GL),
            "GM" => Ok(Self::GM),
            "GN" => Ok(Self::GN),
            "GP" => Ok(Self::GP),
            "GQ" => Ok(Self::GQ),
            "GR" => Ok(Self::GR),
            "GS" => Ok(Self::GS),
            "GT" => Ok(Self::GT),
            "GU" => Ok(Self::GU),
            "GW" => Ok(Self::GW),
            "GY" => Ok(Self::GY),
            "HK" => Ok(Self::HK),
            "HM" => Ok(Self::HM),
            "HN" => Ok(Self::HN),
            "HR" => Ok(Self::HR),
            "HT" => Ok(Self::HT),
            "HU" => Ok(Self::HU),
            "ID" => Ok(Self::ID),
            "IE" => Ok(Self::IE),
            "IL" => Ok(Self::IL),
            "IM" => Ok(Self::IM),
            "IN" => Ok(Self::IN),
            "IO" => Ok(Self::IO),
            "IQ" => Ok(Self::IQ),
            "IR" => Ok(Self::IR),
            "IS" => Ok(Self::IS),
            "IT" => Ok(Self::IT),
            "JE" => Ok(Self::JE),
            "JM" => Ok(Self::JM),
            "JO" => Ok(Self::JO),
            "JP" => Ok(Self::JP),
            "KE" => Ok(Self::KE),
            "KG" => Ok(Self::KG),
            "KH" => Ok(Self::KH),
            "KI" => Ok(Self::KI),
            "KM" => Ok(Self::KM),
            "KN" => Ok(Self::KN),
            "KP" => Ok(Self::KP),
            "KR" => Ok(Self::KR),
            "KW" => Ok(Self::KW),
            "KY" => Ok(Self::KY),
            "KZ" => Ok(Self::KZ),
            "LA" => Ok(Self::LA),
            "LB" => Ok(Self::LB),
            "LC" => Ok(Self::LC),
            "LI" => Ok(Self::LI),
            "LK" => Ok(Self::LK),
            "LR" => Ok(Self::LR),
            "LS" => Ok(Self::LS),
            "LT" => Ok(Self::LT),
            "LU" => Ok(Self::LU),
            "LV" => Ok(Self::LV),
            "LY" => Ok(Self::LY),
            "MA" => Ok(Self::MA),
            "MC" => Ok(Self::MC),
            "MD" => Ok(Self::MD),
            "ME" => Ok(Self::ME),
            "MF" => Ok(Self::MF),
            "MG" => Ok(Self::MG),
            "MH" => Ok(Self::MH),
            "MK" => Ok(Self::MK),
            "ML" => Ok(Self::ML),
            "MM" => Ok(Self::MM),
            "MN" => Ok(Self::MN),
            "MO" => Ok(Self::MO),
            "MP" => Ok(Self::MP),
            "MQ" => Ok(Self::MQ),
            "MR" => Ok(Self::MR),
            "MS" => Ok(Self::MS),
            "MT" => Ok(Self::MT),
            "MU" => Ok(Self::MU),
            "MV" => Ok(Self::MV),
            "MW" => Ok(Self::MW),
            "MX" => Ok(Self::MX),
            "MY" => Ok(Self::MY),
            "MZ" => Ok(Self::MZ),
            "NA" => Ok(Self::NA),
            "NC" => Ok(Self::NC),
            "NE" => Ok(Self::NE),
            "NF" => Ok(Self::NF),
            "NG" => Ok(Self::NG),
            "NI" => Ok(Self::NI),
            "NL" => Ok(Self::NL),
            "NO" => Ok(Self::NO),
            "NP" => Ok(Self::NP),
            "NR" => Ok(Self::NR),
            "NU" => Ok(Self::NU),
            "NZ" => Ok(Self::NZ),
            "OM" => Ok(Self::OM),
            "PA" => Ok(Self::PA),
            "PE" => Ok(Self::PE),
            "PF" => Ok(Self::PF),
            "PG" => Ok(Self::PG),
            "PH" => Ok(Self::PH),
            "PK" => Ok(Self::PK),
            "PL" => Ok(Self::PL),
            "PM" => Ok(Self::PM),
            "PN" => Ok(Self::PN),
            "PR" => Ok(Self::PR),
            "PS" => Ok(Self::PS),
            "PT" => Ok(Self::PT),
            "PW" => Ok(Self::PW),
            "PY" => Ok(Self::PY),
            "QA" => Ok(Self::QA),
            "RE" => Ok(Self::RE),
            "RO" => Ok(Self::RO),
            "RS" => Ok(Self::RS),
            "RU" => Ok(Self::RU),
            "RW" => Ok(Self::RW),
            "SA" => Ok(Self::SA),
            "SB" => Ok(Self::SB),
            "SC" => Ok(Self::SC),
            "SD" => Ok(Self::SD),
            "SE" => Ok(Self::SE),
            "SG" => Ok(Self::SG),
            "SH" => Ok(Self::SH),
            "SI" => Ok(Self::SI),
            "SJ" => Ok(Self::SJ),
            "SK" => Ok(Self::SK),
            "SL" => Ok(Self::SL),
            "SM" => Ok(Self::SM),
            "SN" => Ok(Self::SN),
            "SO" => Ok(Self::SO),
            "SR" => Ok(Self::SR),
            "SS" => Ok(Self::SS),
            "ST" => Ok(Self::ST),
            "SV" => Ok(Self::SV),
            "SX" => Ok(Self::SX),
            "SY" => Ok(Self::SY),
            "SZ" => Ok(Self::SZ),
            "TC" => Ok(Self::TC),
            "TD" => Ok(Self::TD),
            "TF" => Ok(Self::TF),
            "TG" => Ok(Self::TG),
            "TH" => Ok(Self::TH),
            "TJ" => Ok(Self::TJ),
            "TK" => Ok(Self::TK),
            "TL" => Ok(Self::TL),
            "TM" => Ok(Self::TM),
            "TN" => Ok(Self::TN),
            "TO" => Ok(Self::TO),
            "TR" => Ok(Self::TR),
            "TT" => Ok(Self::TT),
            "TV" => Ok(Self::TV),
            "TW" => Ok(Self::TW),
            "TZ" => Ok(Self::TZ),
            "UA" => Ok(Self::UA),
            "UG" => Ok(Self::UG),
            "UM" => Ok(Self::UM),
            "US" => Ok(Self::US),
            "UY" => Ok(Self::UY),
            "UZ" => Ok(Self::UZ),
            "VA" => Ok(Self::VA),
            "VC" => Ok(Self::VC),
            "VE" => Ok(Self::VE),
            "VG" => Ok(Self::VG),
            "VI" => Ok(Self::VI),
            "VN" => Ok(Self::VN),
            "VU" => Ok(Self::VU),
            "WF" => Ok(Self::WF),
            "WS" => Ok(Self::WS),
            "YE" => Ok(Self::YE),
            "YT" => Ok(Self::YT),
            "ZA" => Ok(Self::ZA),
            "ZM" => Ok(Self::ZM),
            "ZW" => Ok(Self::ZW),
            cc => Err(Error::Unrecognized(cc.to_string())),
        }
    }
}

impl FromJson for Alpha2 {
    fn from_json(v: &Json) -> Result<Self, FromJsonError> {
        let value = String::from_json(v)?;
        value
            .parse()
            .map_err(Into::into)
            .map_err(FromJsonError::Parsing)
    }
}
