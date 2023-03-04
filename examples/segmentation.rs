use ichiran::IchiranCli;
use std::{error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let cli = IchiranCli::new(PathBuf::from("./data/ichiran-cli"));

    let segmentation = cli.segment("いい天気ですね。")?; //")?;
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
