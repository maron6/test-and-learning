#![crate_type = "lib"]
use std::env;
use std::fs;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub enum FileType{
    Delimited(delimited: char),
    FixWidth(positions: Vec<u16>)
}

pub struct CellContent{
    
}
pub struct LineContent{
    pub Cells : Vec<CellContent>
    
}
/*
ToDo:
 *  parse string content of file.
 * Split based on line ending
 * For each line:
     *  create a cell entry for each column, based on headers
     * Support modifying a cell's content, and modifying some tracker on the FileInfo? 
         Could potentially just flag the FileInfo as modified as an indication that we have anything to write
  */
#[derive(Debug, Copy, Clone)]
// #[derive(Debug)]
pub struct FileInfo{
    pub Path: &str,
    pub FileType: FileType,
    pub HasHeader: bool,
    pub Header: Option<LineContent>
}
impl FileInfo{
    fn Load(&mut self) -> Vec<LineContent>{
        let mut lineCount = 0;
        let mut headerLine = None(LineContent);
        let mut output = Vec::<LineContent>::new();
        let content = fs::read_to_string(self.Path);
        assert_ne!(Err, content, "Unable to read file");
        for text in content.lines(){
            let mut Vec<&str> cellSplit;
            match self.FileType{
                 FileType.Delimited(delim) => cellSplit = SplitDelimited(),
                 FileType.FixWidth(positions) => cellSplit = 
            };
            let line = LineContent::new(cellSplit);
            if lineContent == 0 {
                if self.HasHeader{
                    self.Header = Some(line);
                }
                else{
                    self.Header = None;
                    output.push(line);
                }    
            }
            else{
                output.push(line);
            }
            lineCount ++;
            
        }
    }
}
fn SplitDelimited(delim: char, src: &str) -> Vec<Option<&str>>{
    
}
//Might need to change this to Vec<Option<string>> ? Need to research a bit more and practice with borrow/ownership of string content
fn SplitByPositions(positions: vec<u16>, src: &str) -> Vec<Option<&str>>{
    let mut lp = 0u16;
    let mut colCount = positions.len();
    // if positions[colCount] == src.len(){
    //    colCount = colCount - 1;
    // }
    assert_ne!(0, colCount);
    let mut colIndices = src.char_indices();
    let mut start = positions[0];// 0usize;
    let mut strEnd;
    let vs = Vec::<&str>::with_capacity(colCount);
    for p in positions.iter().enumerate(){
        if p==lp && p == start {
            continue;// Columns positions are start of column, rather than end of column.
        }
        let endingIndex = p - lp;

        match colIndices.nth(endingIndex){
            Some(colEnd,_) => {
                let st = usize::from(start);
                let s = &src[st..colEnd];
                vs.push(Some(&s));
                start = colEnd; // Note: should be exclusive ending position, so should not need to modify before assigning to start
            },
            None =>  vs.push(None);
        };
        /*example: Field1   Field2 Field3  (EOL)
        Start at 0, then we want to look for the position of F in Field2 (10,  field 1 is len 9)
        File1: 0..10
        For field2, we would want 10..18 (exclusive on right side)
        For Field3, we would want 18..26 (Ignore EOL)
        What if we want to throw out some beginning space consistently? Have first column start later
        E.g. 
    Field1  Field2  Field3  '
        4 = Start, so:
        4..12
        12..20
        20..
        
        */
        lp = p;
    }
    //Get last index of line, if any are remaining
    while{
        match colIndices.next(){
            Some(colEnd, _) => strEnd = colEnd,
            None=> break,
        };
    };
    if strEnd > start{
        let last =  &src[start..strEnd];
        vs.push(Some(&last));   
    }   
    else{
        vs.push(None);
    }
    vs
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
