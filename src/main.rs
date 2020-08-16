use exitfailure::ExitFailure;
use failure::ResultExt;
use io::BufWriter;
use std::{fs, io, path::PathBuf};
use structopt::StructOpt;

mod id;
mod outlets;

#[derive(StructOpt)]
struct XibT {
    #[structopt(subcommand)]
    command: Command,
    #[structopt(parse(from_os_str))]
    files: Vec<PathBuf>,
}

#[derive(StructOpt)]
enum Command {
    Outlets { collection: String, xpath: String },
    Unconstrain,
}

fn main() -> Result<(), ExitFailure> {
    let app: XibT = XibT::from_args();

    let file_contents: Vec<String> = app
        .files
        .iter()
        .map(|path| {
            fs::read_to_string(path).with_context(|_| {
                format!(
                    "could not read file `{}`",
                    path.to_str().unwrap_or("for unknown path")
                )
            })
        })
        .collect::<Result<_, _>>()?;

    file_contents
        .iter()
        .map(|contents| match &app.command {
            Command::Outlets { collection, xpath } => {
                let stdout = io::stdout();
                let mut handle = BufWriter::new(stdout);

                outlets::outlets(&contents, &collection, &xpath, &mut handle)
            }
            Command::Unconstrain => todo!(),
        })
        .collect()
}
