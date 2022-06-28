//! Configuration for Toipe.
//!
//! Designed for command-line arguments using [`clap`], but can be used
//! as a library too.

use clap::{ArgEnum, Parser};

use crate::wordlists::BuiltInWordlist;

const CLI_HELP: &str = "A trusty terminal typing tester.

Keyboard shortcuts:
ctrl-c: quit
ctrl-r: restart test with a new set of words
ctrc-w: delete last word
";

/// Main configuration for Toipe.
#[derive(Parser)]
#[clap(author, version, about = CLI_HELP)]
pub struct ToipeConfig {
    /// Word list name.
    #[clap(arg_enum, short, long, default_value_t = BuiltInWordlist::Top250)]
    pub wordlist: BuiltInWordlist,
    /// Path to custom word list file.
    ///
    /// Providing this will override `-w`/`--wordlist`.
    // TODO: find a way to ensure `-w` isn't provided along with this.
    #[clap(short = 'f', long = "file")]
    pub wordlist_file: Option<String>,
    /// Number of words to show on each test.
    #[clap(short, long, default_value_t = 30)]
    pub num_words: usize,
    /// Name of file to store test results in after completion
    #[clap(short = 'r', long = "result", default_value = "results.csv")]
    pub result_file: String,
}

impl ToipeConfig {
    /// Name of the text used for typing test
    pub fn text_name(&self) -> String {
        if let Some(wordlist_file) = &self.wordlist_file {
            format!("custom file `{}`", wordlist_file)
        } else {
            if let Some(possible_value) = self.wordlist.to_possible_value() {
                possible_value.get_name()
            } else {
                "unknown"
            }
            .to_string()
        }
    }
}
