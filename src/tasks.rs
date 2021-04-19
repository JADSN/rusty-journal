use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Task {
        let created_at: DateTime<Utc> = Utc::now();
        Task { text, created_at }
    }
}

use std::fmt;

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<50} [{}]", self.text, created_at)
    }
}

use std::{
    fs::{File, OpenOptions},
    io::{Error, ErrorKind, Result, Seek, SeekFrom},
    path::PathBuf,
};

// Requer um argumento Task. Esse argumento especifica a tarefa que será adicionada à lista.
pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;
    let mut tasks = collect_tasks(&file)?;
    tasks.push(task);
    serde_json::to_writer(file, &tasks)?;
    Ok(())
}

// Requer um argumento task_position para indicar qual Task será removida.
// Quando uma tarefa é removida, isso significa que ela está concluída.
pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()> {
    // Abre o arquivo.
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;

    // Obtem os `tasks` contidos no arquivo
    let mut tasks = collect_tasks(&file)?;

    // Remove a task.
    if task_position == 0 || task_position > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Task ID inválido"));
    }

    tasks.remove(task_position - 1);
    file.set_len(0)?;

    // Escreve o conteúdo no arquivo
    serde_json::to_writer(file, &tasks)?;
    Ok(())
}

// Não precisa de informações adicionais.
// Ela apenas apresentará ao usuário todas as tarefas armazenadas atualmente no arquivo de diário em um formato bonito.
pub fn list_tasks(journal_path: PathBuf) -> Result<()> {
    // Abre o arquivo
    let file = OpenOptions::new().read(true).open(journal_path)?;

    // Obtem as tasks do arquivo
    let tasks = collect_tasks(&file)?;

    // Enumerate and display tasks, if any.
    if tasks.is_empty() {
        println!("Lista de tasks vazia");
    } else {
        let mut order: u32 = 1;
        for task in tasks {
            println!("{}: {}", order, task);
            order += 1;
        }
    }

    Ok(())
}

// Obtem as tasks do arquivo
fn collect_tasks(mut file: &File) -> Result<Vec<Task>> {
    file.seek(SeekFrom::Start(0))?; // Rewind the file before.
    let tasks = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };

    // Movimenta o cursos para o início do arquivo
    file.seek(SeekFrom::Start(0))?;

    Ok(tasks)
}
