// Machine-built from the vendored cldr-json 48.2.1 dataset. Do not edit by hand.
//
// CLDR_INPUT_SHA256 hashes every input file, sorted by filename:
// per file, the filename, a newline, the raw bytes, a newline.

use super::Entry;
use crate::{Grouping, Locale};

/// cldr-json tag the table was generated from.
pub const CLDR_VERSION: &str = "48.2.1";

/// Pinned source of the CLDR data.
pub const CLDR_SOURCE_URL: &str = "https://github.com/unicode-org/cldr-json/tree/48.2.1";

/// sha256 over the vendored input JSON set the generator read.
pub const CLDR_INPUT_SHA256: &str =
    "4146a1a765381c18449b1440e4127660e431731659a274bdbd702523d73cd2a0";

const AM: Entry = Entry {
    grouping: Grouping::Standard,
    separator: ",",
    minus_sign: "-",
    decimal: ".",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "\u{1260}\u{1241}\u{1325}\u{122d}\u{a0}\u{120a}\u{1308}\u{1208}\u{133d}\u{a0}\u{12e8}\u{121b}\u{12ed}\u{127d}\u{120d}",
};

const AR: Entry = Entry {
    grouping: Grouping::Standard,
    separator: ",",
    minus_sign: "\u{200e}-",
    decimal: ".",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "\u{644}\u{64a}\u{633}\u{a0}\u{631}\u{642}\u{645}\u{64b}\u{627}",
};

const AR_EG: Entry = Entry {
    grouping: Grouping::Standard,
    separator: "\u{66c}",
    minus_sign: "\u{61c}-",
    decimal: "\u{66b}",
    minimum_grouping_digits: 1,
    numbering_system: "arab",
    digits: [
        "\u{660}", "\u{661}", "\u{662}", "\u{663}", "\u{664}", "\u{665}", "\u{666}", "\u{667}",
        "\u{668}", "\u{669}",
    ],
    infinity: "\u{221e}",
    nan: "\u{644}\u{64a}\u{633}\u{a0}\u{631}\u{642}\u{645}\u{64b}\u{627}",
};

const AR_SA: Entry = Entry {
    grouping: Grouping::Standard,
    separator: "\u{66c}",
    minus_sign: "\u{61c}-",
    decimal: "\u{66b}",
    minimum_grouping_digits: 1,
    numbering_system: "arab",
    digits: [
        "\u{660}", "\u{661}", "\u{662}", "\u{663}", "\u{664}", "\u{665}", "\u{666}", "\u{667}",
        "\u{668}", "\u{669}",
    ],
    infinity: "\u{221e}",
    nan: "\u{644}\u{64a}\u{633}\u{a0}\u{631}\u{642}\u{645}\u{64b}\u{627}",
};

const BN: Entry = Entry {
    grouping: Grouping::Indian,
    separator: ",",
    minus_sign: "-",
    decimal: ".",
    minimum_grouping_digits: 1,
    numbering_system: "beng",
    digits: [
        "\u{9e6}", "\u{9e7}", "\u{9e8}", "\u{9e9}", "\u{9ea}", "\u{9eb}", "\u{9ec}", "\u{9ed}",
        "\u{9ee}", "\u{9ef}",
    ],
    infinity: "\u{221e}",
    nan: "NaN",
};

const BN_IN: Entry = Entry {
    grouping: Grouping::Indian,
    separator: ",",
    minus_sign: "-",
    decimal: ".",
    minimum_grouping_digits: 1,
    numbering_system: "beng",
    digits: [
        "\u{9e6}", "\u{9e7}", "\u{9e8}", "\u{9e9}", "\u{9ea}", "\u{9eb}", "\u{9ec}", "\u{9ed}",
        "\u{9ee}", "\u{9ef}",
    ],
    infinity: "\u{221e}",
    nan: "NaN",
};

