use clap::{ArgAction, Parser};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(value_parser)]
    user_input: Vec<String>,

    #[clap(short, action = ArgAction::SetFalse)]
    no_trailing_newline: bool,
}

fn main() {
    let cli = Cli::parse();
    
    print!("{}", cli.user_input.join(" "));
    if cli.no_trailing_newline {
        print!("\n");
    }
}
