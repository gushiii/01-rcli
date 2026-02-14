use std::fs;

use anyhow::Result;
use serde_json::{Map, Value};

use crate::cli::OutputFormat;

pub fn process_csv(
    input: &str,
    header: bool,
    delimiter: &char,
    output: String,
    format: OutputFormat,
) -> Result<()> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(header)
        .delimiter(*delimiter as u8)
        .from_path(input)?;

    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
        OutputFormat::Toml => {
            let mut root_map = Map::new();
            root_map.insert("data".to_string(), Value::Array(ret.clone()));
            let root_value = Value::Object(root_map);
            toml::to_string(&root_value)?
        }
    };

    fs::write(output, content)?;

    Ok(())
}
