mod md;

use std::path::PathBuf;

use clap::{command, Parser, Subcommand};

use crate::md::MarkdownContent;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    LocalizeImage {
        #[arg(short, long)]
        source: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::LocalizeImage { source }) => {
            let res = MarkdownContent::load_from_file(source.clone()).map_err(|e| {
                println!("🚨\t{:?}", e);
                return e;
            });
            if res.is_err() {
                return;
            }
            let mut content = res.unwrap();
            let res = content.localize_image().map_err(|e| {
                println!("🚨\t{:?}", e);
                return e;
            });
            if res.is_err() {
                return;
            }
            let old_file_name = source
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
                .to_string();
            let new_file_name = format!(
                "{}-localized.md",
                source
                    .file_stem()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default()
                    .to_string()
            );
            let new_file_path_string = source
                .to_string_lossy()
                .replace(&old_file_name, &new_file_name);
            let _ = content
                .save_to_local(PathBuf::from(new_file_path_string))
                .map_err(|e| {
                    println!("🚨\t{:?}", e);
                });
        }
        None => {
            println!("Please specify a COMMAND, use --help to see more")
        }
    }
}
