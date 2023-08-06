use crate::taskfile::taskfile::Taskfile;

pub mod taskfile;

fn main() {
    let tf = Taskfile::new_from_file("./net-task.yml".to_string());

    let _res = tf.tasks.first().expect("huh").run();
    //println!("{:?}", tf);
}
