use ureq;
use serde::Deserialize;
use std::path::PathBuf;
use std::fs;

 use crate::taskfile::taskdefinition::TaskDefinition;

/// we want to allow the user to give us YAML hashlike syntax but turn it into an array
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
                let os = item.os.clone().unwrap_or("any".to_string());
                let anys = "any".to_string();

                let unix_like = ["linux", "freebsd", "macos", "openbsd", "netbsd"];
                let current_os = std::env::consts::OS;

                let can_be_executed = match os {
                    _ if os == anys => true,
                    _ if (os == "unix".to_string() && (unix_like.contains(&current_os)))  => true,
                    _ if os == current_os => true,
                    _ => false,
                };

                if can_be_executed {
                    self.0.push(item);
                }
            }
            Ok(self.0)
        }
    }

    des.deserialize_map(Vis(vec![]))
}

type Taskfiles = Vec<Box<Taskfile>>;

/// externals refer to other task files in other locations. Resolve the location and parse, returning us a list of all Taskfiles you found
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
                if location.starts_with("http") {
                    externals.push(Box::new(Taskfile::new_from_url(location)))
                } else {
                    let file_path = PathBuf::from(location);
                        externals.push(Box::new(Taskfile::new_from_file(file_path)));
                    }
            }

            Ok(externals)
        }
    }

    des.deserialize_seq(Vis(Taskfiles::new()))

}


#[derive(Debug, Deserialize, Clone)]
pub struct Taskfile {
    pub version: String,

    // sadly we can't just do vvvv because we may have multiple items
    // with the same name, varying only in specified OS
    //pub tasks: HashMap<String, TaskDefinition>,

    #[serde(default)]
    #[serde(deserialize_with = "tasks_definition")]
    pub tasks: Vec<TaskDefinition>,

    #[serde(default)]
    #[serde(deserialize_with = "external_definition")]
    pub externals: Taskfiles,

    pub from_file: Option<String>,

    pub name: String
}

impl Taskfile {

    /// filepath may be a relative path OR (eventually) a URL
    pub fn new_from_file(filepath: PathBuf) -> Taskfile {
        let fpath = filepath.as_path();

        let in_str = fs::read_to_string(fpath).expect("Unable to read given file");

        Taskfile::new_from_string(in_str, filepath.to_str().unwrap().to_string())
    }

    pub fn new_from_string(in_str: String, location: String) -> Taskfile {
        let mut output: Taskfile = serde_yaml::from_str(&in_str).expect("could not parse file");
        output.from_file = Some(location);
        return output
    }

    pub fn new_from_url(extend_url_str: String) -> Taskfile {
         let body: String = ureq::get(extend_url_str.as_str()).call().unwrap()
            .into_string().expect("could not fetch url");

        Taskfile::new_from_string(body, extend_url_str)
    }

}
