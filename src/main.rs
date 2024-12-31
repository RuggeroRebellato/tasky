use clap::{Parser, Subcommand};
use colored::*;
use eyre::Result;
use tasky::{
    store::{load_tags, load_tasks, save_tags, save_tasks, DATA_FILE},
    task::Task,
};

use inquire::{validator::Validation, MultiSelect, Select, Text};

#[derive(Parser)]
#[command(name = "Tasky")]
#[command(about = "Gerenciador de Tarefas CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add,
    List,
    Complete,
    Remove,
}

fn main() -> Result<()> {
    println!("{}", "Bem-vindo ao Tasky!".green());
    main_menu()
}

fn list_tasks() -> Result<()> {
    let tasks = load_tasks(DATA_FILE)?;
    if tasks.is_empty() {
        println!("{}", "Nenhuma tarefa encontrada.".yellow());
    } else {
        for task in &tasks {
            let status = if task.done {
                "✔".green()
            } else {
                "✘".red()
            };
            println!("{} [{}] {}", status, task.id, task.description);
        }
    }
    Ok(())
}

fn interactive_add_task() -> Result<()> {
    // Carregar as tags existentes
    let mut tags = load_tags()?;

    // Exibir lista de tags e perguntar o próximo passo
    println!("\nTags atuais disponíveis:");
    println!("{}", format_tags(&tags, 3));

    let action = Select::new(
        "O que você deseja fazer?",
        vec![
            "Adicionar novas tags",
            "Selecionar tags existentes",
            "Cancelar",
        ],
    )
    .prompt()?;

    match action {
        "Adicionar novas tags" => {
            // Adicionar novas tags
            loop {
                let new_tag =
                    Text::new("Digite o nome da nova tag (ou deixe em branco para finalizar):")
                        .prompt()?;
                if new_tag.trim().is_empty() {
                    break;
                }
                if !tags.contains(&new_tag) {
                    tags.push(new_tag.clone());
                }
            }

            // Salvar as novas tags no store
            save_tags(&tags)?;
            println!("{}", "Novas tags adicionadas com sucesso!".green());
        }
        "Selecionar tags existentes" => {}
        "Cancelar" => {
            println!("{}", "Operação cancelada.".yellow());
            return Ok(()); // Sai do fluxo
        }
        _ => unreachable!(),
    }

    // Selecionar tags da lista atualizada
    let selected_tags = MultiSelect::new(
        "Selecione as tags para a tarefa (use espaço para marcar/desmarcar):",
        tags.clone(),
    )
    .prompt()
    .unwrap_or_default();

    // Obter descrição da tarefa
    let description = Text::new("Digite a descrição da tarefa:")
        .with_validator(|input: &str| {
            if input.trim().is_empty() {
                Err("A descrição não pode estar vazia.".into())
            } else {
                Ok(Validation::Valid)
            }
        })
        .prompt()
        .unwrap_or_else(|_| "Tarefa sem descrição".to_string());

    // Criar nova tarefa
    let mut tasks = load_tasks(DATA_FILE)?;
    let id = tasks.len() as u32 + 1;
    let task = Task::new(id, description, selected_tags);
    tasks.push(task);
    save_tasks(DATA_FILE, &tasks)?;

    println!("{}", "Tarefa adicionada com sucesso!".green());
    Ok(())
}

fn interactive_complete_task() -> Result<()> {
    let tasks = load_tasks(DATA_FILE)?;
    if tasks.is_empty() {
        println!("{}", "Nenhuma tarefa encontrada para completar.".yellow());
        return Ok(());
    }

    let options: Vec<String> = tasks
        .iter()
        .map(|task| format!("{}: {}", task.id, task.description))
        .collect();

    let selected = Select::new("Selecione uma tarefa para completar:", options).prompt();

    match selected {
        Ok(choice) => {
            let id: u32 = choice.split(':').next().unwrap().parse().unwrap();
            let mut tasks = load_tasks(DATA_FILE)?;
            if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
                task.done = true;
                save_tasks(DATA_FILE, &tasks)?;
                println!("{}", "Tarefa marcada como concluída!".green());
            }
        }
        Err(_) => println!("{}", "Nenhuma tarefa foi selecionada.".yellow()),
    }

    Ok(())
}

fn interactive_remove_task() -> Result<()> {
    let tasks = load_tasks(DATA_FILE)?;
    if tasks.is_empty() {
        println!("{}", "Nenhuma tarefa encontrada para remover.".yellow());
        return Ok(());
    }

    let options: Vec<String> = tasks
        .iter()
        .map(|task| format!("{}: {}", task.id, task.description))
        .collect();

    let selected = Select::new("Selecione uma tarefa para remover:", options).prompt();

    match selected {
        Ok(choice) => {
            let id: u32 = choice.split(':').next().unwrap().parse().unwrap();
            let mut tasks = load_tasks(DATA_FILE)?;
            let len_before = tasks.len();
            tasks.retain(|t| t.id != id);
            if tasks.len() < len_before {
                save_tasks(DATA_FILE, &tasks)?;
                println!("{}", "Tarefa removida com sucesso!".green());
            }
        }
        Err(_) => println!("{}", "Nenhuma tarefa foi selecionada.".yellow()),
    }

    Ok(())
}

fn format_tags(tags: &[String], columns: usize) -> String {
    let mut formatted = String::new();
    for (i, tag) in tags.iter().enumerate() {
        formatted.push_str(&format!("{:<15}", tag));
        if (i + 1) % columns == 0 {
            formatted.push('\n');
        }
    }
    formatted
}

fn main_menu() -> Result<()> {
    loop {
        let choices = vec![
            "Adicionar Tarefa",
            "Listar Tarefas",
            "Completar Tarefa",
            "Remover Tarefa",
            "Sair",
        ];

        let choice = Select::new("O que você deseja fazer?", choices).prompt();

        match choice {
            Ok(option) => match option {
                "Adicionar Tarefa" => {
                    interactive_add_task()?;
                }
                "Listar Tarefas" => {
                    list_tasks()?;
                }
                "Completar Tarefa" => {
                    interactive_complete_task()?;
                }
                "Remover Tarefa" => {
                    interactive_remove_task()?;
                }
                "Sair" => {
                    println!("{}", "Até logo!".green());
                    break;
                }
                _ => unreachable!(),
            },
            Err(_) => {
                println!("{}", "Erro ao processar a escolha. Tente novamente.".red());
            }
        }
    }

    Ok(())
}
