use csv::Writer;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};

use csv_types::*;

fn read_file() {
    let toggle_entries =
        csv::Reader::from_path("data/report.csv")
            .unwrap()
            .into_deserialize()
            .collect::<Result<Vec<ToggleTimeCSVEntry>, _>>()
            .expect("Error reading toggle summary report");

    let mut mapping_entries: Vec<MappingCSVEntry> = csv::Reader::from_path("data/mapping.csv")
        .unwrap()
        .into_deserialize()
        .collect::<Result<_, _>>().unwrap();

    if mapping_entries.is_empty() {
        panic!("No mappings were read");
    }

    let mut wtr = Writer::from_path("data/mapping.csv").expect("failed to open mapping.csv for writing");
    mapping_entries.sort();

    wtr.write_field("Project").unwrap();
    wtr.write_field("Client").unwrap();
    wtr.write_field("Description").unwrap();
    wtr.write_field("TidsregPath").unwrap();
    wtr.write_record(None::<&[u8]>).unwrap();

    for record in mapping_entries.iter() {
        wtr.write_field(record.task_key.project.clone()).unwrap();
        wtr.write_field(record.task_key.client.clone()).unwrap();
        wtr.write_field(record.task_key.description.clone()).unwrap();
        wtr.write_field(record.tidsreg_path.clone()).unwrap();
        wtr.write_record(None::<&[u8]>).unwrap();
        //wtr.write_field(record.tidsreg_path.clone()).unwrap();
        //wtr.serialize(record).unwrap();
    }
    wtr.flush().unwrap();

    let mapping_entries_map = mapping_entries.into_iter()
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
        let missing_mapping_keys = unmapped_outputs.into_iter();
        for ele in missing_mapping_keys {
            println!("{ele}");
        }
    }
}





fn main() {
    read_file()
}

mod csv_types {
    use std::fmt::Display;

    use serde::{Deserialize, Serialize};

    use crate::TaskKey;

    #[derive(Deserialize, Debug)]
    pub struct ToggleTimeCSVEntry {
        #[serde(flatten)]
        pub task_key: TaskKey,
        #[serde(rename = "Duration")]
        pub duration: String,
    }

    
    impl Display for ToggleTimeCSVEntry {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "TaskKey: {},{},{}, {}", self.task_key.project, self.task_key.client, self.task_key.description, self.duration)
        }
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct MappingCSVEntry {
        #[serde(flatten)]
        pub task_key: TaskKey,
        #[serde(rename = "TidsregPath")]
        pub tidsreg_path: String,
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
        write!(f, "TaskKey: {},{},{}", self.project, self.client, self.description)
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
        write!(f, "Output: \t{}, \t{}, {}", self.description, self.tidsreg_path, self.duration)
    }
}