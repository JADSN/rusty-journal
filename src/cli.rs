use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Adicionar tarefa ao arquivo filename.json
    Add {
        /// Contém um String que descreve a tarefa que está sendo adicionada,
        #[structopt()]
        text: String,
    },
    /// Mantém o número de uma tarefa que marcaremos como concluída.
    /// Por exemplo, um 2 marcará a segunda tarefa na lista de tarefas pendentes numerada.
    Done {
        #[structopt()]
        position: usize,
    },
    /// Imprimirá a lista de tarefas no terminal.
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Rusty Journal",
    about = "A command line to-do app written in Rust"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    /// Define o arquivo a ser guardado as tasks
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}
