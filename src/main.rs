use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, about)]
struct Cli {
    #[clap(help = "A list of file paths or regular expressions to match")]
    files: Vec<String>,
}

fn main() {
    human_panic::setup_panic!();

    let args = Cli::parse();

    let canon_paths = args
        .files
        .iter()
        .map(|f| {
            let components = f.split(['/', '\\']).collect::<Vec<&str>>();

            dunce::canonicalize(f)
                .unwrap_or_else(|_| panic!("Failed to canonicalize path (invalid path: {})", f))
        })
        .collect::<Vec<_>>();

    trash::delete_all(canon_paths).expect("failed to move files to trash");
}
