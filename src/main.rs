use clap::Parser;
use rand::seq::IndexedRandom;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::collections::BTreeMap;
use std::fs;
use std::io::prelude::*;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The path to the config file to read
    #[arg(default_value = "conf.yml")]
    path_config: std::path::PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    // The bamboo version
    bamboo_version: i8,
    // The dataset name
    dataset_name: String,
    // Columns configs
    columns: BTreeMap<String, ColumnConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ColumnConfig {
    // Column name
    _name: String,
    // Column type
    _type: String,
    // Column values
    _values: Vec<String>,
}

fn write_output(_config: Config) -> std::io::Result<()> {
    // Output fie - csv for now
    let mut output = fs::File::create("data.csv").expect("Failed to create file");
    // csv header
    let mut header = String::from("");
    for (_, col_config) in &_config.columns {
        header.push_str(&col_config._name);
        header.push_str(";");
    }
    header.pop(); // Removes last ";"
    writeln!(output, "{}", header).expect("Failed to write formatted data");
    let mut body = String::from("");
    let mut record = String::from("");
    let mut rng = rand::rng();
    // must loop on a number of wished records
    for n in 1..10 {
        record.clear();
        for (_, col_config) in &_config.columns {
            // 1) select a value from column & push it to a record
            record.push_str(&col_config._values.choose(&mut rng).unwrap());
            // 3) push a ";" to the record
            record.push_str(";");
        }
        record.pop(); // Removes last ";"
        body.push_str(&record);
        body.push_str("\n");
    }
    writeln!(output, "{}", body).expect("Failed to write formatted data");
    // Note : adding \n between records works fine for duckdb processing
    // writeln!(output, "ben;hoe;tea\nbecky;smash;coffee").expect("Failed to write formatted data");
    Ok(())
}

fn main() {
    // Get arguments
    let args = Cli::parse();

    // Read file
    let _content = fs::read_to_string(&args.path_config).expect("Unable to read file");
    let dataset_config: Config = serde_yaml::from_str(&_content).unwrap();
    println!(
        "Using bamboo version {} for dataset {}",
        dataset_config.bamboo_version, dataset_config.dataset_name
    );
    // for (_, col_config) in &dataset_config.columns {
    //     println!("{:?}", col_config)
    // }
    write_output(dataset_config);
}
