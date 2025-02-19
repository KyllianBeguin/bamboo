use std::fs;
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::collections::BTreeMap;

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
	_values: Vec<String>
}

fn main() {
	// Get arguments
	let args = Cli::parse();

	// Read file
	let _content = fs::read_to_string(&args.path_config).expect("Unable to read file");
	let _config: Config = serde_yaml::from_str(&_content).unwrap();
	println!("Using bamboo version {} for dataset {}", _config.bamboo_version, _config.dataset_name);
	for (_, col_config) in &_config.columns {
		println!("{:?}", col_config)
	}
	// TODO: Fix the column access
	//for val in &_config.columns[0]._values {
	//	println!("{:?}", val)
	//}
}
