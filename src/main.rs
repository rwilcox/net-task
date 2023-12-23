#[macro_use] extern crate prettytable;

use crate::taskfile::taskfile::{Taskfile, TaskDefinition};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use prettytable::{Table};


pub mod taskfile;

//const default_file: PathBuf = PathBuf::from("./net-task.yml");

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    taskfile: Option<PathBuf>,
}


#[derive(Subcommand, Debug)]
enum Commands {
    /// lists all availiable tasks
    List,
    Run {
        command_name: String
    },
    /// like run but prints out the script source for the found item
    Print {
        command_name: String
    }
}


/// for the given Vector< ... of Taskfiles> loop through the tasks in each one.
/// Tasks directly inside (each item) will be iterated over first, then external
/// taskfile declarations inside those taskfiles will be iterated over in the same manner.
///
/// # Arguments
/// * `taskfiles` - Taskfiles to iterate over
/// * `callback_closure` - A mutable closure that takes a reference to a `TaskDefinition` object, called
/// for every task this iterator encounters. If the closure returns false the iteration is STOPPED
fn taskfile_iterator<F>(taskfiles: &Vec<Box<Taskfile>>,
                     mut callback_closure: F) where F: FnMut(&TaskDefinition) -> bool {

    for task_list in taskfiles {
        for x in task_list.tasks.iter() {
            let res = callback_closure(x);
            if !res {
                return
            }
        }
    }

    let externals: Vec<Box<Taskfile>> = taskfiles.iter().flat_map( |x| { x.externals.clone()}).collect();

    if !externals.is_empty() {
        taskfile_iterator(&externals, callback_closure);
    }
}


fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::List => {
            let task_list = Taskfile::new_from_file(cli.taskfile.unwrap_or(PathBuf::from("./net-task.yml")));
            let mut table = Table::new();

            // NOTE: this styling does not work in eshell, but works everywhere else...
            table.add_row(row![b => "name", "description"]);

            let b = Box::new(task_list);
            taskfile_iterator(&vec![b], |x| -> bool {
                let task_name = x.name.clone().unwrap_or("UNKNOWN".to_string());
                let description = x.description.clone().unwrap_or(" ".to_string());
                table.add_row(row![task_name, description]);
                true
            });

            table.printstd();
        }

        Commands::Run { command_name } => {
            let task_list = Taskfile::new_from_file(cli.taskfile.unwrap_or(PathBuf::from("./net-task.yml")));
            let b = Box::new(task_list);
            taskfile_iterator(&vec![b], |x| -> bool {
                if x.name.as_ref().unwrap() == command_name {
                    x.run();
                    false
                } else {
                    true
                }
            });
        }

        Commands::Print { command_name } => {
            // does everything run does but prints the found command
let task_list = Taskfile::new_from_file(cli.taskfile.unwrap_or(PathBuf::from("./net-task.yml")));
            let b = Box::new(task_list);
            taskfile_iterator(&vec![b], |x| -> bool {
                if x.name.as_ref().unwrap() == command_name {
                    println!("{}", x.script.trim());
                    false
                } else {
                    true
                }
            });
        }
    }
}
