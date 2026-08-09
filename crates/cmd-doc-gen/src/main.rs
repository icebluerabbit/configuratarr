use core_cli::Cli;

fn main() {
    print!("{}", clap_markdown::help_markdown::<Cli>());
}
