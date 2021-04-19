mod cli;
mod tasks;

use anyhow::anyhow;
use std::path::PathBuf;
use structopt::StructOpt;

use cli::{Action::*, CommandLineArgs};
use tasks::Task;

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}

fn main() -> anyhow::Result<()> {
    // Get the command-line arguments.
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    // Abre o arquivo journal
    let journal_file = journal_file
        .or_else(find_default_journal_file)
        .ok_or(anyhow!("Errir ao abrir o aquivo"))?;

    // Inicializa cli
    match action {
        Add { text } => tasks::add_task(journal_file, Task::new(text)),
        List => tasks::list_tasks(journal_file),
        Done { position } => tasks::complete_task(journal_file, position),
    }?;

    Ok(())
}
