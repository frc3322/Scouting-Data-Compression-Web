use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ColumnKind {
    Int,
    Enum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum ColumnSchema {
    #[serde(rename = "int")]
    Int {
        name: String,
        bits: u32,
        int_max: u64,
    },
    #[serde(rename = "enum")]
    Enum {
        name: String,
        bits: u32,
        values: Vec<String>,
    },
}

impl ColumnSchema {
    pub fn name(&self) -> &str {
        match self {
            ColumnSchema::Int { name, .. } => name,
            ColumnSchema::Enum { name, .. } => name,
        }
    }

    pub fn kind(&self) -> ColumnKind {
        match self {
            ColumnSchema::Int { .. } => ColumnKind::Int,
            ColumnSchema::Enum { .. } => ColumnKind::Enum,
        }
    }

    pub fn bits(&self) -> u32 {
        match self {
            ColumnSchema::Int { bits, .. } => *bits,
            ColumnSchema::Enum { bits, .. } => *bits,
        }
    }

    pub fn int_max(&self) -> Option<u64> {
        match self {
            ColumnSchema::Int { int_max, .. } => Some(*int_max),
            ColumnSchema::Enum { .. } => None,
        }
    }

    pub fn values(&self) -> Option<&Vec<String>> {
        match self {
            ColumnSchema::Int { .. } => None,
            ColumnSchema::Enum { values, .. } => Some(values),
        }
    }
}

pub fn get_default_schema() -> Vec<ColumnSchema> {
    vec![
        ColumnSchema::Enum {
            name: "ScoutName".to_string(),
            bits: 2,
            values: vec!["Jude".to_string(), "Dillon".to_string(), "".to_string(), "".to_string()],
        },
        ColumnSchema::Int {
            name: "MatchNumber".to_string(),
            bits: 8,
            int_max: 200,
        },
        ColumnSchema::Int {
            name: "TeamNumber".to_string(),
            bits: 14,
            int_max: 16383,
        },
        ColumnSchema::Int {
            name: "Mobility".to_string(),
            bits: 1,
            int_max: 1,
        },
        ColumnSchema::Int {
            name: "AutonL1Attempted".to_string(),
            bits: 4,
            int_max: 12,
        },
        ColumnSchema::Int {
            name: "AutonL1Scored".to_string(),
            bits: 4,
            int_max: 12,
        },
        ColumnSchema::Int {
            name: "AutonL2Attempted".to_string(),
            bits: 4,
            int_max: 12,
        },
        ColumnSchema::Int {
            name: "AutonL2Scored".to_string(),
            bits: 4,
            int_max: 12,
        },
        ColumnSchema::Int {
            name: "AutonL3Attempted".to_string(),
            bits: 4,
            int_max: 12,
        },
        ColumnSchema::Int {
            name: "AutonL3Scored".to_string(),
            bits: 4,
            int_max: 12,
        },
        ColumnSchema::Int {
            name: "AutonL4Attempted".to_string(),
            bits: 4,
            int_max: 12,
        },
        ColumnSchema::Int {
            name: "AutonL4Scored".to_string(),
            bits: 4,
            int_max: 12,
        },
        ColumnSchema::Int {
            name: "AutonBargeAttempted".to_string(),
            bits: 0,
            int_max: 0,
        },
        ColumnSchema::Int {
            name: "AutonBargeScored".to_string(),
            bits: 0,
            int_max: 0,
        },
        ColumnSchema::Int {
            name: "AutonProcessorAttempted".to_string(),
            bits: 0,
            int_max: 0,
        },
        ColumnSchema::Int {
            name: "AutonProcessorScored".to_string(),
            bits: 0,
            int_max: 0,
        },
        ColumnSchema::Int {
            name: "AutonAlgaeRemoved".to_string(),
            bits: 0,
            int_max: 0,
        },
        ColumnSchema::Int {
            name: "TeleopL1Attempted".to_string(),
            bits: 4,
            int_max: 12,
        },
        ColumnSchema::Int {
            name: "TeleopL1Scored".to_string(),
            bits: 4,
            int_max: 12,
        },
        ColumnSchema::Int {
            name: "TeleopL2Attempted".to_string(),
            bits: 4,
            int_max: 12,
        },
        ColumnSchema::Int {
            name: "TeleopL2Scored".to_string(),
            bits: 4,
            int_max: 12,
        },
        ColumnSchema::Int {
            name: "TeleopL3Attempted".to_string(),
            bits: 4,
            int_max: 12,
        },
        ColumnSchema::Int {
            name: "TeleopL3Scored".to_string(),
            bits: 4,
            int_max: 12,
        },
        ColumnSchema::Int {
            name: "TeleopL4Attempted".to_string(),
            bits: 4,
            int_max: 12,
        },
        ColumnSchema::Int {
            name: "TeleopL4Scored".to_string(),
            bits: 4,
            int_max: 12,
        },
        ColumnSchema::Int {
            name: "TeleopBargeAttempted".to_string(),
            bits: 3,
            int_max: 7,
        },
        ColumnSchema::Int {
            name: "TeleopBargeScored".to_string(),
            bits: 3,
            int_max: 7,
        },
        ColumnSchema::Int {
            name: "TeleopProcessorAttempted".to_string(),
            bits: 3,
            int_max: 7,
        },
        ColumnSchema::Int {
            name: "TeleopProcessorScored".to_string(),
            bits: 3,
            int_max: 7,
        },
        ColumnSchema::Int {
            name: "TeleopAlgaeRemoved".to_string(),
            bits: 3,
            int_max: 7,
        },
        ColumnSchema::Int {
            name: "ClimbSuccessful".to_string(),
            bits: 1,
            int_max: 1,
        },
        ColumnSchema::Enum {
            name: "Climb".to_string(),
            bits: 2,
            values: vec!["None".to_string(), "Shallow".to_string(), "Deep".to_string(), "Park".to_string()],
        },
        ColumnSchema::Enum {
            name: "Breakdown".to_string(),
            bits: 1,
            values: vec!["False".to_string(), "True".to_string()],
        },
        ColumnSchema::Enum {
            name: "DefenseDescription".to_string(),
            bits: 0,
            values: vec!["".to_string()],
        },
        ColumnSchema::Enum {
            name: "Notes".to_string(),
            bits: 1,
            values: vec!["".to_string(), "Some note".to_string()],
        },
    ]
}

pub fn load_schema_from_json(path: &Path) -> Result<Vec<ColumnSchema>, anyhow::Error> {
    if !path.exists() {
        return Err(anyhow::anyhow!("Schema file not found: {}", path.display()));
    }

    let content = std::fs::read_to_string(path)?;
    let schema: Vec<ColumnSchema> = serde_json::from_str(&content)
        .map_err(|e| anyhow::anyhow!("Invalid JSON in schema file: {}", e))?;

    validate_schema(&schema)?;
    Ok(schema)
}

pub fn load_schema(schema_bytes: Option<&[u8]>) -> Result<Vec<ColumnSchema>, anyhow::Error> {
    match schema_bytes {
        None => Ok(get_default_schema()),
        Some(bytes) => {
            let schema: Vec<ColumnSchema> = serde_json::from_slice(bytes)
                .map_err(|e| anyhow::anyhow!("Invalid JSON in schema: {}", e))?;
            validate_schema(&schema)?;
            Ok(schema)
        }
    }
}

pub fn validate_schema(schema: &[ColumnSchema]) -> Result<(), anyhow::Error> {
    if schema.is_empty() {
        return Err(anyhow::anyhow!("Schema cannot be empty"));
    }

    let mut seen_names = HashSet::new();
    for col in schema.iter() {
        let name = col.name();
        if seen_names.contains(name) {
            return Err(anyhow::anyhow!("Duplicate column name in schema: {}", name));
        }
        seen_names.insert(name);

        match col {
            ColumnSchema::Int { bits, int_max, name, .. } => {
                if *bits > 0 {
                    let max_representable = (1u64 << bits) - 1;
                    if *int_max > max_representable {
                        return Err(anyhow::anyhow!(
                            "Column {}: int_max {} exceeds {}-bit capacity ({})",
                            name,
                            int_max,
                            bits,
                            max_representable
                        ));
                    }
                }
            }
            ColumnSchema::Enum { bits, values, name, .. } => {
                let count = values.len();
                if count > 1 {
                    let min_bits = (count as f64).log2().ceil() as u32;
                    if *bits < min_bits {
                        return Err(anyhow::anyhow!(
                            "Column {}: bits={} insufficient for {} enum values (need at least {})",
                            name,
                            bits,
                            count,
                            min_bits
                        ));
                    }
                }
            }
        }
    }

    Ok(())
}

