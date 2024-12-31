use serde_json::Error as SerdeError;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

use colored::*;

use crate::task::Task;

pub const TAGS_FILE: &str = "tags.json";
pub const DATA_FILE: &str = "tasks.json";

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("Failed to read the file: {0}")]
    ReadError(#[from] io::Error),
    #[error("Failed to parse JSON: {0}")]
    ParseError(#[from] SerdeError),
}

pub fn load_tasks(path: &str) -> Result<Vec<Task>, StoreError> {
    if !Path::new(path).exists() {
        return Ok(Vec::new());
    }

    let data = fs::read_to_string(path)?;
    let tasks_obj: serde_json::Value = serde_json::from_str(&data)?;
    let tasks = tasks_obj["tasks"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|t| serde_json::from_value(t.clone()).unwrap())
        .collect();
    Ok(tasks)
}

pub fn save_tasks(path: &str, tasks: &[Task]) -> Result<(), StoreError> {
    let tasks_obj = serde_json::json!({ "tasks": tasks });
    let data = serde_json::to_string_pretty(&tasks_obj)?;
    let mut file = File::create(path)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

pub fn load_tags() -> Result<Vec<String>, StoreError> {
    if !Path::new(TAGS_FILE).exists() {
        return Ok(Vec::new());
    }

    let data = fs::read_to_string(TAGS_FILE)?;
    let tags: Vec<String> = serde_json::from_str(&data)?;
    Ok(tags)
}

pub fn save_tags(tags: &[String]) -> Result<(), StoreError> {
    let data = serde_json::to_string_pretty(tags)?;
    let mut file = File::create(TAGS_FILE)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

pub fn display_tags(tags: &[String]) {
    println!("{}", "Tags Disponíveis:".bold().underline());

    let columns = 3; // Número de colunas
    for (i, tag) in tags.iter().enumerate() {
        print!("{:<20}", tag.blue());
        if (i + 1) % columns == 0 {
            println!(); // Quebra de linha a cada `columns` tags
        }
    }
    println!(); // Quebra final
}

pub fn display_tasks(tasks: &[Task]) {
    println!("{}", "Tarefas:".bold().underline());
    println!("{:<5} {:<30} {:<10} Tags", "ID", "Descrição", "Status");

    for task in tasks {
        let status = if task.done {
            "✔️".green()
        } else {
            "⏳".yellow()
        };

        let tags = task.tags.join(", ");
        println!(
            "{:<5} {:<30} {:<10} {}",
            task.id,
            task.description,
            status,
            tags.cyan()
        );
    }
}
