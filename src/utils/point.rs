#[derive(Eq, PartialEq, Debug, Hash, Clone, Copy)]
pub struct Point {
    pub row: i32,
    pub col: i32,
    pub val: i32,
}

impl Point {
    pub fn from(row: i32, col: i32, grid: &[String]) -> Option<Self> {
        if row >= 0_i32 && row < grid.len() as i32 && col >= 0_i32 && col < grid.len() as i32 {
            grid.get(row as usize)
                .and_then(|r| r.chars().nth(col as usize))
                .and_then(|ch| ch.to_digit(10))
                .and_then(|digit| {
                    Some(Self {
                        row: row as i32,
                        col: col as i32,
                        val: digit as i32,
                    })
                })
        } else {
            None
        }
    }
}
