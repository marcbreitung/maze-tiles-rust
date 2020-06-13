pub struct Size {
    pub rows: u32,
    pub columns: u32,
}

impl Size {
    pub fn new(columns: u32, rows: u32) -> Self {
        Self { columns, rows }
    }
}

#[cfg(test)]
mod tests {
    use size::Size;

    #[test]
    fn position_new() {
        let size = Size::new(9, 5);
        assert_eq!(9, size.columns);
        assert_eq!(5, size.rows);
    }
}