pub fn i_u(value: i32) -> Option<usize> {
    if value >= 0 {
        Some(value as usize)
    } else {
        None
    }
}
