pub fn covered(i: u32) -> u32  {
    i * 2
}

pub fn uncovered(i: u32) -> u32  {
    i * 3 
}

pub fn another_uncovered(i: u32) -> u32  {
    i * 5 
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_covered() {
        assert_eq!(covered(1), 2);
    }
    #[test]
    fn test_uncovered() {
        assert_eq!(uncovered(1), 3);
    }
    #[test]
    fn test_another_uncovered() {
        assert_eq!(another_uncovered(1), 5);
    }

}
