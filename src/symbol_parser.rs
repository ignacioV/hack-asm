
// ka man cia dabar reikia padaryti
// pradzioj, tiesiog pasidaryti map, kuriame butu sudetos visos konstantos
// tada reikia sunumeruoti visas eilutes (bet ne LABELius) kad poto galeciau zinoti kur kas pointina
// tada kiekviena 
//23 predefined symbols:
//symbol | value
//R0     | 0
//R1     | 1
//R2     | 2
//...    | ...
//R15    | 15
//SCREEN | 16384
//KBD    | 24576
//SP     | 0
//LCL    | 1
//ARG    | 2
//THIS   | 3
//THAT   | 4
use std::collections::HashMap;

#[derive(Debug)]
pub struct SymbolsTable{
    symbols: HashMap<String, String>
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
        Self {
            symbols
        }
    }

    // NOTE: DO IN TDD STYLE
    //pub fn add_label(&mut self, label: String, line_no: u8) {
        //self.symbols.insert(label, line_no.to_string());
    //}

    //pub fn add_symbol(&mut self, label: String, line_no: u8) {
        //self.symbols.insert(label, line_no.to_string());
    //}
}
}

#[cfg(test)]
mod test {
    
    use crate::symbol_parser::SymbolsTable;

    #[test]
    fn should() {
        //given
        // turiu map, kuriame yra sudeti simboliai
        let symbolsTable = SymbolsTable::new();

        //when

        //then
        print!("table: {:?}", symbolsTable);
    }

}
