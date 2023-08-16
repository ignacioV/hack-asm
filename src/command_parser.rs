//--- komandos @ parsinimas
//- paziureti ar prasideda komanda su `@` ->
//- paversti ta skaiciu einanti po `@` i binary
//- issaugoti ji kaip `0 + binary skaicius (padded to n=15)` // op code = 0

// saugoti abidvi komanadas i enuma. ir abudu enumai tures `fn toMachine() -> String`


// turiu tarkim @7
// 1st bit - op code = 0 
// rest 15 bits - padded left binary
// sita turetu paversi i 0000 0000 0000 0101

// tai ka pirma reikia padaryti
// DONE pirma paziureti ar komanda prasideda su @
// toliau, pasiimti viska kas eina poto ir paversti i binary
pub enum ASM_CMD {
    A_CMD { op_code: u8, reference: String },
    C_CMD,
}

impl ASM_CMD {
    pub fn to_machine_cmd(&self) -> String {
        unimplemented!()
    }
}

pub fn is_a_command(command: String) -> bool {
    command.starts_with("@")
}

pub fn a_cmd_to_binary(a_command: String) -> String {
    let int_val: &u16 = &a_command[1..].parse::<u16>().unwrap();
    let binary_val: String = format!("{int_val:b}");
    let a_op_code: &str = "0";
    format!("{a_op_code}{:0>15}", binary_val)
}

#[cfg(test)]
mod test {
    use crate::command_parser::is_a_command;
    use crate::command_parser::a_cmd_to_binary;

    #[test]
    fn should_determine_string_is_a_command() {
        //given
        let a_command: String = "@123".to_string();

        //when
        let result: bool = is_a_command(a_command);

        //then
        assert!(result)
    }

    #[test]
    fn should_determine_string_is_not_a_command() {
        //given
        let a_command: String = "D-1".to_string();

        //when
        let result: bool = is_a_command(a_command);

        //then
        assert!(!result)
    }

    #[test]
    fn should_parse_a_command_value_into_padded_binary() {
        //given
        let a_command: String = "@1".to_string();

        //when
        let got: String = a_cmd_to_binary(a_command);

        //then
        let mut expected: String = "".to_string();
        expected.push_str(&"0"); // op code 0 + 0's for 4 bits
        expected.push_str(&"000"); 
        expected.push_str(&"0000"); 
        expected.push_str(&"0000"); 
        expected.push_str(&"0001"); 
        assert_eq!(got, expected)
    }
}

