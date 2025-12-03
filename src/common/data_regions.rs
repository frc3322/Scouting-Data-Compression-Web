use crate::common::apriltag;

pub struct DataRegion {
    pub row_start: usize,
    pub row_end: usize,
    pub col_start: usize,
    pub col_end: usize,
}

pub fn get_data_regions(
    image_width: usize,
    image_height: usize,
    tag_data_gap: usize,
    data_padding: usize,
) -> Vec<DataRegion> {
    let tag_size = apriltag::get_april_tag_size();
    
    let mut regions = Vec::new();
    
    // Top region (between tag 0 and tag 1)
    regions.push(DataRegion {
        row_start: data_padding,
        row_end: data_padding + tag_size + tag_data_gap,
        col_start: data_padding + tag_size + tag_data_gap,
        col_end: image_width - data_padding - tag_size - tag_data_gap,
    });
    
    // Bottom region (extends to right edge)
    regions.push(DataRegion {
        row_start: image_height - data_padding - tag_size - tag_data_gap,
        row_end: image_height - data_padding,
        col_start: data_padding + tag_size + tag_data_gap,
        col_end: image_width - data_padding,
    });
    
    // Left region (between tag 0 and tag 2)
    regions.push(DataRegion {
        row_start: data_padding + tag_size + tag_data_gap,
        row_end: image_height - data_padding - tag_size - tag_data_gap,
        col_start: data_padding,
        col_end: data_padding + tag_size + tag_data_gap,
    });
    
    // Right region
    regions.push(DataRegion {
        row_start: data_padding + tag_size + tag_data_gap,
        row_end: image_height - data_padding - tag_size - tag_data_gap,
        col_start: image_width - data_padding - tag_size - tag_data_gap,
        col_end: image_width - data_padding,
    });
    
    // Center region
    regions.push(DataRegion {
        row_start: data_padding + tag_size + tag_data_gap,
        row_end: image_height - data_padding - tag_size - tag_data_gap,
        col_start: data_padding + tag_size + tag_data_gap,
        col_end: image_width - data_padding - tag_size - tag_data_gap,
    });
    
    regions.retain(|r| {
        let valid_row = r.row_start < r.row_end && r.row_start < image_height && r.row_end <= image_height;
        let valid_col = r.col_start < r.col_end && r.col_start < image_width && r.col_end <= image_width;
        valid_row && valid_col
    });
    
    regions
}

