use clap::Parser;
use untitled10::{classify_lyrics, get_all_zeroshotcandidates, read_lyrics};

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Noah Gift",
    about = "A Command-line tool to analyze lyrics to songs and put them into a sqlite database."
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Noah Gift")]
    Classify {
        #[clap(short, long, default_value = "lyrics.txt")]
        file: String,
    },
    Candidates {},
    Lyrics {
        #[clap(short, long, default_value = "lyrics.txt")]
        file: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Classify { file }) => {
            println!("Classify {}", file);
            let lyrics = read_lyrics(&file);
            let result = classify_lyrics(lyrics).unwrap();
            // print out the results in a nice format
            for label in result {
                for l in label {
                    println!("{}: {}", l.text, l.score);
                }
            }
        }
        // use get_all_zeroshotcandidates() from lib.rs to get all candidates
        Some(Commands::Candidates {}) => {
            for candidate in get_all_zeroshotcandidates() {
                println!("{}", candidate);
            }
        }
        Some(Commands::Lyrics { file }) => {
            println!("Lyrics {}", file);
            for line in read_lyrics(&file) {
                println!("{}", line);
            }
        }
        None => {
            println!("No command given");
        }
    }
}
