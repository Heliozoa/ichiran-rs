//! Contains more Rusty equivalents of the raw types.

use crate::raw;

/// A single segment which may consist of one or more words,
/// or punctuation or other non-word text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Segment {
    /// A list of alternate segmentations for a sequence of words.
    Segmentations(Vec<Segmentation>),
    /// Punctuation or other non-word text.
    Other(String),
}

impl From<raw::FullSplitInfo> for Vec<Segment> {
    fn from(value: raw::FullSplitInfo) -> Self {
        value
            .0
            .into_iter()
            .map(|s| match s {
                raw::Segment::Segmentations(words) => {
                    Segment::Segmentations(words.into_iter().map(Into::into).collect())
                }
                raw::Segment::Other(other) => Segment::Other(other),
            })
            .collect()
    }
}

/// A possible segmentation for a sequence of one or more words.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Segmentation {
    pub words: Vec<Word>,
    /// A higher score indicates that this segmentation is more likely to be correct.
    pub score: i32,
}

impl From<raw::Segmentation> for Segmentation {
    fn from(value: raw::Segmentation) -> Self {
        Self {
            words: value.0.into_iter().map(Into::into).collect(),
            score: value.1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Word {
    pub romanized: String,
    /// Possible interpretations for this word.
    pub alternatives: Vec<Alternative>,
}

impl From<raw::Word> for Word {
    fn from(value: raw::Word) -> Self {
        Self {
            romanized: value.0,
            alternatives: value.1.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Alternative {
    WordInfo(WordInfo),
    CompoundWordInfo(CompoundWordInfo),
}

impl From<raw::Alternatives> for Vec<Alternative> {
    fn from(value: raw::Alternatives) -> Self {
        let mut alternatives = vec![];
        match value {
            raw::Alternatives::WordInfo(info) => alternatives.push(info.into()),
            raw::Alternatives::Alternatives { alternative } => {
                alternatives.extend(alternative.into_iter().map(Into::into))
            }
        };
        alternatives
    }
}

impl From<raw::Alternative> for Alternative {
    fn from(value: raw::Alternative) -> Self {
        match value {
            raw::Alternative::WordInfo(info) => Self::WordInfo(info.into()),
            raw::Alternative::CompoundWordInfo(compound) => Self::CompoundWordInfo(compound.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WordInfo {
    pub reading: String,
    pub text: String,
    pub kana: String,
    pub score: i32,
    pub counter: Option<Counter>,
    pub seq: Option<i32>,
    pub gloss: Vec<Gloss>,
    pub suffix: Option<String>,
    pub conj: Vec<Conj>,
}

impl From<raw::WordInfo> for WordInfo {
    fn from(value: raw::WordInfo) -> Self {
        Self {
            reading: value.reading,
            text: value.text,
            kana: value.kana,
            score: value.score,
            counter: value.counter.map(Into::into),
            seq: value.seq,
            gloss: value.gloss.into_iter().map(Into::into).collect(),
            suffix: value.suffix,
            conj: value.conj.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompoundWordInfo {
    pub reading: String,
    pub text: String,
    pub kana: String,
    pub score: i32,
    pub compound: Vec<String>,
    pub components: Vec<WordInfo>,
}

impl From<raw::CompoundWordInfo> for CompoundWordInfo {
    fn from(value: raw::CompoundWordInfo) -> Self {
        Self {
            reading: value.reading,
            text: value.text,
            kana: value.kana,
            score: value.score,
            compound: value.compound,
            components: value.components.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Counter {
    pub value: String,
    pub ordinal: bool,
}

impl From<raw::Counter> for Counter {
    fn from(value: raw::Counter) -> Self {
        Self {
            value: value.value,
            ordinal: match value.ordinal {
                raw::Ordinal::Bool(b) => b,
                raw::Ordinal::Vec(_) => false,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gloss {
    pub pos: String,
    pub gloss: String,
    pub info: Option<String>,
}

impl From<raw::Gloss> for Gloss {
    fn from(value: raw::Gloss) -> Self {
        Self {
            pos: value.pos,
            gloss: value.gloss,
            info: value.info,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Conj {
    pub prop: Vec<ConjProp>,
    pub via: Vec<Via>,
    pub reading: Option<String>,
    pub gloss: Vec<Gloss>,
    pub readok: bool,
}

impl From<raw::Conj> for Conj {
    fn from(value: raw::Conj) -> Self {
        Self {
            prop: value.prop.into_iter().map(Into::into).collect(),
            via: value.via.into_iter().map(Into::into).collect(),
            reading: value.reading,
            gloss: value.gloss.into_iter().map(Into::into).collect(),
            readok: value.readok.into(),
        }
    }
}

impl From<raw::Readok> for bool {
    fn from(value: raw::Readok) -> Self {
        match value {
            raw::Readok::Bool(b) => b,
            raw::Readok::Vec(_) => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConjProp {
    pub pos: String,
    pub prop_type: Option<String>,
    pub fml: bool,
    pub neg: bool,
}

impl From<raw::ConjProp> for ConjProp {
    fn from(value: raw::ConjProp) -> Self {
        Self {
            pos: value.pos,
            prop_type: value.prop_type.into(),
            fml: value.fml,
            neg: value.neg,
        }
    }
}

impl From<raw::PropType> for Option<String> {
    fn from(value: raw::PropType) -> Self {
        match value {
            raw::PropType::String(s) => Some(s),
            raw::PropType::Vec(_) => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Via {
    pub prop: Vec<ConjProp>,
    pub reading: Option<String>,
    pub gloss: Vec<Gloss>,
    pub readok: bool,
}

impl From<raw::Via> for Via {
    fn from(value: raw::Via) -> Self {
        Self {
            prop: value.prop.into_iter().map(Into::into).collect(),
            reading: value.reading,
            gloss: value.gloss.into_iter().map(Into::into).collect(),
            readok: value.readok.into(),
        }
    }
}
