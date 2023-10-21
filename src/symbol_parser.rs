use std::collections::HashMap;

#[derive(Debug)]
pub struct SymbolsTable {
    symbols: HashMap<String, String>,
}

impl SymbolsTable {
    pub fn new() -> Self {
        let mut symbols: HashMap<String, String> = HashMap::new();
        symbols.insert("R1".to_string(), "1".to_string());
        symbols.insert("R2".to_string(), "2".to_string());
        symbols.insert("R3".to_string(), "3".to_string());
        symbols.insert("R4".to_string(), "4".to_string());
        symbols.insert("R5".to_string(), "5".to_string());
        symbols.insert("R6".to_string(), "6".to_string());
        symbols.insert("R7".to_string(), "7".to_string());
        symbols.insert("R8".to_string(), "8".to_string());
        symbols.insert("R9".to_string(), "9".to_string());
        symbols.insert("R10".to_string(), "10".to_string());
        symbols.insert("R11".to_string(), "11".to_string());
        symbols.insert("R12".to_string(), "12".to_string());
        symbols.insert("R13".to_string(), "13".to_string());
        symbols.insert("R14".to_string(), "14".to_string());
        symbols.insert("R15".to_string(), "15".to_string());
        symbols.insert("SCREEN".to_string(), "16384".to_string());
        symbols.insert("KBD".to_string(), "24576".to_string());
        symbols.insert("SP".to_string(), "0".to_string());
        symbols.insert("LCL".to_string(), "1".to_string());
        symbols.insert("ARG".to_string(), "2".to_string());
        symbols.insert("THIS".to_string(), "3".to_string());
        symbols.insert("THAT".to_string(), "4".to_string());
        Self { symbols }
    }

    pub fn get(&self, key: String) -> Option<String> {
        self.symbols.get(&key).cloned()
    }

    pub fn add_label(&mut self, label: String, line_no: u64) {
        self.symbols.insert(label, line_no.to_string());
    }

    //pub fn add_symbol(&mut self, label: String, line_no: u8) {
    //self.symbols.insert(label, line_no.to_string());
    //}
}

#[derive(Debug)]
pub struct NumeratedLine {
    pub line: String,
    pub number: Option<u64>, // none if its a label
}

pub fn numerate_lines(lines: Vec<String>) -> Vec<NumeratedLine> {
    let mut result: Vec<NumeratedLine> = vec![];
    let mut non_label_line_counter = 0;
    for line in lines {
        if line.starts_with("(") {
            result.push(NumeratedLine { line, number: None });
            continue;
        }
        result.push(NumeratedLine {
            line,
            number: Some(non_label_line_counter),
        });
        non_label_line_counter += 1;
    }
    result
}

pub fn add_labeled_lines_into_symbols_table(
    numerated_lines: Vec<NumeratedLine>,
    symbols_table: &mut SymbolsTable,
) {
    let mut current_line: u64 = 0;
    for line in numerated_lines {
        if let Some(line_number) = line.number {
            current_line = line_number;
        } else {
            symbols_table.add_label(parse_label(line.line), current_line + 1);
        }
    }
}

fn parse_label(raw: String) -> String {
    let len = raw.len();
    raw[1..len - 1].to_string()
}

pub fn remove_label_lines(lines: Vec<NumeratedLine>) -> Vec<NumeratedLine> {
    lines.into_iter()
        .filter(|line| line.number.is_some())
        .collect()
}

#[cfg(test)]
mod test {

    use crate::symbol_parser::add_labeled_lines_into_symbols_table;
    use crate::symbol_parser::numerate_lines;
    use crate::symbol_parser::NumeratedLine;
    use crate::symbol_parser::SymbolsTable;
    use crate::symbol_parser::remove_label_lines;

    #[test]
    fn should_number_lines_without_labels() {
        //given
        let lines: Vec<String> = vec![
            "M=0", "D=0", "A=0", "(BUUP)", "(LOOP)", "M+1", "@LOOP", "(WAM)", "M+1", "M+1",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect();

        //when
        let result: Vec<NumeratedLine> = numerate_lines(lines);

        //then
        assert_eq!(result.len(), 10);
        assert_eq!(result[0].number, Some(0));
        assert_eq!(result[1].number, Some(1));
        assert_eq!(result[2].number, Some(2));
        assert_eq!(result[3].number, None);
        assert_eq!(result[4].number, None);
        assert_eq!(result[5].number, Some(3));
        assert_eq!(result[6].number, Some(4));
        assert_eq!(result[7].number, None);
        assert_eq!(result[8].number, Some(5));
        assert_eq!(result[9].number, Some(6));
    }

    #[test]
    fn should_add_labeled_lines_number_into_symbols_table() {
        //given
        let lines: Vec<String> = vec![
            "M=0", "D=0", "A=0", "(BUUP)", "(LOOP)", "M+1", "@LOOP", "(BAM)", "M+1", "M+1",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect();

        let numerated_lines: Vec<NumeratedLine> = numerate_lines(lines);

        let mut symbols_table = SymbolsTable::new();
        //when

        add_labeled_lines_into_symbols_table(numerated_lines, &mut symbols_table);

        //then
        assert_eq!(symbols_table.get("LOOP".to_string()), Some("3".to_string()));
        assert_eq!(symbols_table.get("BUUP".to_string()), Some("3".to_string()));
        assert_eq!(symbols_table.get("BAM".to_string()), Some("5".to_string()));
    }

    #[test]
    fn should_remove_label_lines() {
        //given
        let lines: Vec<String> = vec![
            "M=0", "D=0", "A=0", "(BUUP)", "(LOOP)", "M+1", "@LOOP", "(BAM)", "M+1", "M+1",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect();

        let numerated_lines: Vec<NumeratedLine> = numerate_lines(lines);

        //when
        let result: Vec<NumeratedLine> = remove_label_lines(numerated_lines);

        //then
        assert_eq!(result.len(), 7);
        assert_eq!(result[0].number, Some(0));
        assert_eq!(result[1].number, Some(1));
        assert_eq!(result[2].number, Some(2));
        assert_eq!(result[3].number, Some(3));
        assert_eq!(result[4].number, Some(4));
        assert_eq!(result[5].number, Some(5));
        assert_eq!(result[6].number, Some(6));
    }
}
