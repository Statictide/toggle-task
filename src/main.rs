use serde::Deserialize;
use std::{collections::HashMap, fmt::Display};

use csv_types::*;

fn read_file() {
    let toggle_entries =
        csv::Reader::from_path("data/Toggl_Track_summary_report.csv")
            .unwrap()
            .into_deserialize()
            .collect::<Result<Vec<ToggleTimeCSVEntry>, _>>()
            .expect("Error reading toggle summary report");

    let mapping_entries = csv::Reader::from_path("data/mapping.csv")
        .unwrap()
        .into_deserialize()
        .collect::<Result<Vec<MappingCSVEntry>, _>>()
        .expect("Error reading mapping");

    let mapping_entries_map = mapping_entries
        .into_iter()
        .map(|entry| (entry.task_key, entry.tidsreg_path))
        .collect::<HashMap<_, _>>();

    let mut mapped_outputs = Vec::new();
    let mut unmapped_outputs = Vec::new();
    for toggle_entry in toggle_entries {
        let path_option = mapping_entries_map.get(&toggle_entry.task_key);

        match path_option {
            Some(path) => mapped_outputs.push(Output {
                description: toggle_entry.task_key.description,
                tidsreg_path: path.clone(),
                duration: toggle_entry.duration,
            }),
            None => unmapped_outputs.push(toggle_entry),
        }
    }

    mapped_outputs.sort_by_key(|e| e.tidsreg_path.clone());


    println!("====================== Sucessfully mapped ======================");
    for ele in mapped_outputs {
        println!("{ele}");
    }

    if !unmapped_outputs.is_empty() {
        println!("====================== Add mapping data ======================");
        let missing_mapping_keys = unmapped_outputs.into_iter().map(|e|e.task_key).collect::<Vec<_>>();
        for ele in missing_mapping_keys {
            print!("{ele}");
        }
    }

}

fn main() {
    read_file()
}

mod csv_types {
    use serde::Deserialize;

    use crate::TaskKey;

    #[derive(Deserialize, Debug)]
    pub struct ToggleTimeCSVEntry {
        #[serde(flatten)]
        pub task_key: TaskKey,
        #[serde(rename = "Duration")]
        pub duration: String,
    }

    #[derive(serde::Deserialize, Debug)]
    pub struct MappingCSVEntry {
        #[serde(flatten)]
        pub task_key: TaskKey,
        #[serde(rename = "TidsregPath")]
        pub tidsreg_path: String,
    }
}

#[derive(Deserialize, Hash, Eq, PartialEq, Debug)]
pub struct TaskKey {
    #[serde(rename = "Project")]
    pub project: String,
    #[serde(rename = "Client")]
    pub client: String,
    #[serde(rename = "Description")]
    pub description: String,
}

impl Display for TaskKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TaskKey: {{{},{},{}}}", self.project, self.client, self.description)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Output {
    pub description: String,
    pub tidsreg_path: String,
    pub duration: String,
}


impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Output: \t{{{}, \t{}, {}}}", self.description, self.tidsreg_path, self.duration)
    }
}