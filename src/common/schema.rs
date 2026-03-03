use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::Path;

#[derive(Deserialize)]
#[serde(tag = "kind")]
enum RawColumnSchema {
    #[serde(rename = "int")]
    Int {
        name: String,
        bits: Option<u32>,
        int_max: Option<u64>,
    },
    #[serde(rename = "enum")]
    Enum {
        name: String,
        bits: Option<u32>,
        values: Vec<String>,
    },
}

fn resolve_raw_schema(
    raw: Vec<RawColumnSchema>,
    warnings: &mut Vec<String>,
) -> Result<Vec<ColumnSchema>, anyhow::Error> {
    let mut resolved = Vec::with_capacity(raw.len());
    for col in raw {
        match col {
            RawColumnSchema::Int { name, bits, int_max } => {
                let (resolved_bits, resolved_int_max) = match (bits, int_max) {
                    (None, None) => {
                        return Err(anyhow::anyhow!(
                            "int column '{}': must provide 'bits' or 'int_max'",
                            name
                        ));
                    }
                    (Some(b), Some(m)) => {
                        let bits_needed = if m == 0 {
                            0u32
                        } else {
                            ((m as f64 + 1.0).log2().ceil()) as u32
                        };
                        let effective_bits = b.min(bits_needed);
                        let effective_int_max = m.min(if effective_bits > 0 {
                            (1u64 << effective_bits) - 1
                        } else {
                            0
                        });
                        warnings.push(format!(
                            "Column '{}': both 'bits' and 'int_max' provided; \
                             using bits={}, int_max={}",
                            name, effective_bits, effective_int_max
                        ));
                        (effective_bits, effective_int_max)
                    }
                    (Some(b), None) => {
                        let m = if b > 0 { (1u64 << b) - 1 } else { 0 };
                        (b, m)
                    }
                    (None, Some(m)) => {
                        let b = if m == 0 {
                            0u32
                        } else {
                            ((m as f64 + 1.0).log2().ceil()) as u32
                        };
                        (b, m)
                    }
                };
                resolved.push(ColumnSchema::Int {
                    name,
                    bits: resolved_bits,
                    int_max: resolved_int_max,
                });
            }
            RawColumnSchema::Enum { name, bits, values } => {
                let count = values.len();
                let bits_needed = if count <= 1 {
                    0u32
                } else {
                    (count as f64).log2().ceil() as u32
                };
                let resolved_bits = match bits {
                    Some(b) => {
                        let effective_bits = b.min(bits_needed);
                        if b != effective_bits {
                            warnings.push(format!(
                                "Column '{}': 'bits' provided with 'values'; using bits={}",
                                name, effective_bits
                            ));
                        }
                        effective_bits
                    }
                    None => bits_needed,
                };
                resolved.push(ColumnSchema::Enum {
                    name,
                    bits: resolved_bits,
                    values,
                });
            }
        }
    }
    Ok(resolved)
}

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
    let raw: Vec<RawColumnSchema> = serde_json::from_str(&content)
        .map_err(|e| anyhow::anyhow!("Invalid JSON in schema file: {}", e))?;

    let mut warnings = Vec::new();
    let schema = resolve_raw_schema(raw, &mut warnings)?;
    for w in &warnings {
        eprintln!("[schema warning] {}", w);
    }
    validate_schema(&schema)?;
    Ok(schema)
}

pub fn load_schema(schema_bytes: Option<&[u8]>) -> Result<Vec<ColumnSchema>, anyhow::Error> {
    let (schema, warnings) = load_schema_with_warnings(schema_bytes)?;
    for w in &warnings {
        eprintln!("[schema warning] {}", w);
    }
    Ok(schema)
}

/// Like `load_schema` but returns resolution warnings instead of printing them.
pub fn load_schema_with_warnings(
    schema_bytes: Option<&[u8]>,
) -> Result<(Vec<ColumnSchema>, Vec<String>), anyhow::Error> {
    match schema_bytes {
        None => Ok((get_default_schema(), Vec::new())),
        Some(bytes) => {
            let raw: Vec<RawColumnSchema> = serde_json::from_slice(bytes)
                .map_err(|e| anyhow::anyhow!("Invalid JSON in schema: {}", e))?;
            let mut warnings = Vec::new();
            let schema = resolve_raw_schema(raw, &mut warnings)?;
            validate_schema(&schema)?;
            Ok((schema, warnings))
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

