//! Bindings for ichiran-cli.

pub mod raw;
mod rusty;

pub use self::rusty::*;
use std::{path::PathBuf, process::Command};
use thiserror::Error;

/// Crate error type.
#[derive(Debug, Error)]
pub enum IchiranError {
    #[error("Error while trying to run ichiran-cli")]
    CommandError(#[source] std::io::Error),
    #[error("ichiran-cli output invalid utf-8")]
    InvalidUtf8(#[from] std::string::FromUtf8Error),
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

    /// Calls and parses the output of `ichiran-cli -f`.
    /// The optional limit argument defines the max number of alternative segmentations that are returned for each segment.
    pub fn segment(&self, input: &str, limit: Option<u32>) -> Result<Vec<Segment>, IchiranError> {
        let (stdout, _stderr) = if let Some(limit) = limit {
            self.run(&["-f", "-l", &limit.to_string(), input])?
        } else {
            self.run(&["-f", input])?
        };
        let jd = &mut serde_json::Deserializer::from_str(&stdout);
        let info: raw::FullSplitInfo = serde_path_to_error::deserialize(jd)?;
        Ok(info.into())
    }

    /// Calls and parses the output of `ichiran-cli -i`.
    pub fn romanize_with_info(&self, input: &str) -> Result<RomanizedWithInfo, IchiranError> {
        let (stdout, _stderr) = self.run(&["-i", input])?;
        let mut lines = stdout.lines();
        let mut romanized = lines
            .next()
            .ok_or_else(|| IchiranError::UnexpectedOutput(stdout.clone()))?
            .to_string();
        let trimmed = romanized.trim_end().len();
        romanized.truncate(trimmed);
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

    /// Calls and parses the output of `ichiran-cli` without any flags.
    pub fn romanize(&self, input: &str) -> Result<String, IchiranError> {
        let (mut stdout, _stderr) = self.run(&[input])?;
        // truncate to cut off the newline
        let trimmed = stdout.trim_end().len();
        stdout.truncate(trimmed);
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

    fn ichiran() -> IchiranCli {
        IchiranCli::new(PathBuf::from("./data/ichiran-cli"))
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
                    "1. [n,vs,vt] look; glance; sight; having a look at; looking over; glancing through; running one's eyes over".to_string(),
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
                alternatives: vec!["1. [aux-v,cop-da,cop] 《plain copula》 be; is".to_string()]
            }
        );
        assert_eq!(out.entries.len(), 4);
    }

    #[test]
    fn gets_full_split_info() {
        let ichiran = ichiran();
        let _segmented = ichiran.segment("一覧は最高だぞ", None).unwrap();
    }

    #[test]
    fn uses_limit() {
        let ichiran = ichiran();
        let segmented = ichiran.segment("一人目", None).unwrap();
        let Segment::Segmentations(segmentations) = &segmented[0] else {
            panic!();
        };
        assert_eq!(segmentations.len(), 1);
        let segmented = ichiran.segment("一人目", Some(2)).unwrap();
        let Segment::Segmentations(segmentations) = &segmented[0] else {
            panic!();
        };
        assert_eq!(segmentations.len(), 2);
    }

    #[test]
    #[ignore = "takes a very long time, requires a book to test with from aozora bunko"]
    fn book() {
        let ichiran = ichiran();
        let file = std::fs::read_to_string("./data/book").unwrap();
        for (idx, line) in file.lines().enumerate() {
            println!("{idx} {line}");
            ichiran.segment(line, None).unwrap();
        }
    }
}
