use clap::{Parser, Subcommand};
use todo_list::TodoList;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    New { task: String },
    Do { id: usize },
    Show {},
    Todo {},
    Done {},
}

fn main() {
    let cli = Cli::parse();
    let mut todo = TodoList::load("todo.txt");

    match &cli.command {
        Commands::New { task } => {
            todo.add_task(task);
        }
        Commands::Show {} => {
            todo.show_tasks();
        }
        Commands::Todo {} => {
            todo.show_todo();
        }
        Commands::Done {} => {
            todo.show_done();
        }
        Commands::Do { id } => {
            todo.dotask(id);
        }
    }

    todo.save("todo.txt");
}
