pub const DATA_COLOR_SEQUENCE: &[(u8, u8, u8)] = &[
    (255, 0, 0),     // Red
    (255, 128, 0),   // Orange-red
    (255, 255, 0),   // Yellow
    (128, 255, 0),   // Yellow-green
    (0, 255, 0),     // Green
    (0, 255, 128),   // Cyan-green
    (0, 255, 255),   // Cyan
    (0, 128, 255),   // Blue-cyan
    (0, 0, 255),     // Blue
    (128, 0, 255),   // Purple-blue
    (255, 0, 255),   // Magenta
    (255, 0, 128),   // Pink
    (255, 64, 64),   // Light red
    (255, 192, 64),  // Orange
    (128, 64, 255),  // Purple
    (64, 128, 255),  // Light blue
];

pub const WHITE_COLOR: (u8, u8, u8) = (255, 255, 255);

pub fn get_default_palette_rgb() -> Vec<(u8, u8, u8)> {
    DATA_COLOR_SEQUENCE.to_vec()
}

