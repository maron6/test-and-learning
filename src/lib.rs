#![crate_type = "lib"]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub enum FileType{
    Delimited(delimited: char),
    FixWidth(positions: Vec<u16>)
}

#[derive(Debug, Copy, Clone)]
// #[derive(Debug)]
pub struct FileInfo{
    pub Path: &str,
    pub FileType: fileType,
    pub HasHeader: bool
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
