#[derive(Debug)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
        }
    }

    pub fn len(&self) -> usize {
        (self.width * self.height) as usize
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
    fn position_length() {
        let size = Size::new(3, 3);
        assert_eq!(9, size.len());
    }

}