//! Contains more Rusty equivalents of the raw types.

use crate::raw;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Segment {
    /// Japanese
    Words(Vec<WordSegment>),
    /// Punctuation, etc.
    Other(String),
}

impl From<raw::FullSplitInfo> for Vec<Segment> {
    fn from(value: raw::FullSplitInfo) -> Self {
        value
            .0
            .into_iter()
            .map(|s| match s {
                raw::Segment::Words(words) => {
                    Segment::Words(words.into_iter().map(Into::into).collect())
                }
                raw::Segment::Other(other) => Segment::Other(other),
            })
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WordSegment {
    pub words: Vec<Word>,
    pub unknown: i32,
}

impl From<raw::WordSegment> for WordSegment {
    fn from(value: raw::WordSegment) -> Self {
        Self {
            words: value.0.into_iter().map(Into::into).collect(),
            unknown: value.1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Word {
    pub romanized: String,
    /// One or more alternative WordInfo
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
    pub prop_type: String,
    pub fml: bool,
    pub neg: bool,
}

impl From<raw::ConjProp> for ConjProp {
    fn from(value: raw::ConjProp) -> Self {
        Self {
            pos: value.pos,
            prop_type: value.prop_type,
            fml: value.fml,
            neg: value.neg,
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
