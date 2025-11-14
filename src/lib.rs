pub fn add(x: usize, y: usize) -> usize {
    x + y
}

#[cfg(test)]
mod tests {
    use crate::add;

    #[test]
    fn testing() {
        assert_eq!(4, add(1, 3));
    }
}
