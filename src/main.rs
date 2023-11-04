#[macro_use] extern crate prettytable;

use crate::taskfile::taskfile::Taskfile;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use prettytable::{Table, Row, Cell};

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
    Run
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
            // todo: refactor this to use the standard iterator... which will certainly handle more than 1 deep external...
            for x in task_list.tasks.iter() {
                let task_name = x.name.clone().unwrap_or("UNKNOWN".to_string());
                let description = x.description.clone().unwrap_or(" ".to_string());

                table.add_row(row![task_name, description]);
            }
            for external in task_list.externals {
                for x in external.tasks.iter() {
                    let task_name = x.name.clone().unwrap_or("UNKNOWN".to_string());
                    let description = x.description.clone().unwrap_or(" ".to_string());
                    table.add_row(row![task_name, description]);
                }
            }

            table.printstd();
        }

        Commands::Run => {
            println!("run called!")
        }
    }
    // let tf = Taskfile::new_from_file("./net-task.yml".to_string());

    // let _res = tf.tasks.first().expect("no tasks given").run();
    // println!("{:?}", tf);
}
