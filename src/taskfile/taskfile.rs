use serde::Deserialize;
use std::process::{Command, Stdio, ExitStatus};
use std::io::{self, Write};

use std::path::Path;
use std::fs;

use url::Url;

#[derive(Debug, Deserialize)]
pub struct TaskDefinition {
    pub os: Option<String>,
    pub name: Option<String>,
    pub command: String,
    pub script: String,

}

impl TaskDefinition {
    pub fn run(&self) -> ExitStatus {
        let mut run_task = Command::new(self.command.clone())
            .stdin(Stdio::piped())
            .spawn()
            .unwrap();
        let _ = run_task.stdin.as_mut()
            .ok_or("Child process stdin has not been captured!").unwrap()
            .write_all(self.script.clone().as_bytes());

        let output = run_task.wait_with_output();
        println!("{:?}", output);
        return output.unwrap().status;
    }
}

/// mostly from https://stackoverflow.com/a/72947051/224334
fn tasks_definition<'de, D>(des: D) -> Result<Vec<TaskDefinition>, D::Error> where
    D: serde::Deserializer<'de>,
{
    struct Vis(Vec<TaskDefinition>);
    impl<'de> serde::de::Visitor<'de> for Vis {
        type Value = Vec<TaskDefinition>;
        fn expecting(&self, _formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            todo!("return nice descriptive error")
        }

        fn visit_map<A>(mut self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>,
        {
             while let Some((name, mut item)) = map.next_entry::<String, TaskDefinition>()? {
                item.name = Some(name);
                self.0.push(item);
            }
            Ok(self.0)
        }
    }

    des.deserialize_map(Vis(vec![]))
}

type Taskfiles = Vec<Box<Taskfile>>;

fn external_definition<'de, D>(des: D) -> Result<Taskfiles, D::Error> where
    D: serde::Deserializer<'de>,
{
    struct Vis(Taskfiles);
    impl<'de> serde::de::Visitor<'de> for Vis {
        type Value = Taskfiles;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
             write!(formatter, "sequence")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where
            A: serde::de::SeqAccess<'de>,
        {
            let mut externals = Taskfiles::new();
            while let Some(location) = seq.next_element::<String>()? {
                externals.push(Box::new(Taskfile::new_from_file(location)));
            }

            Ok(externals)
        }
    }

    des.deserialize_seq(Vis(Taskfiles::new()))

}


#[derive(Debug, Deserialize)]
pub struct Taskfile {
    pub version: String,

    // sadly we can't just do vvvv because we may have multiple items
    // with the same name, varying only in specified OS
    //pub tasks: HashMap<String, TaskDefinition>,

    #[serde(deserialize_with = "tasks_definition")]
    pub tasks: Vec<TaskDefinition>,

    #[serde(default)]
    #[serde(deserialize_with = "external_definition")]
    pub externals: Taskfiles
}

impl Taskfile {

    /// filepath may be a relative path OR (eventually) a URL
    pub fn new_from_file(filepath: String) -> Taskfile {
        let fpath = Path::new(&filepath);

        let in_str = fs::read_to_string(fpath).expect("Unable to read given file");

        let output: Taskfile = serde_yaml::from_str(&in_str).expect("could not parse file");
        return output
    }

}
