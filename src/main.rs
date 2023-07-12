use std::{collections::HashMap, fmt::{Display}};

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
        .map(|entry| (TaskKey::from(&entry), entry.tidsreg_path))
        .collect::<HashMap<_, _>>();

    let mut mapped_outputs = Vec::new();
    let mut unmapped_outputs = Vec::new();
    for toggle_entry in toggle_entries {
        let path_option = mapping_entries_map.get(&TaskKey::from(&toggle_entry));

        match path_option {
            Some(path) => mapped_outputs.push(Output {
                description: toggle_entry.description,
                tidsreg_path: path.clone(),
                duration: toggle_entry.duration,
            }),
            None => unmapped_outputs.push(toggle_entry),
        }
    }

    mapped_outputs.sort_by_key(|e| e.tidsreg_path.clone());


    println!("====================== Sucessfully mapped ======================");
    println!("{mapped_outputs:#?}");

    if !unmapped_outputs.is_empty() {
        println!("====================== Add mapping data ======================");
        let missing_mapping_keys = unmapped_outputs.iter().map(|e| TaskKey::from(e)).collect::<Vec<_>>();
        println!("{}", TaskKey::display_vec(missing_mapping_keys));
    }

}

fn main() {
    read_file()
}

mod csv_types {
    #[derive(serde::Deserialize, Debug)]
    pub struct ToggleTimeCSVEntry {
        #[serde(rename = "Project")]
        pub project: String,
        #[serde(rename = "Client")]
        pub client: String,
        #[serde(rename = "Description")]
        pub description: String,
        #[serde(rename = "Duration")]
        pub duration: String,
    }

    #[derive(serde::Deserialize, Debug)]
    pub struct MappingCSVEntry {
        #[serde(rename = "Project")]
        pub project: String,
        #[serde(rename = "Client")]
        pub client: String,
        #[serde(rename = "Description")]
        pub description: String,
        #[serde(rename = "TidsregPath")]
        pub tidsreg_path: String,
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct TaskKey {
    pub project: String,
    pub client: String,
    pub description: String,
}

impl Display for TaskKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.project, self.client, self.description)
    }
}

impl TaskKey {
    fn display_vec(vec: Vec<Self>) -> String {
        let mut output = String::new();
        for task_key in vec {
            output += &task_key.to_string();
            output += "\n"
        }

        
        return output;
    }
}

impl From<&ToggleTimeCSVEntry> for TaskKey {
    fn from(value: &ToggleTimeCSVEntry) -> Self {
        Self {
            project: value.project.clone(),
            client: value.client.clone(),
            description: value.description.clone(),
        }
    }
}

impl From<&MappingCSVEntry> for TaskKey {
    fn from(value: &MappingCSVEntry) -> Self {
        Self {
            project: value.project.clone(),
            client: value.client.clone(),
            description: value.description.clone(),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Output {
    pub description: String,
    pub tidsreg_path: String,
    pub duration: String,
}
