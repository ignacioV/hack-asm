// A command stuff------------------------------

pub struct A_CMD {
    reference: u16,
}

impl A_CMD {
    const PREFIX: &str = "@";
    const OP_CODE: &str = "0";

    pub fn new(cmd: &str) -> Self {
        Self {
            reference: Self::get_int_val(cmd),
        }
    }

    pub fn is_a_command(command: &str) -> bool {
        command.starts_with(A_CMD::PREFIX)
    }

    fn get_int_val(cmd: &str) -> u16 {
        cmd[1..].parse::<u16>().unwrap()
    }

    fn to_binary(&self) -> String {
        let binary_val: String = format!("{:b}", self.reference);
        format!("{}{:0>15}", A_CMD::OP_CODE, binary_val)
    }
}

#[cfg(test)]
mod test_struct {

    use crate::command_parser::A_CMD;

    #[test]
    fn should_determine_type_a_command() {
        //given
        let a_command: &str = "@123";

        //when
        let result: bool = A_CMD::is_a_command(a_command);

        //then
        assert!(result)
    }

    #[test]
    fn should_parse_into_a_command() {
        //given
        let a_command: &str = "@1";

        //when
        let got: A_CMD = A_CMD::new(a_command);

        //then
        assert_eq!(got.reference, 1)
    }

    #[test]
    fn a_command_into_binary() {
        //given
        let a_command: A_CMD = A_CMD::new("@1");

        //when
        let got: String = a_command.to_binary();

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
//----------------------------------------
//----------------------------------------

pub fn is_a_command(command: String) -> bool {
    command.starts_with("@")
}

pub fn a_cmd_to_binary(a_command: String) -> String {
    let int_val: &u16 = &a_command[1..].parse::<u16>().unwrap();
    let binary_val: String = format!("{int_val:b}");
    let a_op_code: &str = "0";
    format!("{a_op_code}{:0>15}", binary_val)
}
//----------------------------------------

#[cfg(test)]
mod test {
    use crate::command_parser::a_cmd_to_binary;
    use crate::command_parser::is_a_command;

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
