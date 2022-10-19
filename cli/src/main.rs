use clap::{arg, Command};

fn cli() -> Command {
    Command::new("dm")
        .about("Darkmatter parser's CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("parse")
                .about("Parses markdown into an output format (default is HTML")
                .arg(arg!(<FILE> "the file to convert"))
                .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("tokens")
                .about("Generates \"tokens\" for a given markdown file; where a tokens are words in the document after removing stop words and stemming")
                .arg(arg!(<File> "the markdown file to use"))
                .arg(
                    arg!(--lang <LANG>)
                        .num_args(0..=1)
                        .default_value("English")
                        .default_missing_value("English")
                )
                .arg_required_else_help(true)
        )
}

fn push_args() -> Vec<clap::Arg> {
    vec![arg!(-m --message <MESSAGE>)]
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("parse", sub_matches)) => {
            println!(
                "Parsing {}",
                sub_matches.get_one::<String>("FILE").expect("required")
            )
        }
        Some(("tokens", sub_matches)) => {
            let file = sub_matches
                .get_one::<String>("file")
                .map(|s| s.as_str())
                .expect("required field");
            let lang = sub_matches
                .get_one::<String>("lang")
                .map(|s| s.as_str())
                .expect("has default values");

            println!("Tokens [{}] from {} were:", lang, file)
        }

        _ => unreachable!(),
    }
}
