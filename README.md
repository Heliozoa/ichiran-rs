# ichiran-rs

[![Crates.io](https://img.shields.io/crates/v/ichiran)](https://crates.io/crates/ichiran)
[![docs.rs](https://img.shields.io/badge/docs.rs-ichiran-success)](https://docs.rs/ichiran)
[![Crates.io](https://img.shields.io/crates/l/ichiran)](https://choosealicense.com/licenses/mpl-2.0/)
[![GitHub](https://img.shields.io/badge/GitHub-Heliozoa-24292f)](https://github.com/Heliozoa/ichiran)

Rust bindings for [`ichiran-cli`](https://github.com/tshatrov/ichiran).

Ichiran's API is not stable, so this crate may not work with some future version of `ichiran-cli`.

## Setup
Instructions for building `ichiran-cli` can be found at https://readevalprint.tumblr.com/post/639359547843215360/ichiranhome-2021-the-ultimate-guide

## Example
```rs
use ichiran::IchiranCli;
use std::{error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let cli = IchiranCli::new(PathBuf::from("./path-to-cli"));

    let segmentation = cli.segment("いい天気ですね。")?;
    println!("{segmentation:#?}");
    /* outputs
    [
        Words(
            [
                WordSegment {
                    words: [
                        Word {
                            romanized: "iitenki",
                            alternatives: [
                                WordInfo(
                                    WordInfo {
                                        reading: "いい天気 【いいてんき】",
                                        text: "いい天気",
                                        kana: "いいてんき",
                                        score: 315,
                                        counter: None,
                                        seq: Some(
                                            1914340,
                                        ),
                                        gloss: [
                                            Gloss {
                                                pos: "[n,exp]",
                                                gloss: "fine weather; fair weather",
                                                info: None,
                                            },
                                        ],
                                        suffix: None,
                                        conj: [],
                                    },
                                ),
                            ],
                        },
                        Word {
                            romanized: "desu",
                            alternatives: [
                                WordInfo(
                                    WordInfo {
                                        reading: "です",
                                        text: "です",
                                        kana: "です",
                                        score: 64,
                                        counter: None,
                                        seq: Some(
                                            1628500,
                                        ),
                                        gloss: [
                                            Gloss {
                                                pos: "[cop]",
                                                gloss: "be; is",
                                                info: None,
                                            },
                                        ],
                                        suffix: None,
                                        conj: [
                                            Conj {
                                                prop: [
                                                    ConjProp {
                                                        pos: "cop",
                                                        prop_type: None,
                                                        fml: true,
                                                        neg: false,
                                                    },
                                                ],
                                                via: [],
                                                reading: Some(
                                                    "だ",
                                                ),
                                                gloss: [
                                                    Gloss {
                                                        pos: "[cop,cop-da]",
                                                        gloss: "be; is",
                                                        info: Some(
                                                            "plain copula",
                                                        ),
                                                    },
                                                ],
                                                readok: true,
                                            },
                                        ],
                                    },
                                ),
                            ],
                        },
                        Word {
                            romanized: "ne",
                            alternatives: [
                                WordInfo(
                                    WordInfo {
                                        reading: "ね",
                                        text: "ね",
                                        kana: "ね",
                                        score: 16,
                                        counter: None,
                                        seq: Some(
                                            2029080,
                                        ),
                                        gloss: [
                                            Gloss {
                                                pos: "[prt]",
                                                gloss: "right?; isn't it?; doesn't it?; don't you?; don't you think?",
                                                info: Some(
                                                    "at sentence end; used as a request for confirmation or agreement",
                                                ),
                                            },
                                            Gloss {
                                                pos: "[int]",
                                                gloss: "hey; say; listen; look; come on",
                                                info: None,
                                            },
                                            Gloss {
                                                pos: "[prt]",
                                                gloss: "you know; you see; I must say; I should think",
                                                info: Some(
                                                    "at sentence end; used to express one's thoughts or feelings",
                                                ),
                                            },
                                            Gloss {
                                                pos: "[prt]",
                                                gloss: "will you?; please",
                                                info: Some(
                                                    "at sentence end; used to make an informal request",
                                                ),
                                            },
                                            Gloss {
                                                pos: "[prt]",
                                                gloss: "so, ...; well, ...; you see, ...; you understand?",
                                                info: Some(
                                                    "at the end of a non-final clause; used to draw the listener's attention to something",
                                                ),
                                            },
                                            Gloss {
                                                pos: "[prt]",
                                                gloss: "I'm not sure if ...; I have my doubts about whether ...",
                                                info: Some(
                                                    "at sentence end after the question marker か",
                                                ),
                                            },
                                        ],
                                        suffix: None,
                                        conj: [],
                                    },
                                ),
                            ],
                        },
                    ],
                    unknown: 405,
                },
            ],
        ),
        Other(
            ". ",
        ),
    ]
    */

    Ok(())
}
```