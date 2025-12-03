use std::path::PathBuf;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <csv_path> [output_image_path] [packed_file_path] [--schema <schema_path>] [--palette <palette_path>]", args[0]);
        std::process::exit(1);
    }
    
    let csv_path = PathBuf::from(&args[1]);
    let output_image_path = args.get(2)
        .map(|s| PathBuf::from(s))
        .unwrap_or_else(|| csv_path.with_extension("png"));
    let packed_file_path = args.get(3)
        .map(|s| PathBuf::from(s))
        .unwrap_or_else(|| csv_path.with_extension("packed"));
    
    let mut schema_path = None;
    let mut palette_path = None;
    
    let mut i = 4;
    while i < args.len() {
        match args[i].as_str() {
            "--schema" => {
                if i + 1 < args.len() {
                    schema_path = Some(PathBuf::from(&args[i + 1]));
                    i += 2;
                } else {
                    eprintln!("Error: --schema requires a path");
                    std::process::exit(1);
                }
            }
            "--palette" => {
                if i + 1 < args.len() {
                    palette_path = Some(PathBuf::from(&args[i + 1]));
                    i += 2;
                } else {
                    eprintln!("Error: --palette requires a path");
                    std::process::exit(1);
                }
            }
            _ => {
                eprintln!("Unknown argument: {}", args[i]);
                std::process::exit(1);
            }
        }
    }
    
    if !csv_path.exists() {
        eprintln!("Error: CSV file not found: {}", csv_path.display());
        std::process::exit(1);
    }
    
    let csv_bytes = fs::read(&csv_path)?;
    let schema_bytes = schema_path.as_ref()
        .map(|p| fs::read(p))
        .transpose()?;
    let palette_bytes = palette_path.as_ref()
        .map(|p| fs::read(p))
        .transpose()?;
    
    let result = scouting_data_compression::api::encode_csv_to_image(
        &csv_bytes,
        schema_bytes.as_deref(),
        palette_bytes.as_deref(),
    )?;
    
    fs::write(&output_image_path, &result.image_bytes)?;
    println!("Encoded image saved to: {}", output_image_path.display());
    
    fs::write(&packed_file_path, &result.packed_data)?;
    println!("Packed data saved to: {}", packed_file_path.display());
    
    Ok(())
}

