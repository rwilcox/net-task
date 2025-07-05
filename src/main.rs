#[macro_use] extern crate prettytable;

use crate::taskfile::taskfile::Taskfile;
use crate::taskfile::taskdefinition::TaskDefinition;

use clap::{Parser, Subcommand, ArgAction};
use std::path::{PathBuf};
use prettytable::{Table};
use std::env;

use std::fs;

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
    List {
        /// shows only tasks that start with the given pattern
        pattern_name: Option<String>,
        #[clap(long, help = "Plain print the output", action = ArgAction::SetTrue)]
        plain: Option<bool>
    },
    Run {
        command_name: String
    },
    /// like run but prints out the script source for the found item
    Print {
        command_name: String
    },

    /// Generates an example net-task.yml file
    Init
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
                     mut callback_closure: F) where F: FnMut(&TaskDefinition, &Taskfile) -> bool {

    for task_list in taskfiles {
        for x in task_list.tasks.iter() {
            let res = callback_closure(x, task_list);
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


fn find_file_upwards(file_name: &str) -> Option<PathBuf> {
    let mut current_dir = env::current_dir().unwrap();
    let home_dir = match env::var_os("HOME") {
        Some(val) => PathBuf::from(val),
        None => PathBuf::from(""),
    };

    loop {
        let file_path = current_dir.join(file_name);

        if file_path.exists() {
            return Some(file_path);
        }

        if current_dir == home_dir {
            break;
        }

        current_dir = match current_dir.parent() {
            Some(path) => path.to_path_buf(),
            None => break,
        };
    }

    None
}


fn main() {
    let cli = Cli::parse();

    // task file may exist here, may exist up directory OR may not exist at all!
    let task_file: Option<PathBuf> = cli.taskfile.clone().or(find_file_upwards("net-task.yml"));

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::List { pattern_name, plain } => {
            let task_list = Taskfile::new_from_file(cli.taskfile.unwrap_or(task_file.expect("did not find a net-task file!")));

            let mut table = Table::new();

            // NOTE: this styling does not work in eshell, but works everywhere else...
            table.add_row(row![b => "name", "description"]);

            let b = Box::new(task_list);
            taskfile_iterator(&vec![b], |x, _in_taskfile| -> bool {
                let task_name = x.name.clone().unwrap_or("UNKNOWN".to_string());
                let description = x.description.clone().unwrap_or(" ".to_string());
                let mut add_to_row = true;

                if pattern_name.is_some() {
                    // TODO: make this fancy

                    // for now if there's a star at the end of the string - habit - drop it
                    let pattern = pattern_name.as_ref().unwrap().trim_end_matches('*');
                    add_to_row =  task_name.starts_with(pattern)
                }

                if add_to_row {
                    table.add_row(row![task_name, description]);
                    if plain.unwrap_or(false) {
                        println!("{}", task_name)
                    }
                }
                true
            });

            if !(plain.unwrap_or(false)) {
                table.printstd();
            }
        }

        Commands::Run { command_name } => {
            let task_list = Taskfile::new_from_file(cli.taskfile.unwrap_or(task_file.expect("did not find specified net-task file")));
            let b = Box::new(task_list);
            taskfile_iterator(&vec![b], |x, current_taskfile| -> bool {
                if x.name.as_ref().unwrap() == command_name {
                    std::process::exit(x.run(current_taskfile).code().expect("exit code for child process not found"));
                    // false
                } else {
                    println!("ERROR: {} task not found. Check spelling or OS selector.", command_name);
                    std::process::exit(1);
                }
            });
        }

        Commands::Print { command_name } => {
            // does everything run does but prints the found command
            let task_list = Taskfile::new_from_file(cli.taskfile.unwrap_or(task_file.expect("did not find specified net-task file")));
            let b = Box::new(task_list);
            taskfile_iterator(&vec![b], |x, _in_taskfile| -> bool {
                if x.name.as_ref().unwrap() == command_name {
                    println!("{}", x.get_script().trim());
                    false
                } else {
                    true
                }
            });
        }

        Commands::Init => {
          let content = include_str!("../example.yml");
           match fs::write("net-task.yml", content) {
               Ok(_) => println!("Successfully created net-task.yml."),
               Err(e) => eprintln!("Failed to create net-task.yml: {}", e),
    }
        }
    }
}
