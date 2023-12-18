use clap::Parser;
use lipsum::lipsum_words_with_rng;
use rand::{rngs::StdRng, Rng, SeedableRng};
use title::TitleCase;

mod cli;
mod title;

const MIN: usize = 64;
const MAX: usize = 192;

fn main() {
    let args = cli::Args::parse();

    let seed = args.seed();

    if args.dump_seed() {
        eprintln!("{seed}");
    }

    let mut rng = StdRng::seed_from_u64(seed as u64);

    if args.title() {
        let title_len = rng.gen::<usize>() % 10;

        let title = lipsum_words_with_rng(&mut rng, title_len).title_case();

        if args.md() {
            print!("# ")
        }

        println!("{title}\n")
    }

    if args.paragraph() {
        let base = MIN;

        (0..args.length())
            .map(|_| base + rng.gen::<usize>() % MAX)
            .collect::<Vec<usize>>()
            .iter()
            .map(|length| lipsum_words_with_rng(&mut rng, *length))
            .for_each(|paragraph| println!("{paragraph}\n"));
    } else {
        let lorem = lipsum_words_with_rng(rng, args.length());

        println!("{lorem}")
    }
}
