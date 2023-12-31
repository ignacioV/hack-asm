use crate::a_command_parser::A_CMD;
use crate::c_command_parser::C_CMD;

#[derive(Debug)]
pub enum SimpleCommand {
    A(A_CMD),
    C(C_CMD),
}

impl SimpleCommand {
    pub fn new(cmd: &str) -> Self {
        if A_CMD::is_a_command(cmd) {
            return Self::A(A_CMD::new(cmd));
        }
        if C_CMD::is_c_command(cmd) {
            return Self::C(C_CMD::new(cmd));
        }
        todo!("Not recognized command")
    }

    pub fn to_binary(&self) -> String {
        match self {
            SimpleCommand::A(a_cmd) => a_cmd.to_binary(),
            SimpleCommand::C(c_cmd) => c_cmd.to_binary(),
        }
    }
}

pub fn parse_simple(commands: Vec<String>) -> Vec<SimpleCommand> {
    commands
        .iter()
        .map(|cmd| SimpleCommand::new(cmd))
        .collect()
}

pub fn turn_into_binary(commands: Vec<SimpleCommand>) -> Vec<String> {
    commands.into_iter().map(|cmd| cmd.to_binary()).collect()
}

#[cfg(test)]
mod command_test {
    use crate::command_parser::parse_simple;
    use crate::command_parser::turn_into_binary;
    use crate::command_parser::SimpleCommand;

    #[test]
    fn should_create_a_command() {
        //given
        let a_cmd: &str = "@123";

        //when
        let result = SimpleCommand::new(a_cmd);

        //then
        assert_eq!("0000000001111011", result.to_binary());
    }

    #[test]
    fn should_create_a_command_2() {
        //given
        let a_cmd: &str = "@2";

        //when
        let result = SimpleCommand::new(a_cmd);

        //then
        assert_eq!("0000000000000010", result.to_binary());
    }

    #[test]
    fn should_create_c_command() {
        //given
        let c_cmd: &str = "D=M-D;JNE";

        //when
        let result = SimpleCommand::new(c_cmd);

        //then
        assert_eq!("1111000111010101", result.to_binary());
    }

    #[test]
    fn should_parse_multiple_commands() {
        //given
        let a_cmd1: String = "@123".to_string();
        let c_cmd2: String = "D=M-D;JNE".to_string();
        let a_cmd3: String = "@321".to_string();
        let c_cmd4: String = "A=D|M".to_string();
        let commands: Vec<String> = vec![a_cmd1, c_cmd2, a_cmd3, c_cmd4];

        //when
        let result = parse_simple(commands);

        //then
        assert_eq!(result.len(), 4);
        assert_eq!("0000000001111011", result[0].to_binary());
        assert_eq!("1111000111010101", result[1].to_binary());
        assert_eq!("0000000101000001", result[2].to_binary());
        assert_eq!("1111010101100000", result[3].to_binary());
    }

    #[test]
    fn should_turn_vec_command_to_binary_vec() {
        //given
        let a_cmd1: String = "@123".to_string();
        let c_cmd2: String = "D=M-D;JNE".to_string();
        let a_cmd3: String = "@321".to_string();
        let c_cmd4: String = "A=D|M".to_string();
        let commands: Vec<String> = vec![a_cmd1, c_cmd2, a_cmd3, c_cmd4];
        let parsed_commands = parse_simple(commands);

        //when
        let result: Vec<String> = turn_into_binary(parsed_commands);

        //then
        assert_eq!(result.len(), 4);
        assert_eq!("0000000001111011", result[0]);
        assert_eq!("1111000111010101", result[1]);
        assert_eq!("0000000101000001", result[2]);
        assert_eq!("1111010101100000", result[3]);
    }
}
