mod storage;

use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use directories::ProjectDirs;
use storage::Notebook;

#[derive(Parser)]
#[command(version, about = "A small notebook for your items")]
struct Cli {
    /// Store items in this directory instead of the platform data directory
    #[arg(long, global = true, value_name = "PATH")]
    data_dir: Option<PathBuf>,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Save a new item
    Add {
        title: String,
        #[arg(long, default_value = "")]
        body: String,
    },
    /// List saved items, oldest first
    List,
    /// Read an item
    Show { id: u64 },
    /// Permanently remove an item
    Remove { id: u64 },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let data_dir = match cli.data_dir {
        Some(path) => path,
        None => ProjectDirs::from("", "Klyve", "items")
            .context("Could not find your data directory; use --data-dir <path>")?
            .data_dir()
            .to_path_buf(),
    };
    let path = data_dir.join("items.json");
    let mut notebook = Notebook::load(&path)?;
    match cli.command {
        Command::Add { title, body } => {
            let id = notebook.add(&title, body)?;
            notebook.save(&path)?;
            println!("Added item {id}.");
        }
        Command::List => {
            if notebook.items().is_empty() {
                println!("No items yet. Use 'items add <title>' to save one.");
            }
            for item in notebook.items() {
                println!("{}  {}", item.id, item.title);
            }
        }
        Command::Show { id } => {
            let Some(item) = notebook.items().iter().find(|item| item.id == id) else {
                bail!("Item {id} not found");
            };
            println!("{}  {}", item.id, item.title);
            if !item.body.is_empty() {
                println!("\n{}", item.body);
            }
        }
        Command::Remove { id } => {
            notebook.remove(id)?;
            notebook.save(&path)?;
            println!("Removed item {id}.");
        }
    }
    Ok(())
}
