#[derive(PartialEq, Eq, PartialOrd, Ord, serde::Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum TagType {
    #[serde(rename = "warning")]
    Warning,

    #[serde(rename = "pairing")]
    Pairing,

    #[serde(rename = "character")]
    Character,

    #[serde(rename = "general")]
    General,
}