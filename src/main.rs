
use csv_types::*;

fn read_file() {
    let toggle_entries = csv::Reader::from_path("data/Toggl_Track_summary_report_2023-07-11_2023-07-11.csv").unwrap()
    .into_deserialize()
    .collect::<Result<Vec<ToggleTimeCSVEntry>, _>>().expect("Error reading toggle summary report");


    let mapping_entries = csv::Reader::from_path("data/mapping.csv").unwrap()
    .into_deserialize()
    .collect::<Result<Vec<MappingCSVEntry>, _>>().expect("Error reading mapping");

    print!("{toggle_entries:#?}");
    print!("{mapping_entries:#?}");
}

fn main()  {
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
