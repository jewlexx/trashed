use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, about)]
struct Cli {
    files: Vec<String>,
}

fn main() {
    human_panic::setup_panic!();

    let args = Cli::parse();

    let canon_paths = args
        .files
        .iter()
        .map(|f| {
            dunce::canonicalize(f)
                .unwrap_or_else(|_| panic!("Failed to canonicalize path (invalid path: {})", f))
        })
        .collect::<Vec<_>>();

    trash::delete_all(canon_paths).expect("failed to move files to trash");
}
