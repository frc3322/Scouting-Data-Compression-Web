use crate::common::schema::ColumnSchema;
use csv::ReaderBuilder;
use std::collections::HashMap;

pub fn read_csv(csv_bytes: &[u8]) -> Result<(Vec<String>, Vec<Vec<String>>), anyhow::Error> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(csv_bytes);
    
    let headers = reader.headers()?.iter().map(|s| s.to_string()).collect();
    
    let mut rows = Vec::new();
    for result in reader.records() {
        let record = result?;
        rows.push(record.iter().map(|s| s.to_string()).collect());
    }
    
    if rows.is_empty() {
        return Err(anyhow::anyhow!("CSV is empty"));
    }
    
    Ok((headers, rows))
}

pub fn pack_columnar_bitplanes(
    values_by_col: &[Vec<u64>],
    bits_by_col: &[u32],
) -> Vec<u8> {
    let mut out = Vec::new();
    
    for (vals, &bits) in values_by_col.iter().zip(bits_by_col.iter()) {
        if bits == 0 {
            continue;
        }
        
        for b in (0..bits).rev() {
            let mut acc = 0u8;
            let mut nbits = 0;
            
            for &v in vals {
                let bit = ((v >> b) & 1) as u8;
                acc = (acc << 1) | bit;
                nbits += 1;
                
                if nbits == 8 {
                    out.push(acc);
                    acc = 0;
                    nbits = 0;
                }
            }
            
            if nbits > 0 {
                out.push(acc << (8 - nbits));
            }
        }
    }
    
    out
}

pub fn encode(
    headers: &[String],
    rows: &[Vec<String>],
    schema: Option<&[ColumnSchema]>,
) -> Result<Vec<u8>, anyhow::Error> {
    let schema_to_use = match schema {
        Some(s) => s,
        None => {
            let default = crate::common::schema::get_default_schema();
            return encode(headers, rows, Some(&default));
        }
    };
    
    crate::common::schema::validate_schema(schema_to_use)?;
    
    let schema_names: Vec<&str> = schema_to_use.iter().map(|s| s.name()).collect();
    let header_to_csv_idx: HashMap<&str, usize> = headers
        .iter()
        .enumerate()
        .map(|(idx, name)| (name.as_str(), idx))
        .collect();
    
    let missing_columns: Vec<&str> = schema_names
        .iter()
        .filter(|name| !header_to_csv_idx.contains_key(*name))
        .copied()
        .collect();
    
    if !missing_columns.is_empty() {
        return Err(anyhow::anyhow!(
            "CSV missing required columns from schema: {:?}",
            missing_columns
        ));
    }
    
    let num_rows = rows.len();
    
    let mut enum_lookups: Vec<Option<HashMap<String, u64>>> = Vec::new();
    for col in schema_to_use {
        match col {
            ColumnSchema::Enum { values, .. } => {
                let lookup: HashMap<String, u64> = values
                    .iter()
                    .enumerate()
                    .map(|(i, v)| (v.clone(), i as u64))
                    .collect();
                enum_lookups.push(Some(lookup));
            }
            ColumnSchema::Int { .. } => {
                enum_lookups.push(None);
            }
        }
    }
    
    let mut values_by_col: Vec<Vec<u64>> = Vec::new();
    let mut bits_by_col: Vec<u32> = Vec::new();
    
    for (col_idx, col) in schema_to_use.iter().enumerate() {
        if col.bits() == 0 {
            continue;
        }
        
        let csv_col_idx = header_to_csv_idx[col.name()];
        let mut col_vals: Vec<u64> = Vec::new();
        
        for row in rows {
            if csv_col_idx >= row.len() {
                return Err(anyhow::anyhow!(
                    "Row has fewer columns than expected. Column {} (index {}) not found in row",
                    col.name(),
                    csv_col_idx
                ));
            }
            
            let raw = row[csv_col_idx].trim().replace('\n', " ").replace('\r', "");
            let raw = if raw == " " { "" } else { raw.as_str() };
            
            let value = match col {
                ColumnSchema::Int { int_max, name, .. } => {
                    let val: u64 = raw.to_string().parse()
                        .map_err(|_| anyhow::anyhow!("Invalid integer value '{}' for column {}", raw, name))?;
                    if val > *int_max {
                        return Err(anyhow::anyhow!(
                            "Value {} exceeds int_max {} for column {}",
                            val,
                            int_max,
                            name
                        ));
                    }
                    val
                }
                ColumnSchema::Enum { name, .. } => {
                    let lookup = enum_lookups[col_idx].as_ref().unwrap();
                    *lookup.get(raw)
                        .ok_or_else(|| anyhow::anyhow!(
                            "Value '{}' not in enum values for column {}",
                            raw,
                            name
                        ))?
                }
            };
            
            col_vals.push(value);
        }
        
        values_by_col.push(col_vals);
        bits_by_col.push(col.bits());
    }
    
    let data_bytes = pack_columnar_bitplanes(&values_by_col, &bits_by_col);
    
    let compressed_data = zstd::encode_all(data_bytes.as_slice(), 22)?;
    
    let mut packed = Vec::new();
    packed.extend_from_slice(b"SCOUTPK5");
    packed.extend_from_slice(&(num_rows as u32).to_be_bytes());
    packed.extend_from_slice(&compressed_data);
    
    Ok(packed)
}

