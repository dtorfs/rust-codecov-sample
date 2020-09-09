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

}
