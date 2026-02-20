# FRC Scouting Data Compression - Rust/WebAssembly

A Rust implementation of [https://github.com/frc3322/Scouting-Data-Compression](https://github.com/frc3322/Scouting-Data-Compression) with WebAssembly bindings for web frontend use. Encodes CSV scouting data into visual images using AprilTags, zstandard compression, and color encoding.

See [Python Implementation](https://github.com/frc3322/Scouting-Data-Compression) for decoding tags (only python supports this).

<img width="320" height="320" alt="MatchData_resized_8x" src="https://github.com/user-attachments/assets/b0144fe2-e985-48b7-9459-d29c621a7d37" />

## Features

- **Rust Implementation**: High-performance encoding written in Rust
- **WebAssembly Support**: Compile to WASM for use in web browsers
- **FRC Scouting Optimized**: Specifically designed for FIRST Robotics Competition match data encoding
- **High Compression**: Custom data packing and Zstandard compression significantly reduces scouting data size
- **Visual Data Transfer**: Convert structured CSV scouting data into images that can be captured by phones/tablets
- **Robust Detection**: Three AprilTag-based spatial reference ensures reliable data extraction
- **Color Encoding**: Pure RGB color palette optimized for camera capture

## Project Structure

```
Scouting-Data-Compression-Web/
├── Cargo.toml              # Workspace configuration
├── src/                    # Main Rust library
│   ├── lib.rs
│   ├── api.rs              # High-level encode API
│   ├── common/             # Shared utilities
│   │   ├── schema.rs       # Schema loading/validation
│   │   ├── constants.rs    # Default color sequence
│   │   ├── color_palette.rs # Palette loading/parsing
│   │   ├── data_regions.rs # Data region calculation
│   │   └── apriltag.rs     # AprilTag generation
│   └── encoder/            # Encoding components
│       ├── data_packer.rs   # CSV reading, bit packing, compression
│       ├── color_encoder.rs # Color pixel encoding
│       └── image_generator.rs # Image generation
├── wasm/                   # WebAssembly bindings
│   ├── Cargo.toml
│   └── src/lib.rs         # WASM exports
├── scripts/               # Helper scripts
│   ├── encode.sh          # Encode CSV to image
│   └── build-wasm.sh      # Build WASM package
└── examples/              # Example data files
```

## Installation

### Prerequisites

- **Rust 1.70+** — [rustup.rs](https://rustup.rs/)
- **wasm-pack** (for browser builds only): `cargo install wasm-pack`

---

### Installation for Command Line / Rust

```bash
git clone <repository-url>
cd Scouting-Data-Compression-Web
cargo build --release
```

---

### Installation for JavaScript / Browser Projects

This project has no npm package. You build the WASM binary and copy it into your project.

**1. Clone and build the WASM package:**

```bash
git clone <repository-url>
cd Scouting-Data-Compression-Web
./scripts/build-wasm.sh
```

This produces `wasm/pkg/` containing:

- `scouting_data_compression_wasm.js` — ES module loader
- `scouting_data_compression_wasm_bg.wasm` — WASM binary

**2. Copy `pkg` into your web project:**

| Project layout   | Copy from                    | Copy to         | Import path                          |
|------------------|-----------------------------|-----------------|--------------------------------------|
| Vanilla (no bundler) | `wasm/pkg/`              | `public/pkg/`   | `./pkg/scouting_data_compression_wasm.js` |
| Vite             | `wasm/pkg/`                 | `public/pkg/`   | `./pkg/scouting_data_compression_wasm.js` |
| Create React App | `wasm/pkg/`                 | `public/pkg/`   | `./pkg/scouting_data_compression_wasm.js` |

```bash
# Example: copy into a project with a public folder
cp -r wasm/pkg /path/to/your-project/public/
```

**3. Serve over HTTP.** ES modules and WASM require a web server (do not use `file://`). Use `npx serve public`, `python -m http.server`, or your framework’s dev server.

**Other setups:** If your project uses `static/`, `assets/`, or another folder, copy `pkg` there and adjust the import path accordingly. Most bundlers (Vite, webpack) handle `.wasm` automatically.

**4. Import and use:**

```javascript
import init, { encode_csv_to_image } from './pkg/scouting_data_compression_wasm.js';

await init();
const csvBytes = new TextEncoder().encode(csvText);
const imageBytes = encode_csv_to_image(csvBytes, null, null);
```

## Usage

### Command Line

Encode CSV data to an image:

```bash
# Basic usage
./scripts/encode.sh input.csv

# With custom output paths
./scripts/encode.sh input.csv output.png packed.packed

# With custom schema (must specify all 3 positional args before flags)
./scripts/encode.sh input.csv output.png packed.packed --schema schema.json

# With custom palette
./scripts/encode.sh input.csv output.png packed.packed --palette palette.json
```

Or use cargo directly:

```bash
cargo run --bin encode -- input.csv [output.png] [packed.packed] [--schema schema.json] [--palette palette.json]
# When using --schema or --palette, provide all 3 positional args first.
```

### Programmatic API

```rust
use scouting_data_compression::api;

let csv_bytes = std::fs::read("data.csv")?;
let schema_bytes = Some(std::fs::read("schema.json")?);
let palette_bytes = None; // Use default palette

let result = api::encode_csv_to_image(
    &csv_bytes,
    schema_bytes.as_deref(),
    palette_bytes.as_deref(),
)?;

std::fs::write("output.png", &result.image_bytes)?;
std::fs::write("output.packed", &result.packed_data)?;
```

### WebAssembly (Browser)

After installing (see [Installation for JavaScript / Browser Projects](#installation-for-javascript--browser-projects)), import the WASM module and use it.

**Vanilla JavaScript Example:**

```html
<!DOCTYPE html>
<html>
<head>
    <title>Scouting Data Encoder</title>
</head>
<body>
    <input type="file" id="csvFile" accept=".csv" />
    <button id="encodeBtn">Encode to Image</button>
    <img id="output" style="display: none;" />
    
    <script type="module">
        import init, { encode_csv_to_image } from './pkg/scouting_data_compression_wasm.js';
        
        let wasmInitialized = false;
        
        async function initializeWasm() {
            if (!wasmInitialized) {
                await init();
                wasmInitialized = true;
            }
        }
        
        async function encodeCSV(csvText, schemaJson = null, paletteJson = null) {
            await initializeWasm();
            
            try {
                const csvBytes = new TextEncoder().encode(csvText);
                const schemaBytes = schemaJson 
                    ? new TextEncoder().encode(JSON.stringify(schemaJson)) 
                    : null;
                const paletteBytes = paletteJson 
                    ? new TextEncoder().encode(JSON.stringify(paletteJson)) 
                    : null;
                
                const imageBytes = encode_csv_to_image(
                    csvBytes,
                    schemaBytes ? new Uint8Array(schemaBytes) : null,
                    paletteBytes ? new Uint8Array(paletteBytes) : null
                );
                
                return imageBytes;
            } catch (error) {
                console.error('Encoding error:', error);
                throw new Error(`Failed to encode CSV: ${error.message}`);
            }
        }
        
        document.getElementById('encodeBtn').addEventListener('click', async () => {
            const fileInput = document.getElementById('csvFile');
            const file = fileInput.files[0];
            
            if (!file) {
                alert('Please select a CSV file');
                return;
            }
            
            try {
                const csvText = await file.text();
                const imageBytes = await encodeCSV(csvText);
                
                const blob = new Blob([imageBytes], { type: 'image/png' });
                const url = URL.createObjectURL(blob);
                
                const img = document.getElementById('output');
                img.src = url;
                img.style.display = 'block';
                
                const a = document.createElement('a');
                a.href = url;
                a.download = 'encoded.png';
                a.click();
                
                URL.revokeObjectURL(url);
            } catch (error) {
                alert(`Error: ${error.message}`);
            }
        });
    </script>
</body>
</html>
```

#### React Example

```jsx
import React, { useState, useRef } from 'react';
import init, { encode_csv_to_image } from './pkg/scouting_data_compression_wasm.js';

function ScoutingEncoder() {
    const [imageUrl, setImageUrl] = useState(null);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState(null);
    const fileInputRef = useRef(null);
    const wasmInitialized = useRef(false);
    
    const initializeWasm = async () => {
        if (!wasmInitialized.current) {
            await init();
            wasmInitialized.current = true;
        }
    };
    
    const handleEncode = async (event) => {
        event.preventDefault();
        const file = fileInputRef.current?.files[0];
        
        if (!file) {
            setError('Please select a CSV file');
            return;
        }
        
        setLoading(true);
        setError(null);
        
        try {
            await initializeWasm();
            
            const csvText = await file.text();
            const csvBytes = new TextEncoder().encode(csvText);
            
            const imageBytes = encode_csv_to_image(
                csvBytes,
                null, // Use default schema
                null  // Use default palette
            );
            
            const blob = new Blob([imageBytes], { type: 'image/png' });
            const url = URL.createObjectURL(blob);
            setImageUrl(url);
        } catch (err) {
            setError(`Encoding failed: ${err.message}`);
        } finally {
            setLoading(false);
        }
    };
    
    const handleDownload = () => {
        if (imageUrl) {
            const a = document.createElement('a');
            a.href = imageUrl;
            a.download = 'scouting-data.png';
            a.click();
        }
    };
    
    return (
        <div>
            <form onSubmit={handleEncode}>
                <input 
                    type="file" 
                    ref={fileInputRef} 
                    accept=".csv" 
                    required 
                />
                <button type="submit" disabled={loading}>
                    {loading ? 'Encoding...' : 'Encode to Image'}
                </button>
            </form>
            
            {error && <div style={{ color: 'red' }}>{error}</div>}
            
            {imageUrl && (
                <div>
                    <img src={imageUrl} alt="Encoded scouting data" />
                    <button onClick={handleDownload}>Download Image</button>
                </div>
            )}
        </div>
    );
}

export default ScoutingEncoder;
```

#### Advanced: Custom Schema and Palette

```javascript
import init, { encode_csv_to_image } from './pkg/scouting_data_compression_wasm.js';

async function encodeWithCustomSchema(csvText, schemaJson, paletteJson) {
    await init();
    
    const csvBytes = new TextEncoder().encode(csvText);
    const schemaBytes = new TextEncoder().encode(JSON.stringify(schemaJson));
    const paletteBytes = new TextEncoder().encode(JSON.stringify(paletteJson));
    
    try {
        const imageBytes = encode_csv_to_image(
            csvBytes,
            new Uint8Array(schemaBytes),
            new Uint8Array(paletteBytes)
        );
        
        return imageBytes;
    } catch (error) {
        throw new Error(`Encoding failed: ${error.message}`);
    }
}

const customSchema = [
    {
        name: "TeamNumber",
        kind: "int",
        bits: 14,
        int_max: 16383
    },
    {
        name: "MatchResult",
        kind: "enum",
        bits: 2,
        values: ["Win", "Loss", "Tie", "DQ"]
    }
];

const customPalette = [
    [255, 0, 0],   // Red
    [0, 255, 0],   // Green
    [0, 0, 255],   // Blue
    [255, 255, 0], // Yellow
    [255, 0, 255], // Magenta
    [0, 255, 255], // Cyan
    [128, 128, 128], // Gray
    [0, 0, 0]      // Black
];

encodeWithCustomSchema(csvText, customSchema, customPalette)
    .then(imageBytes => {
        const blob = new Blob([imageBytes], { type: 'image/png' });
        const url = URL.createObjectURL(blob);
        // Use the URL for display or download
    })
    .catch(error => {
        console.error('Error:', error);
    });
```

#### File Upload with Drag & Drop

```html
<div id="dropZone" style="border: 2px dashed #ccc; padding: 20px; text-align: center;">
    <p>Drag and drop CSV file here or click to select</p>
    <input type="file" id="fileInput" accept=".csv" style="display: none;" />
</div>

<script type="module">
    import init, { encode_csv_to_image } from './pkg/scouting_data_compression_wasm.js';
    
    await init();
    
    const dropZone = document.getElementById('dropZone');
    const fileInput = document.getElementById('fileInput');
    
    dropZone.addEventListener('click', () => fileInput.click());
    
    dropZone.addEventListener('dragover', (e) => {
        e.preventDefault();
        dropZone.style.borderColor = '#007bff';
    });
    
    dropZone.addEventListener('dragleave', () => {
        dropZone.style.borderColor = '#ccc';
    });
    
    dropZone.addEventListener('drop', async (e) => {
        e.preventDefault();
        dropZone.style.borderColor = '#ccc';
        
        const file = e.dataTransfer.files[0];
        if (file && file.name.endsWith('.csv')) {
            await processFile(file);
        }
    });
    
    fileInput.addEventListener('change', async (e) => {
        const file = e.target.files[0];
        if (file) {
            await processFile(file);
        }
    });
    
    async function processFile(file) {
        try {
            const csvText = await file.text();
            const csvBytes = new TextEncoder().encode(csvText);
            
            const imageBytes = encode_csv_to_image(csvBytes, null, null);
            
            const blob = new Blob([imageBytes], { type: 'image/png' });
            const url = URL.createObjectURL(blob);
            
            const img = document.createElement('img');
            img.src = url;
            img.style.maxWidth = '100%';
            document.body.appendChild(img);
            
            const downloadLink = document.createElement('a');
            downloadLink.href = url;
            downloadLink.download = file.name.replace('.csv', '.png');
            downloadLink.textContent = 'Download Image';
            document.body.appendChild(downloadLink);
        } catch (error) {
            alert(`Error: ${error.message}`);
        }
    }
</script>
```

#### Error Handling Best Practices

```javascript
import init, { encode_csv_to_image } from './pkg/scouting_data_compression_wasm.js';

async function encodeWithErrorHandling(csvText, schemaJson = null, paletteJson = null) {
    try {
        await init();
    } catch (error) {
        throw new Error(`Failed to initialize WASM: ${error.message}`);
    }
    
    if (!csvText || csvText.trim().length === 0) {
        throw new Error('CSV content cannot be empty');
    }
    
    try {
        const csvBytes = new TextEncoder().encode(csvText);
        const schemaBytes = schemaJson 
            ? new TextEncoder().encode(JSON.stringify(schemaJson)) 
            : null;
        const paletteBytes = paletteJson 
            ? new TextEncoder().encode(JSON.stringify(paletteJson)) 
            : null;
        
        const imageBytes = encode_csv_to_image(
            csvBytes,
            schemaBytes ? new Uint8Array(schemaBytes) : null,
            paletteBytes ? new Uint8Array(paletteBytes) : null
        );
        
        if (!imageBytes || imageBytes.length === 0) {
            throw new Error('Encoding produced empty result');
        }
        
        return imageBytes;
    } catch (error) {
        if (error.message.includes('schema')) {
            throw new Error(`Schema validation error: ${error.message}`);
        } else if (error.message.includes('palette')) {
            throw new Error(`Palette error: ${error.message}`);
        } else if (error.message.includes('CSV')) {
            throw new Error(`CSV parsing error: ${error.message}`);
        } else {
            throw new Error(`Encoding error: ${error.message}`);
        }
    }
}
```

#### Web Worker Example (for Large Files)

```javascript
// worker.js
import init, { encode_csv_to_image } from './pkg/scouting_data_compression_wasm.js';

let wasmInitialized = false;

self.onmessage = async function(e) {
    const { csvText, schemaJson, paletteJson } = e.data;
    
    try {
        if (!wasmInitialized) {
            await init();
            wasmInitialized = true;
        }
        
        const csvBytes = new TextEncoder().encode(csvText);
        const schemaBytes = schemaJson 
            ? new TextEncoder().encode(JSON.stringify(schemaJson)) 
            : null;
        const paletteBytes = paletteJson 
            ? new TextEncoder().encode(JSON.stringify(paletteJson)) 
            : null;
        
        const imageBytes = encode_csv_to_image(
            csvBytes,
            schemaBytes ? new Uint8Array(schemaBytes) : null,
            paletteBytes ? new Uint8Array(paletteBytes) : null
        );
        
        self.postMessage({ success: true, imageBytes });
    } catch (error) {
        self.postMessage({ success: false, error: error.message });
    }
};

// main.js
const worker = new Worker('./worker.js', { type: 'module' });

function encodeInWorker(csvText, schemaJson = null, paletteJson = null) {
    return new Promise((resolve, reject) => {
        worker.onmessage = (e) => {
            if (e.data.success) {
                resolve(e.data.imageBytes);
            } else {
                reject(new Error(e.data.error));
            }
        };
        
        worker.postMessage({ csvText, schemaJson, paletteJson });
    });
}
```

#### Integration Tips

1. **Initialize WASM Once**: Call `init()` once at application startup, not on every encode call
2. **Memory Management**: Large images are automatically managed by WASM, but clean up object URLs with `URL.revokeObjectURL()`
3. **Error Boundaries**: Wrap encoding calls in try-catch blocks for production apps
4. **Loading States**: Show loading indicators during encoding, especially for large CSV files
5. **File Validation**: Validate CSV format before encoding to provide better error messages
6. **Progressive Enhancement**: Provide fallback options if WASM fails to load

## API Reference

### Rust: `encode_csv_to_image(csv_bytes, schema_bytes, palette_bytes) -> Result<EncodeResult>`

Encode CSV data into an image with AprilTags.

**Parameters:**
- `csv_bytes`: CSV file content as bytes
- `schema_bytes`: Optional schema JSON bytes. If None, uses default schema.
- `palette_bytes`: Optional color palette JSON bytes. If None, uses default palette.

**Returns:** `EncodeResult` containing:
- `image_bytes`: PNG image bytes
- `packed_data`: Packed binary data (for compatibility with Python decoder)

**Errors:**
- Returns `anyhow::Error` for file I/O errors, schema validation errors, or encoding failures.

### WebAssembly: `encode_csv_to_image(csv, schema, palette) -> Uint8Array`

Same parameters. Returns PNG image bytes directly (packed_data not exposed in WASM).

## Schema Format

Schemas define the structure of CSV data. See `examples/schema.json` for format:

```json
[
  {
    "name": "TeamNumber",
    "kind": "int",
    "bits": 14,
    "int_max": 16383
  },
  {
    "name": "Result",
    "kind": "enum",
    "bits": 2,
    "values": ["Win", "Loss", "Tie"]
  }
]
```

## Color Palette Format

Color palettes are JSON arrays of RGB values:

```json
[
  [0, 0, 255],
  [0, 255, 0],
  [255, 0, 0],
  [0, 0, 0]
]
```

The system will use the largest power-of-two subset of colors for encoding.

## Compatibility

- Generated `.packed` files are compatible with the Python decoder
- Generated PNG images are compatible with the Python decoder
- Schema JSON format matches the Python version

## Performance

Rust implementation provides significant performance improvements over Python:
- Faster CSV parsing
- Faster image generation
- Smaller WASM binary size (optimized for web)

## Development

### Running Tests

```bash
cargo test
```

### Building for Release

```bash
cargo build --release
```

### Building WASM

```bash
cd wasm
wasm-pack build --target web --out-dir pkg
```

## License

MIT License - see LICENSE file for details.

## Acknowledgments

- [AprilTags](https://april.eecs.umich.edu/software/apriltag/) for visual fiducial markers
- [Zstandard](https://facebook.github.io/zstd/) for high-performance compression
- Python implementation (for decoding mostly): `https://github.com/frc3322/Scouting-Data-Compression`

