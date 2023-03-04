use ichiran::IchiranCli;
use std::{error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let cli = IchiranCli::new(PathBuf::from("./data/ichiran-cli"));

    let romanized = cli.romanize("いい天気ですね。")?;
    println!("{romanized}");
    /* outputs
    iitenki desu ne.
    */

    let info = cli.romanize_with_info("いい天気ですね。")?;
    println!("{info:#?}");
    /* outputs
    RomanizedWithInfo {
        romanized: "iitenki desu ne.",
        entries: [
            RomanizedWithInfoEntry {
                word: "* iitenki  いい天気 【いいてんき】",
                alternatives: [
                    "1. [n,exp] fine weather; fair weather",
                ],
            },
            RomanizedWithInfoEntry {
                word: "* desu  です",
                alternatives: [
                    "1. [cop] be; is",
                    "[ Conjugation: [cop] NIL Affirmative Formal",
                    "  だ : be; is ]",
                ],
            },
        ],
    }
    */

    Ok(())
}
