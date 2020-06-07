#[derive(Debug, Clone, PartialEq)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    /// Returns a size
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// Returns the length (width * height)
    pub fn len(&self) -> usize {
        (self.width * self.height) as usize
    }

    /// Checks if the size is empty
    pub fn is_empty(&self) -> bool {
        (self.width * self.height) == 0
    }
}

#[cfg(test)]
mod tests {
    use tile::size::Size;

    #[test]
    fn position_new() {
        let size = Size::new(9, 5);
        assert_eq!(9, size.width);
        assert_eq!(5, size.height);
    }

    #[test]
    fn position_len() {
        let size = Size::new(3, 3);
        assert_eq!(9, size.len());
    }

    #[test]
    fn position_is_empty() {
        let size = Size::new(0, 0);
        assert_eq!(true, size.is_empty());
    }
}
