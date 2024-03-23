use serde::Deserialize;

use std::process::{Command, Stdio, ExitStatus};
use std::io::{Write};

use tempfile::NamedTempFile;

fn create_temp_file(contents: String) -> Result<String, std::io::Error> {
    let mut temp_file = NamedTempFile::new()?;

    let path = temp_file.path().to_string_lossy().to_string();

    writeln!(temp_file.as_file_mut(), "{}", contents)?;
    temp_file.persist(&path)?;
    Ok(path)
}


#[derive(Debug, Deserialize, Clone)]
pub struct TaskDefinition {
    pub os: Option<String>,
    pub name: Option<String>,
    pub command: String,
    pub script: String,
    pub description: Option<String>,
    pub as_tempfile: Option<bool>
}


impl TaskDefinition {
    pub fn run(&self) -> ExitStatus {
        let mut run_task = Command::new(self.command.clone());

        let needs_tempfile = self.as_tempfile.unwrap_or(false);
        let child_task = if !needs_tempfile {
            run_task.stdin(Stdio::piped());
            let mut child_task = run_task.spawn().unwrap();
            let _ = child_task.stdin.as_mut()
                .ok_or("Child process stdin has not been captured!").unwrap()
                .write_all(self.script.clone().as_bytes());
            child_task
        } else {
            let tmpfile = create_temp_file(self.script.clone());
            run_task.arg(tmpfile.expect("Error when unwrapping tmp file"));

            run_task.spawn().unwrap()
        };
        let output = child_task.wait_with_output();
        //println!("{:?}", output);
        output.unwrap().status
    }
}
