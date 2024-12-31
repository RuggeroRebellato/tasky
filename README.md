# Tasky: A Simple CLI Task Manager

**Tasky** is the centerpiece of my upcoming Rust course, designed for beginners and intermediate learners eager to master Rust through a practical and engaging project. This project provides a hands-on approach to learning Rust by covering foundational and production-ready concepts.

Whether you're new to programming or transitioning from another language, Tasky will expose you to core programming practices in a fun and practical way.

---

## Features

1. **Manage Tasks**:

   - Add tasks with descriptions and tags.
   - List all tasks with visual indicators for completion.
   - Mark tasks as complete.
   - Remove tasks as needed.

2. **Tag System**:

   - Manage tags interactively.
   - Add new tags on the fly.
   - Select tags from an organized list.

3. **Interactive CLI**:

   - Navigate menus with ease.
   - Beautifully formatted outputs for an intuitive experience.

4. **Real-World Rust Concepts**:

   - Command-line parsing with **Clap**.
   - Data serialization and persistence with **Serde**.
   - Interactive inputs using **Inquire**.
   - Error handling with **ThisError** and **Eyre**.
   - Modular design for scalability and maintainability.

---

## Why Tasky?

Tasky is more than just a task manager. It’s a hands-on learning tool that teaches Rust fundamentals while mimicking challenges faced in production environments:

- **Parsing Command-Line Arguments**: Learn how to build feature-rich CLI tools with Clap.
- **Data Persistence**: Understand how to use Serde for saving and loading structured data.
- **Error Handling**: Practice robust error handling using ThisError and Eyre.
- **Interactive CLI**: Get comfortable designing intuitive user experiences for terminal applications.
- **Project Structure**: Dive into Rust’s modular design principles to create clean, maintainable codebases.

---

## Getting Started

### Prerequisites

- Rust (1.80 or later)
- Cargo

### Installation

#### Install Rust

Using rustup (Recommended):

If you're running macOS, Linux, or another Unix-like OS, download Rustup and install Rust by running the following in your terminal. Follow the on-screen instructions.&#x20;

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Setup

Clone the repository:

```bash
$ git clone https://github.com/your-username/tasky.git
$ cd tasky
```

Build the project:

```bash
$ cargo build
```

Run the project:

```bash
$ cargo run
```

---

## Usage

Once you run Tasky, you'll be greeted with a menu of options:

```plaintext
Welcome to Tasky!
> What would you like to do?
  - Add Task
  - List Tasks
  - Complete Task
  - Remove Task
  - Exit
```

### Examples

**Add a Task**: Follow the prompts to add a task with a description and tags. Tasky supports selecting existing tags or creating new ones interactively.

**List Tasks**: View all your tasks with completion status. Tasks are neatly displayed with tags and identifiers.

**Complete a Task**: Select a task from the list to mark it as complete.

**Remove a Task**: Easily remove a task by selecting it from the menu.

---

## File Structure

Tasky uses JSON for data persistence:

- **tasks.json**: Stores your tasks.
- **tags.json**: Manages your tag system.

---

## Contributing

Contributions are welcome! If you’d like to improve Tasky or add new features, feel free to open a pull request or create an issue on GitHub.

---

## Upcoming Rust Course

Tasky is a unique learning project I plan to teach in my upcoming Rust course, providing a practical, hands-on approach to mastering foundational and production-ready Rust concepts. This project provides a hands-on approach to learning Rust, covering concepts like:

- Building CLI tools.
- Data serialization and deserialization.
- Error handling in production.
- Modular coding practices.

Stay tuned for updates on the course launch!

---

## License

This project is licensed under the MIT License. See the `LICENSE` file for more details.

---

**Start your Rust journey with Tasky today!** 🚀
