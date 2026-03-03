# scouting-data-compression-wasm

Encode FRC scouting CSV data into visual images using AprilTags, zstandard compression, and color encoding. Designed for FIRST Robotics Competition match data.

See [Python Implementation](https://github.com/frc3322/Scouting-Data-Compression) for decoding (Python supports this).

## Installation

```bash
npm install scouting-data-compression-wasm
```

## Usage

```javascript
import init, { encode_csv_to_image } from 'scouting-data-compression-wasm';

await init();
const csvBytes = new TextEncoder().encode(csvText);
const imageBytes = encode_csv_to_image(csvBytes, null, null);
// imageBytes is a Uint8Array of PNG data
```

### React Example

```jsx
import React, { useState, useRef } from 'react';
import init, { encode_csv_to_image } from 'scouting-data-compression-wasm';

function ScoutingEncoder() {
    const [imageUrl, setImageUrl] = useState(null);
    const [loading, setLoading] = useState(false);
    const wasmInitialized = useRef(false);

    const handleEncode = async (e) => {
        e.preventDefault();
        const file = e.target.files?.[0];
        if (!file) return;

        setLoading(true);
        try {
            if (!wasmInitialized.current) {
                await init();
                wasmInitialized.current = true;
            }
            const csvText = await file.text();
            const imageBytes = encode_csv_to_image(
                new TextEncoder().encode(csvText),
                null,
                null
            );
            setImageUrl(URL.createObjectURL(new Blob([imageBytes], { type: 'image/png' })));
        } finally {
            setLoading(false);
        }
    };

    return (
        <div>
            <input type="file" accept=".csv" onChange={handleEncode} disabled={loading} />
            {imageUrl && <img src={imageUrl} alt="Encoded scouting data" />}
        </div>
    );
}
```

### Custom Schema and Palette

```javascript
const schemaBytes = new TextEncoder().encode(JSON.stringify([
    { name: "TeamNumber", kind: "int", int_max: 16383 },
    { name: "MatchResult", kind: "enum", values: ["Win", "Loss", "Tie", "DQ"] }
]));
const paletteBytes = new TextEncoder().encode(JSON.stringify([
    [255, 0, 0], [0, 255, 0], [0, 0, 255], [0, 0, 0]
]));

const imageBytes = encode_csv_to_image(csvBytes, schemaBytes, paletteBytes);
```

## API

### `encode_csv_to_image(csv, schema?, palette?) -> Uint8Array`

- **csv**: `Uint8Array` — CSV content as bytes
- **schema**: `Uint8Array | null` — Optional schema JSON bytes (default schema if null)
- **palette**: `Uint8Array | null` — Optional color palette JSON bytes (default palette if null)

Returns PNG image bytes as `Uint8Array`.

**Note:** Call `init()` once before any encode calls.

## Schema Format

**int columns** — provide either `bits` or `int_max` (not both):

- `bits` alone: `int_max` is derived as `(1 << bits) - 1`
- `int_max` alone: `bits` is derived as `ceil(log2(int_max + 1))`
- If both are given, the more restrictive is used (not recommended)

**enum columns** — `values` is required, `bits` is optional and not recommended:

- Omit `bits`: derived from `values.length` as `ceil(log2(count))`
- `bits` is supported but usually unnecessary; omit it to auto-size

```json
[
  { "name": "TeamNumber", "kind": "int", "int_max": 16383 },
  { "name": "MatchNumber", "kind": "int", "bits": 8 },
  { "name": "Result", "kind": "enum", "values": ["Win", "Loss", "Tie", "DQ"] }
]
```

## Palette Format

JSON array of RGB values: `[[r,g,b], ...]`
