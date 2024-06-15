use serde::Deserialize;

use std::process::{Command, Stdio, ExitStatus};
use std::io::{Write};
use crate::taskfile::taskfile::Taskfile;

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
    pub command: Option<String>,
    pub script: Option<String>,
    pub shell: Option<String>,
    pub description: Option<String>,
    pub as_tempfile: Option<bool>
}


impl TaskDefinition {
    pub fn get_command(&self) -> String {
        // TODO: maybe use the OS field to default to Powershell on win??
        self.command.clone().or(Some("/bin/bash".to_string())).unwrap()
    }

    pub fn get_script(&self) -> String {
        self.script.clone().or(self.shell.clone()).unwrap()
    }

    pub fn run(&self, parent_taskfile: &Taskfile) -> ExitStatus {
        let mut run_task = Command::new(self.get_command());
        let taskfile_dir = parent_taskfile.get_location_folder();

        let user_current_dir = std::env::current_dir().expect("could not get CWD");
        run_task.current_dir(taskfile_dir.unwrap_or(user_current_dir.clone()));

        run_task.env("NET_TASK_USER_CURRENT_DIRECTORY", user_current_dir);
        run_task.env("NET_TASK", std::env::current_exe().expect("could not get binary location"));

        let needs_tempfile = self.as_tempfile.unwrap_or(false);
        let child_task = if !needs_tempfile {
            run_task.stdin(Stdio::piped());
            let mut child_task = run_task.spawn().unwrap();
            let _ = child_task.stdin.as_mut()
                .ok_or("Child process stdin has not been captured!").unwrap()
                .write_all(self.get_script().as_bytes());
            child_task
        } else {
            let tmpfile = create_temp_file(self.get_script());
            run_task.arg(tmpfile.expect("Error when unwrapping tmp file"));

            run_task.spawn().unwrap()
        };
        let output = child_task.wait_with_output();
        //println!("{:?}", output);
        output.unwrap().status
    }
}
