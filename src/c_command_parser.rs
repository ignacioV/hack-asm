use crate::a_command_parser::A_CMD_PREFIX;

pub struct C_CMD {
    computation: COMP,
}

pub enum COMP {
    _0,
    _1,
}
impl COMP {
    pub fn value(&self) -> String {
        match self {
            COMP::_0 => String::from("101010"),
            COMP::_1 => String::from("111111"),
        }
    }
}

impl C_CMD {
    const OP_CODE: &str = "1";
    const FILLER: &str = "11";

    pub fn new(cmd: &str) -> Self {
        Self {
            computation: COMP::_0,
        }
    }

    pub fn is_c_command(command: &str) -> bool {
        !command.starts_with(A_CMD_PREFIX)
    }
}

#[cfg(test)]
mod test {

    use crate::c_command_parser::C_CMD;

    #[test]
    fn should_determine_c_command() {
        //given
        let c_command: &str = "D=D+1";

        //when
        let result: bool = C_CMD::is_c_command(c_command);

        //then
        assert!(result)
    }
}
