
pub fn get_center(height: u32, width: u32) -> usize {
    ((width * height) / 2) as usize
}

pub fn get_center_u32(height: u32, width: u32) -> u32 {
    (width * height) / 2
}
