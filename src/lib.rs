#![crate_type = "lib"]
pub mod cell_doc {
    use std::*;

    use io::Error;
    // pub fn add(left: usize, right: usize) -> usize {
    //     left + right
    // }
    #[derive(Debug, Clone)]
    pub enum FileType<'p> {
        Delimited { delimiter: char },
        FixWidth { positions: &'p Vec<u16> },
    }

    // pub trait CellContent {}

    #[derive(Debug, Clone)]
    pub struct LineContent<'s> {
        pub cells: Vec<Option<&'s str>>,
    }
    impl<'s> LineContent<'s> {
        pub fn new(line_cells: Vec<Option<&str>>) -> LineContent {
            LineContent { cells: line_cells }
        }
        // fn get_cells() -> Vec<Option<&str>> {}
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
    #[derive(Debug)]
    // #[derive(Debug)]
    pub struct FileInfo<'f> {
        pub path: &'f str,
        pub file_type: FileType<'f>,
        pub has_header: bool,
        pub header: Option<LineContent<'f>>,
        file_content: Option<Result<String, Error>>,
    }
    impl<'f> FileInfo<'f> {
        pub fn new(file_path: &'f str, ftype: FileType<'f>, header: bool) -> Self {
            let f = FileInfo {
                path: file_path,
                file_type: ftype,
                has_header: header,
                header: None,
                file_content: None,
            };
            f
        }
        // fn set_header(&mut self, line: Option<LineContent<'f>>) {
        //     self.header = line;
        // }
        pub fn load(&'f mut self) -> Vec<LineContent> {
            let mut line_count = 0;
            // let mut header_line = None::<LineContent>;
            let mut output = Vec::<LineContent>::new();
            self.file_content = Some(fs::read_to_string(&self.path)); //.expect("unable to read file")
                                                                      //following approach does not really work quite right because the content is pointing to cont, but cont's lifetime is inside of the match.
                                                                      /*
                                                                      let content = match &file_result {
                                                                          Ok(ref cont) => cont,
                                                                          //For now, if any error while parsing file, treat as empty, no data. Potentially change to return a Result<Vec<LineContent>> could be better, though...
                                                                          //But for purposes of using this, likely any empty file handling should potentially be the same as having had a file error.
                                                                          // This is primarily just for learning, though, so not expecting any practical usage of this.
                                                                          Err(ref _e) => "", // panic!("Unable to read file at {} - {}", &self.path, e),
                                                                      };*/
            if let Some(Ok(ref content)) = self.file_content {
                for text in content.lines() {
                    let cell_split = match self.file_type {
                        FileType::Delimited { delimiter } => split_delimited(delimiter, &text),
                        FileType::FixWidth { positions } => split_by_positions(positions, &text),
                    };
                    let line = LineContent::new(cell_split);
                    if line_count == 0 {
                        if self.has_header {
                            // self.set_header(output.pop());
                            // Need to figure out lifetime of the line here, since not using a vec...
                            // Potentially need to look into RC? Or just add it into a Vec..That seems like unnecessary overhead, though?
                            // self.header = mem::take(output.pop());
                            // self.header = Some(LineContent::new(cell_split.clone()));
                            self.header = Some(line);
                            line_count = line_count + 1;
                            continue;
                        } else {
                            self.header = None;
                        }
                    }
                    output.push(line);
                    line_count = line_count + 1;
                }
            }
            output
        }
    }
    pub fn split_delimited(delim: char, src: &str) -> Vec<Option<&str>> {
        let cells = src.split(delim);
        let mut vs = Vec::<Option<&str>>::new();
        for cell in cells {
            if cell.is_empty() {
                vs.push(None);
            } else {
                vs.push(Some(&cell));
            }
        }
        vs
    }
    //Might need to change this to Vec<Option<string>> ? Need to research a bit more and practice with borrow/ownership of string content
    pub fn split_by_positions<'a>(positions: &Vec<u16>, src: &'a str) -> Vec<Option<&'a str>> {
        // let mut lp = 0u16;
        let col_count = positions.len();
        // if positions[colCount] == src.len(){
        //    colCount = colCount - 1;
        // }
        assert_ne!(0, col_count);
        let mut col_indices = src.char_indices();
        let mut start = positions[0]; // 0usize;
        let mut lp = start;
        let mut str_end: usize = 0;
        let mut vs = Vec::<Option<&str>>::with_capacity(col_count);
        // Since positions is starting positions, our last column will need to be populated after this loop.
        for pos_iter in positions.iter().enumerate() {
            // index, value
            let (idx, pos) = pos_iter;
            if idx == 0 {
                // lp = *pos;
                continue; // Positions contains starting indexes for columns.
                          /* Last column contains starting point for an extra column
                           So Col2 - Col1 -> Col1 slice
                           Col3 - Col2 = Col2 Slice
                           ..
                           Coln.. = ColN slice
                          */
            }

            let p = *pos;
            println!("Pos_iter: {:?}", pos);
            // Compare position against the last position
            // Last Position - Previous Position gives us the length of string..
            // First time
            /*
            if p == lp && p == start {
                if start > 0 {
                    if let Some(s) = col_indices.nth(start as usize) {
                        start = s.0 as u16;
                    }
                }
                continue; // Columns positions are start of column, rather than end of column.
            }*/

            // +1 because range is exclusive on right side of range
            // let ending_index = u16::from(p) - lp + 1;// p - lp + 1 is actually the length of the column, not the index information
            let ending_index = p + 1; // exclusive on right side.

            if let Some(s) = col_indices.nth(lp as usize) {
                start = s.0 as u16;
            }

            match col_indices.nth(usize::from(ending_index)) {
                Some(col_end) => {
                    let st = usize::from(start);
                    let (mut end, _) = col_end;
                    println!("Start:{:?}, End:{:?}", st, end);
                    if end < st {
                        end = end + st;
                    }
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
            lp = ending_index; // Set next column to start where this one stopped
        }
        //Get content of last column in line, or None if we finished the string while going through previous columns
        loop {
            match col_indices.next() {
                Some(col_end) => str_end = col_end.0,
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
}
#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
    #[test]
    fn test_column_parse_positions() {
        // let cols = vec![4u16, 9u16, 14u16, 21u16];
        let cols = vec![4u16, 10u16, 14u16, 22u16];
        let expected_count = cols.len();
        let input = "Col1Start Col2Next to Last column";
        // Col1Start Col2Next to Last column
        let vs = cell_doc::split_by_positions(&cols, input);
        let col_count_actual = vs.len();
        for v in vs {
            // let val = v.as_ref();
            println!("{:?}", v);
        }
        assert_eq!(col_count_actual, expected_count);
    }
}
