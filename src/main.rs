use crate::taskfile::taskfile::Taskfile;

pub mod taskfile;

fn main() {
    let tf: Taskfile = serde_yaml::from_str( include_str!("../net-task.yml") ).expect("could not parse file");

    println!("{}", tf.version);
}
