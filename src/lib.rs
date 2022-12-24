///! Bindings for ichiran-cli.
use serde::Deserialize;
use std::string::FromUtf8Error;
pub use std::{error::Error, path::PathBuf, process::Command};
use thiserror::Error;

/// Crate error type.
#[derive(Debug, Error)]
pub enum IchiranError {
    #[error("Error while trying to run ichiran-cli")]
    CommandError(#[source] std::io::Error),
    #[error("ichiran-cli output invalid utf-8")]
    InvalidUtf8(#[from] FromUtf8Error),
    #[error("Unexpected output from ichiran-cli")]
    UnexpectedOutput(String),
    #[error("ichiran-cli returned a non-zero exit code")]
    IchiranError { stdout: String, stderr: String },
    #[error("Error while deserializing ichiran-cli output")]
    Deserialization(#[from] serde_path_to_error::Error<serde_json::Error>),
}

/// Wrapper for ichiran-cli.
#[derive(Debug)]
pub struct IchiranCli {
    cli_path: PathBuf,
}

impl IchiranCli {
    /// Takes a path to the `ichiran-cli` binary.
    pub fn new(cli_path: PathBuf) -> Self {
        Self { cli_path }
    }

    pub fn full_split_info(&self, input: &str) -> Result<FullSplitInfo, IchiranError> {
        let (stdout, _stderr) = self.run(&["-f", input])?;
        println!("{stdout}");
        let jd = &mut serde_json::Deserializer::from_str(&stdout);
        let json = serde_path_to_error::deserialize(jd)?;
        Ok(json)
    }

    pub fn romanize_with_info(&self, input: &str) -> Result<RomanizedWithInfo, IchiranError> {
        let (stdout, _stderr) = self.run(&["-i", input])?;
        let mut lines = stdout.lines();
        let romanized = lines
            .next()
            .ok_or_else(|| IchiranError::UnexpectedOutput(stdout.clone()))?
            .to_string();
        let mut entries = vec![];
        let mut word = None;
        let mut alternatives = vec![];
        while let Some(line) = lines.next() {
            if line.is_empty() {
                if let Some(w) = word {
                    entries.push(RomanizedWithInfoEntry {
                        word: w,
                        alternatives,
                    });
                    word = None;
                    alternatives = vec![];
                }
            } else {
                if word.is_some() {
                    alternatives.push(line.to_string());
                } else {
                    word = Some(line.to_string());
                }
            }
        }
        Ok(RomanizedWithInfo { romanized, entries })
    }

    pub fn romanize(&self, input: &str) -> Result<String, IchiranError> {
        let (mut stdout, _stderr) = self.run(&[input])?;
        // truncate to cut off the newline
        stdout.truncate(stdout.len() - 1);
        Ok(stdout)
    }

    fn run(&self, args: &[&str]) -> Result<(String, String), IchiranError> {
        let out = Command::new(&self.cli_path)
            .args(args)
            .output()
            .map_err(IchiranError::CommandError)?;
        let stdout = String::from_utf8(out.stdout)?;
        let stderr = String::from_utf8(out.stderr)?;
        if out.status.success() {
            Ok((stdout, stderr))
        } else {
            Err(IchiranError::IchiranError { stdout, stderr })
        }
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(transparent)]
pub struct FullSplitInfo(pub Vec<InputSegment>);

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
pub enum InputSegment {
    Processed(Vec<FullSplitInfoEntry>),
    Plain(String),
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct FullSplitInfoEntry(pub Vec<FullSplitInfoWord>, pub i32);

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct FullSplitInfoWord(
    pub String,
    pub WordOrAlternatives,
    pub Vec<serde_json::Value>,
);

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum WordOrAlternatives {
    Word(Word),
    Alternatives { alternative: Vec<Word> },
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Word {
    pub reading: String,
    pub text: String,
    pub kana: String,
    pub score: i32,
    pub counter: Option<Counter>,
    #[serde(default)]
    pub compound: Vec<String>,
    #[serde(default)]
    pub components: Vec<Word>,
    pub seq: Option<i32>,
    #[serde(default)]
    pub gloss: Vec<Gloss>,
    pub suffix: Option<String>,
    #[serde(default)]
    pub conj: Vec<Conj>,
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
    pub via: Vec<Conj>,
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

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ConjProp {
    pub pos: String,
    #[serde(rename = "type")]
    pub prop_type: String,
    #[serde(default)]
    pub fml: bool,
    #[serde(default)]
    pub neg: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RomanizedWithInfo {
    pub romanized: String,
    pub entries: Vec<RomanizedWithInfoEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RomanizedWithInfoEntry {
    pub word: String,
    pub alternatives: Vec<String>,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    fn ichiran() -> IchiranCli {
        IchiranCli::new(PathBuf::from("./ichiran-cli"))
    }

    #[test]
    fn romanizes() {
        let out = ichiran().romanize("").unwrap();
        assert!(out.is_empty());

        let out = ichiran().romanize("test").unwrap();
        assert_eq!(out, "test");

        let out = ichiran().romanize("一覧は最高だぞ").unwrap();
        assert_eq!(out, "ichiran wa saikō da zo");
    }

    #[test]
    fn romanizes_with_info() {
        let out = ichiran().romanize_with_info("一覧は最高だぞ").unwrap();
        assert_eq!(out.romanized, "ichiran wa saikō da zo");
        assert_eq!(
            out.entries[0],
            RomanizedWithInfoEntry {
                word: "* ichiran  一覧 【いちらん】".to_string(),
                alternatives: vec![
                    "1. [n,vs] look; glance; sight; inspection".to_string(),
                    "2. [n] summary; list; table; catalog; catalogue".to_string()
                ]
            }
        );
        assert_eq!(
            out.entries[1],
            RomanizedWithInfoEntry {
                word: "* wa  は".to_string(),
                alternatives: vec![
                    "1. [prt] 《pronounced わ in modern Japanese》 indicates sentence topic"
                        .to_string(),
                    "2. [prt] indicates contrast with another option (stated or unstated)"
                        .to_string(),
                    "3. [prt] adds emphasis".to_string()
                ]
            }
        );
        assert_eq!(
            out.entries[2],
            RomanizedWithInfoEntry {
                word: "* saikō  最高 【さいこう】".to_string(),
                alternatives: vec![
                    "1. [adj-no,adj-na,n] best; supreme; wonderful; finest".to_string(),
                    "2. [n,adj-na,adj-no] highest; maximum; most; uppermost; supreme".to_string()
                ]
            }
        );
        assert_eq!(
            out.entries[3],
            RomanizedWithInfoEntry {
                word: "* da  だ".to_string(),
                alternatives: vec![
                    "1. [cop,cop-da] 《plain copula》 be; is"
                        .to_string(),
                    "2. [aux-v] 《た after certain verb forms; indicates past or completed action》 did; (have) done"
                        .to_string(),
                    "3. [aux-v] 《indicates light imperative》 please; do".to_string()
                ]
            }
        );
        assert_eq!(out.entries.len(), 4);
    }

    #[test]
    fn gets_full_split_info() {
        let ichiran = ichiran();
        let _json = ichiran.full_split_info("一覧は最高だぞ").unwrap();
    }
}
