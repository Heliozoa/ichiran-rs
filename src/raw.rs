//! Contains the "raw" types that directly correspond to the output of `ichiran-cli`.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(transparent)]
pub struct FullSplitInfo(pub Vec<Segment>);

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
pub enum Segment {
    /// Japanese
    Words(Vec<WordSegment>),
    /// Punctuation, etc.
    Other(String),
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct WordSegment(
    /// Words
    pub Vec<Word>,
    /// Unknown
    pub i32,
);

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Word(
    /// Romanized
    pub String,
    /// One or more alternative WordInfo
    pub Alternatives,
    /// Unknown, seems to always be empty
    pub Vec<serde_json::Value>,
);

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Alternatives {
    WordInfo(Alternative),
    Alternatives { alternative: Vec<Alternative> },
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Alternative {
    WordInfo(WordInfo),
    CompoundWordInfo(CompoundWordInfo),
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct WordInfo {
    pub reading: String,
    pub text: String,
    pub kana: String,
    pub score: i32,
    pub counter: Option<Counter>,
    pub seq: Option<i32>,
    #[serde(default)]
    pub gloss: Vec<Gloss>,
    pub suffix: Option<String>,
    #[serde(default)]
    pub conj: Vec<Conj>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CompoundWordInfo {
    pub reading: String,
    pub text: String,
    pub kana: String,
    pub score: i32,
    pub compound: Vec<String>,
    pub components: Vec<WordInfo>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Counter {
    pub value: String,
    pub ordinal: Ordinal,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Ordinal {
    Bool(bool),
    Vec(Vec<serde_json::Value>),
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Gloss {
    pub pos: String,
    pub gloss: String,
    pub info: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Conj {
    pub prop: Vec<ConjProp>,
    #[serde(default)]
    pub via: Vec<Via>,
    pub reading: Option<String>,
    #[serde(default)]
    pub gloss: Vec<Gloss>,
    pub readok: Readok,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ConjProp {
    pub pos: String,
    #[serde(rename = "type")]
    pub prop_type: PropType,
    #[serde(default)]
    pub fml: bool,
    #[serde(default)]
    pub neg: bool,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum PropType {
    String(String),
    Vec(Vec<serde_json::Value>),
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Via {
    pub prop: Vec<ConjProp>,
    pub reading: Option<String>,
    #[serde(default)]
    pub gloss: Vec<Gloss>,
    pub readok: Readok,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Readok {
    Bool(bool),
    Vec(Vec<serde_json::Value>),
}
