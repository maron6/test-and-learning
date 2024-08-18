#![crate_type = "lib"]
use std::*;

use io::Error;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
#[derive(Debug, Clone)]
pub enum FileType {
    Delimited { delimiter: char },
    FixWidth { positions: Vec<u16> },
}

#[derive(Debug, Copy, Clone)]
pub struct CellContent {}

#[derive(Debug, Clone)]
pub struct LineContent {
    pub cells: Vec<CellContent>,
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
pub struct FileInfo<'a> {
    pub path: &'a str,
    pub file_type: FileType,
    pub has_header: bool,
    pub header: Option<LineContent>,
}
impl<'a> FileInfo<'a> {
    fn load(&mut self) -> Vec<LineContent> {
        let mut line_count = 0;
        let mut header_line = None::<LineContent>;
        let mut output = Vec::<LineContent>::new();
        let file_result = fs::read_to_string(&self.path);
        let content = match file_result {
            Ok(content) => content,
            Err(e) => panic!("Unable to read file at {}", &self.path),
        };
        // ToDo: get content as string instead of Result
        for text in content.lines() {
            let mut cell_split = match self.file_type {
                FileType::Delimited { delimiter } => split_delimited(delimiter, text),
                FileType::FixWidth { positions } => split_by_positions(positions, text),
            };
            let line = LineContent::new(cell_split);
            if line_count == 0 {
                if self.has_header {
                    self.header = Some(line);
                } else {
                    self.header = None;
                    output.push(line);
                }
            } else {
                output.push(line);
            }
            line_count = line_count + 1;
        }
        output
    }
}
fn split_delimited(delim: char, src: &str) -> Vec<Option<&str>> {}
//Might need to change this to Vec<Option<string>> ? Need to research a bit more and practice with borrow/ownership of string content
fn split_by_positions(positions: Vec<u16>, src: &str) -> Vec<Option<&str>> {
    let mut lp = 0u16;
    let mut col_count = positions.len();
    // if positions[colCount] == src.len(){
    //    colCount = colCount - 1;
    // }
    assert_ne!(0, col_count);
    let mut col_indices = src.char_indices();
    let mut start = positions[0]; // 0usize;
    let mut str_end: usize = 0;
    let mut vs = Vec::<Option<&str>>::with_capacity(col_count);
    // Since positions is starting positions, our last column will need to be populated after this loop.
    for pos_iter in positions.iter().enumerate() {
        let (pos, _) = pos_iter;
        let p = pos as u16;
        if p == lp && p == start {
            if start > 0 {
                if let Some(s) = col_indices.nth(start as usize) {
                    start = s.0 as u16;
                }
            }
            continue; // Columns positions are start of column, rather than end of column.
        }
        let ending_index = u16::from(p) - lp;

        match col_indices.nth(usize::from(ending_index)) {
            Some(colEnd) => {
                let st = usize::from(start);
                let (end, _) = colEnd;
                let s = &src[st..end];
                vs.push(Some(&s));
                start = end as u16; // Note: should be exclusive ending position, so should not need to modify before assigning to start
            }
            None => vs.push(None),
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
    loop {
        match col_indices.next() {
            Some(colEnd) => str_end = colEnd.0,
            None => break,
        };
    }

    if str_end > start as usize {
        let last = &src[start as usize..str_end];
        vs.push(Some(&last));
    } else {
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
    fn test_column_parse_positions() {
        let cols = vec![4u16, 9u16, 14u16, 21u16];
        let input = "Col1Start Col2Next to last column";
        let vs = split_by_positions(cols, input);
    }
}
