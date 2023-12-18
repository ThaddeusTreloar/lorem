use clap::Parser;
use lipsum::lipsum_words_with_rng;
use rand::{RngCore, Rng, thread_rng, rngs::StdRng, SeedableRng};

const MIN: usize = 64;
const MAX: usize = 192;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
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
    #[arg(short)]
    seed: Option<u64>,
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

    pub fn seed(&self) -> u64 {
        self.seed.unwrap_or(thread_rng().gen::<u64>())
    }
}


fn main() {
    let args = Args::parse();

    if args.title() {
        let title = lipsum::lipsum_title();

        if args.md() {
            print!("# ")
        }
        
        println!("{title}\n")
    }

    let seed = args.seed();

    println!("Seed: {seed}\n");

    let mut rng = StdRng::seed_from_u64(seed);

    if args.paragraph() {
        let base = MIN;

        let lens = (0..args.length())
            .map(
                |_| base + rng.gen::<usize>() % MAX
            )
            .map(
                |length| lipsum_words_with_rng(&mut rng, length)
            )
            .for_each(
                |paragraph| println!("{paragraph}\n")
            );
    } else {
        let lorem = lipsum_words_with_rng(rng, args.length());

        println!("{lorem}")
    }
}
