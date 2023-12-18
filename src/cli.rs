use clap::Parser;
use rand::{thread_rng, Rng};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    /// The length of the text to generate. This is wordcount by defualt. If p is specified then paragraph count.
    length: usize,
    /// Generate n paragraphs instead.
    #[arg(short)]
    paragraph: bool,
    /// Add a title to the text.
    #[arg(short)]
    title: bool,
    /// Whether to add markdown
    #[arg(short)]
    md: bool,
    /// The seed to use for the random number generator.
    #[arg(short)]
    seed: Option<usize>,
    /// Dump the seed to stderr.
    #[arg(short)]
    dump_seed: bool,
}

impl Args {
    pub fn length(&self) -> usize {
        self.length
    }

    pub fn paragraph(&self) -> bool {
        self.paragraph
    }

    pub fn title(&self) -> bool {
        self.title
    }

    pub fn md(&self) -> bool {
        self.md
    }

    pub fn seed(&self) -> usize {
        self.seed.unwrap_or(thread_rng().gen::<usize>())
    }

    pub fn dump_seed(&self) -> bool {
        self.dump_seed
    }
}