const CS: Entry = Entry {
    grouping: Grouping::Standard,
    separator: "\u{a0}",
    minus_sign: "-",
    decimal: ",",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const DA: Entry = Entry {
    grouping: Grouping::Standard,
    separator: ".",
    minus_sign: "-",
    decimal: ",",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const DE: Entry = Entry {
    grouping: Grouping::Standard,
    separator: ".",
    minus_sign: "-",
    decimal: ",",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const DE_AT: Entry = Entry {
    grouping: Grouping::Standard,
    separator: "\u{a0}",
    minus_sign: "-",
    decimal: ",",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const DE_CH: Entry = Entry {
    grouping: Grouping::Standard,
    separator: "'",
    minus_sign: "-",
    decimal: ".",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const EL: Entry = Entry {
    grouping: Grouping::Standard,
    separator: ".",
    minus_sign: "-",
    decimal: ",",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const EN: Entry = Entry {
    grouping: Grouping::Standard,
    separator: ",",
    minus_sign: "-",
    decimal: ".",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const EN_CH: Entry = Entry {
    grouping: Grouping::Standard,
    separator: "'",
    minus_sign: "-",
    decimal: ".",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const EN_GB: Entry = Entry {
    grouping: Grouping::Standard,
    separator: ",",
    minus_sign: "-",
    decimal: ".",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const EN_IN: Entry = Entry {
    grouping: Grouping::Indian,
    separator: ",",
    minus_sign: "-",
    decimal: ".",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const EN_ZA: Entry = Entry {
    grouping: Grouping::Standard,
    separator: "\u{a0}",
    minus_sign: "-",
    decimal: ",",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const ES: Entry = Entry {
    grouping: Grouping::Standard,
    separator: ".",
    minus_sign: "-",
    decimal: ",",
    minimum_grouping_digits: 2,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const ES_AR: Entry = Entry {
    grouping: Grouping::Standard,
    separator: ".",
    minus_sign: "-",
    decimal: ",",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const ES_MX: Entry = Entry {
    grouping: Grouping::Standard,
    separator: ",",
    minus_sign: "-",
    decimal: ".",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const FA: Entry = Entry {
    grouping: Grouping::Standard,
    separator: "\u{66c}",
    minus_sign: "\u{200e}\u{2212}",
    decimal: "\u{66b}",
    minimum_grouping_digits: 1,
    numbering_system: "arabext",
    digits: [
        "\u{6f0}", "\u{6f1}", "\u{6f2}", "\u{6f3}", "\u{6f4}", "\u{6f5}", "\u{6f6}", "\u{6f7}",
        "\u{6f8}", "\u{6f9}",
    ],
    infinity: "\u{221e}",
    nan: "\u{646}\u{627}\u{639}\u{62f}\u{62f}",
};

const FI: Entry = Entry {
    grouping: Grouping::Standard,
    separator: "\u{a0}",
    minus_sign: "\u{2212}",
    decimal: ",",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "ep\u{e4}luku",
};

const FR: Entry = Entry {
    grouping: Grouping::Standard,
    separator: "\u{202f}",
    minus_sign: "-",
    decimal: ",",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const FR_CA: Entry = Entry {
    grouping: Grouping::Standard,
    separator: "\u{a0}",
    minus_sign: "-",
    decimal: ",",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const FR_CH: Entry = Entry {
    grouping: Grouping::Standard,
    separator: "'",
    minus_sign: "-",
    decimal: ",",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const GU: Entry = Entry {
    grouping: Grouping::Indian,
    separator: ",",
    minus_sign: "-",
    decimal: ".",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const HE: Entry = Entry {
    grouping: Grouping::Standard,
    separator: ",",
    minus_sign: "\u{200e}-",
    decimal: ".",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const HI: Entry = Entry {
    grouping: Grouping::Indian,
    separator: ",",
    minus_sign: "-",
    decimal: ".",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const HI_LATN: Entry = Entry {
    grouping: Grouping::Indian,
    separator: ",",
    minus_sign: "-",
    decimal: ".",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const HU: Entry = Entry {
    grouping: Grouping::Standard,
    separator: "\u{a0}",
    minus_sign: "-",
    decimal: ",",
    minimum_grouping_digits: 2,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const ID: Entry = Entry {
    grouping: Grouping::Standard,
    separator: ".",
    minus_sign: "-",
    decimal: ",",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const IT: Entry = Entry {
    grouping: Grouping::Standard,
    separator: ".",
    minus_sign: "-",
    decimal: ",",
    minimum_grouping_digits: 2,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const JA: Entry = Entry {
    grouping: Grouping::Standard,
    separator: ",",
    minus_sign: "-",
    decimal: ".",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const KO: Entry = Entry {
    grouping: Grouping::Standard,
    separator: ",",
    minus_sign: "-",
    decimal: ".",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const MR: Entry = Entry {
    grouping: Grouping::Indian,
    separator: ",",
    minus_sign: "-",
    decimal: ".",
    minimum_grouping_digits: 1,
    numbering_system: "deva",
    digits: [
        "\u{966}", "\u{967}", "\u{968}", "\u{969}", "\u{96a}", "\u{96b}", "\u{96c}", "\u{96d}",
        "\u{96e}", "\u{96f}",
    ],
    infinity: "\u{221e}",
    nan: "NaN",
};

const NB: Entry = Entry {
    grouping: Grouping::Standard,
    separator: "\u{a0}",
    minus_sign: "\u{2212}",
    decimal: ",",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const NL: Entry = Entry {
    grouping: Grouping::Standard,
    separator: ".",
    minus_sign: "-",
    decimal: ",",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const PL: Entry = Entry {
    grouping: Grouping::Standard,
    separator: "\u{a0}",
    minus_sign: "-",
    decimal: ",",
    minimum_grouping_digits: 2,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const PT: Entry = Entry {
    grouping: Grouping::Standard,
    separator: ".",
    minus_sign: "-",
    decimal: ",",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const PT_PT: Entry = Entry {
    grouping: Grouping::Standard,
    separator: "\u{a0}",
    minus_sign: "-",
    decimal: ",",
    minimum_grouping_digits: 2,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const RO: Entry = Entry {
    grouping: Grouping::Standard,
    separator: ".",
    minus_sign: "-",
    decimal: ",",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const RU: Entry = Entry {
    grouping: Grouping::Standard,
    separator: "\u{a0}",
    minus_sign: "-",
    decimal: ",",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "\u{43d}\u{435}\u{a0}\u{447}\u{438}\u{441}\u{43b}\u{43e}",
};

const SV: Entry = Entry {
    grouping: Grouping::Standard,
    separator: "\u{a0}",
    minus_sign: "\u{2212}",
    decimal: ",",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const SW: Entry = Entry {
    grouping: Grouping::Standard,
    separator: ",",
    minus_sign: "-",
    decimal: ".",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const TA: Entry = Entry {
    grouping: Grouping::Indian,
    separator: ",",
    minus_sign: "-",
    decimal: ".",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const TH: Entry = Entry {
    grouping: Grouping::Standard,
    separator: ",",
    minus_sign: "-",
    decimal: ".",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const TR: Entry = Entry {
    grouping: Grouping::Standard,
    separator: ".",
    minus_sign: "-",
    decimal: ",",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const UK: Entry = Entry {
    grouping: Grouping::Standard,
    separator: "\u{a0}",
    minus_sign: "-",
    decimal: ",",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const UND: Entry = Entry {
    grouping: Grouping::Standard,
    separator: ",",
    minus_sign: "-",
    decimal: ".",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const UR: Entry = Entry {
    grouping: Grouping::Standard,
    separator: ",",
    minus_sign: "\u{200e}-",
    decimal: ".",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const VI: Entry = Entry {
    grouping: Grouping::Standard,
    separator: ".",
    minus_sign: "-",
    decimal: ",",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const ZH: Entry = Entry {
    grouping: Grouping::Standard,
    separator: ",",
    minus_sign: "-",
    decimal: ".",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "NaN",
};

const ZH_HANT: Entry = Entry {
    grouping: Grouping::Standard,
    separator: ",",
    minus_sign: "-",
    decimal: ".",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "\u{975e}\u{6578}\u{503c}",
};

const ZH_HANT_HK: Entry = Entry {
    grouping: Grouping::Standard,
    separator: ",",
    minus_sign: "-",
    decimal: ".",
    minimum_grouping_digits: 1,
    numbering_system: "latn",
    digits: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    infinity: "\u{221e}",
    nan: "\u{975e}\u{6578}\u{503c}",
};

pub(crate) const fn entry(locale: Locale) -> &'static Entry {
    match locale {
        Locale::am => &AM,
        Locale::ar => &AR,
        Locale::ar_EG => &AR_EG,
        Locale::ar_SA => &AR_SA,
        Locale::bn => &BN,
        Locale::bn_IN => &BN_IN,
        Locale::cs => &CS,
        Locale::da => &DA,
        Locale::de => &DE,
        Locale::de_AT => &DE_AT,
        Locale::de_CH => &DE_CH,
        Locale::el => &EL,
        Locale::en => &EN,
        Locale::en_CH => &EN_CH,
        Locale::en_GB => &EN_GB,
        Locale::en_IN => &EN_IN,
        Locale::en_ZA => &EN_ZA,
        Locale::es => &ES,
        Locale::es_AR => &ES_AR,
        Locale::es_MX => &ES_MX,
        Locale::fa => &FA,
        Locale::fi => &FI,
        Locale::fr => &FR,
        Locale::fr_CA => &FR_CA,
        Locale::fr_CH => &FR_CH,
        Locale::gu => &GU,
        Locale::he => &HE,
        Locale::hi => &HI,
        Locale::hi_Latn => &HI_LATN,
        Locale::hu => &HU,
        Locale::id => &ID,
        Locale::it => &IT,
        Locale::ja => &JA,
        Locale::ko => &KO,
        Locale::mr => &MR,
        Locale::nb => &NB,
        Locale::nl => &NL,
        Locale::pl => &PL,
        Locale::pt => &PT,
        Locale::pt_PT => &PT_PT,
        Locale::ro => &RO,
        Locale::ru => &RU,
        Locale::sv => &SV,
        Locale::sw => &SW,
        Locale::ta => &TA,
        Locale::th => &TH,
        Locale::tr => &TR,
        Locale::uk => &UK,
        Locale::und => &UND,
        Locale::ur => &UR,
        Locale::vi => &VI,
        Locale::zh => &ZH,
        Locale::zh_Hant => &ZH_HANT,
        Locale::zh_Hant_HK => &ZH_HANT_HK,
        Locale::en_US => &EN,
        Locale::pt_BR => &PT,
        Locale::de_DE => &DE,
        Locale::fr_FR => &FR,
        Locale::es_ES => &ES,
        Locale::zh_Hans_CN => &ZH,
        Locale::zh_Hant_TW => &ZH_HANT,
        Locale::hi_IN => &HI,
        Locale::ja_JP => &JA,
    }
}

pub(crate) const fn name_of(locale: Locale) -> &'static str {
    match locale {
        Locale::am => "am",
        Locale::ar => "ar",
        Locale::ar_EG => "ar-EG",
        Locale::ar_SA => "ar-SA",
        Locale::bn => "bn",
        Locale::bn_IN => "bn-IN",
        Locale::cs => "cs",
        Locale::da => "da",
        Locale::de => "de",
        Locale::de_AT => "de-AT",
        Locale::de_CH => "de-CH",
        Locale::el => "el",
        Locale::en => "en",
        Locale::en_CH => "en-CH",
        Locale::en_GB => "en-GB",
        Locale::en_IN => "en-IN",
        Locale::en_ZA => "en-ZA",
        Locale::es => "es",
        Locale::es_AR => "es-AR",
        Locale::es_MX => "es-MX",
        Locale::fa => "fa",
        Locale::fi => "fi",
        Locale::fr => "fr",
        Locale::fr_CA => "fr-CA",
        Locale::fr_CH => "fr-CH",
        Locale::gu => "gu",
        Locale::he => "he",
        Locale::hi => "hi",
        Locale::hi_Latn => "hi-Latn",
        Locale::hu => "hu",
        Locale::id => "id",
        Locale::it => "it",
        Locale::ja => "ja",
        Locale::ko => "ko",
        Locale::mr => "mr",
        Locale::nb => "nb",
        Locale::nl => "nl",
        Locale::pl => "pl",
        Locale::pt => "pt",
        Locale::pt_PT => "pt-PT",
        Locale::ro => "ro",
        Locale::ru => "ru",
        Locale::sv => "sv",
        Locale::sw => "sw",
        Locale::ta => "ta",
        Locale::th => "th",
        Locale::tr => "tr",
        Locale::uk => "uk",
        Locale::und => "und",
        Locale::ur => "ur",
        Locale::vi => "vi",
        Locale::zh => "zh",
        Locale::zh_Hant => "zh-Hant",
        Locale::zh_Hant_HK => "zh-Hant-HK",
        Locale::en_US => "en-US",
        Locale::pt_BR => "pt-BR",
        Locale::de_DE => "de-DE",
        Locale::fr_FR => "fr-FR",
        Locale::es_ES => "es-ES",
        Locale::zh_Hans_CN => "zh-Hans-CN",
        Locale::zh_Hant_TW => "zh-Hant-TW",
        Locale::hi_IN => "hi-IN",
        Locale::ja_JP => "ja-JP",
    }
}

pub(crate) const NAMES: [&str; 63] = [
    "am",
    "ar",
    "ar-EG",
    "ar-SA",
    "bn",
    "bn-IN",
    "cs",
    "da",
    "de",
    "de-AT",
    "de-CH",
    "el",
    "en",
    "en-CH",
    "en-GB",
    "en-IN",
    "en-ZA",
    "es",
    "es-AR",
    "es-MX",
    "fa",
    "fi",
    "fr",
    "fr-CA",
    "fr-CH",
    "gu",
    "he",
    "hi",
    "hi-Latn",
    "hu",
    "id",
    "it",
    "ja",
    "ko",
    "mr",
    "nb",
    "nl",
    "pl",
    "pt",
    "pt-PT",
    "ro",
    "ru",
    "sv",
    "sw",
    "ta",
    "th",
    "tr",
    "uk",
    "und",
    "ur",
    "vi",
    "zh",
    "zh-Hant",
    "zh-Hant-HK",
    "en-US",
    "pt-BR",
    "de-DE",
    "fr-FR",
    "es-ES",
    "zh-Hans-CN",
    "zh-Hant-TW",
    "hi-IN",
    "ja-JP",
];

pub(crate) const VARIANTS: [Locale; 63] = [
    Locale::am,
    Locale::ar,
    Locale::ar_EG,
    Locale::ar_SA,
    Locale::bn,
    Locale::bn_IN,
    Locale::cs,
    Locale::da,
    Locale::de,
    Locale::de_AT,
    Locale::de_CH,
    Locale::el,
    Locale::en,
    Locale::en_CH,
    Locale::en_GB,
    Locale::en_IN,
    Locale::en_ZA,
    Locale::es,
    Locale::es_AR,
    Locale::es_MX,
    Locale::fa,
    Locale::fi,
    Locale::fr,
    Locale::fr_CA,
    Locale::fr_CH,
    Locale::gu,
    Locale::he,
    Locale::hi,
    Locale::hi_Latn,
    Locale::hu,
    Locale::id,
    Locale::it,
    Locale::ja,
    Locale::ko,
    Locale::mr,
    Locale::nb,
    Locale::nl,
    Locale::pl,
    Locale::pt,
    Locale::pt_PT,
    Locale::ro,
    Locale::ru,
    Locale::sv,
    Locale::sw,
    Locale::ta,
    Locale::th,
    Locale::tr,
    Locale::uk,
    Locale::und,
    Locale::ur,
    Locale::vi,
    Locale::zh,
    Locale::zh_Hant,
    Locale::zh_Hant_HK,
    Locale::en_US,
    Locale::pt_BR,
    Locale::de_DE,
    Locale::fr_FR,
    Locale::es_ES,
    Locale::zh_Hans_CN,
    Locale::zh_Hant_TW,
    Locale::hi_IN,
    Locale::ja_JP,
];
