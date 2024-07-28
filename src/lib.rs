#![crate_type = "lib"]
use std::*;

use io::Error;


pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub enum FileType{
    Delimited { delimiter: char },
    FixWidth { positions: Vec<u16>}
}

#[derive(Debug, Copy, Clone)]
pub struct CellContent{
    
}

#[derive(Debug, Clone)]
pub struct LineContent{
    pub cells : Vec<CellContent>
    
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
#[derive(Debug, Clone)]
// #[derive(Debug)]
pub struct FileInfo<'a>{
    pub path: &'a str,
    pub file_type: FileType,
    pub has_header: bool,
    pub header: Option<LineContent>
}
impl FileInfo{
    fn load(&mut self) -> Vec<LineContent>{
        let mut line_count = 0;
        let mut header_line = None::<LineContent>;
        let mut output = Vec::<LineContent>::new();
        let content = fs::read_to_string(* self.path);
        if let Error = content{
            panic!("Unable to read file at {}", * self.path);
        }
        for text in content.lines(){
            let mut cell_split = Vec::<&str>::new();
            match self.FileType{
                 FileType::Delimited(delim) => cellSplit = SplitDelimited(delim, text),
                 FileType::FixWidth(positions) => cellSplit = 
            };
            let line = LineContent::new(cellSplit);
            if line_count == 0 {
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
            lineCount = lineCount + 1;
            
        };
        output
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
    // Since positions is starting positions, our last column will need to be populated after this loop.
    for p in positions.iter().enumerate(){
        if p==lp && p == start {
            if start > 0{
               match colIndices.nth(start){
                  Some(s, _) => start = s,
                  None =>,
               };
            }
             
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
            None =>  vs.push(None),
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
    //Get content of last column in line, or None if we finished the string while going through previous columns
    loop{
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
    fn test_column_parse_positions(){
        let cols = [4, 9, 14, 21];
        let input = "Col1Start Col2Next to last column"
        let vs = SplitByPositions(positions, input );
    }
}
