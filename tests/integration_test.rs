use tasky::store::{load_tasks, save_tasks};
use tasky::task::Task;

const TEST_FILE: &str = "test_tasks.json";

#[test]
fn test_add_and_load_tasks() {
    let tasks = vec![
        Task::new(1, "Primeira tarefa".to_string(), vec!["tag1".to_string()]),
        Task::new(2, "Segunda tarefa".to_string(), vec![]),
    ];

    save_tasks(TEST_FILE, &tasks).expect("Salvar tarefas falhou");
    let loaded_tasks = load_tasks(TEST_FILE).expect("Carregar tarefas falhou");

    assert_eq!(tasks, loaded_tasks);

    // Limpeza do arquivo de teste
    std::fs::remove_file(TEST_FILE).unwrap();
}
