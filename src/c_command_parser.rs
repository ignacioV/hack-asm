use crate::a_command_parser::A_CMD_PREFIX;

pub struct C_CMD {
    computation: COMP,
    destination: DEST,
    jump: JMP,
}

pub enum COMP {
    _0,
    _1,
    A
}
impl COMP {

    pub fn from(comp_command: &str) -> Self {
        match comp_command {
            "0" => COMP::_0,
            "1" => COMP::_1,
            "A" => COMP::A,
             _ => todo!("[{}] was not expected", comp_command),
        }
    }

    pub fn value(&self) -> String {
        match &self {
            COMP::_0 => String::from("101010"),
            COMP::_1 => String::from("111111"),
            COMP::A => String::from("110000"),
        }
    }

    pub fn a_val(&self) -> String {
        match &self {
            COMP::_0 => String::from("0"),
            COMP::_1 => String::from("0"),
            COMP::A => String::from("0"),
        }
    }
}

pub enum DEST {
    null,
}
impl DEST {
    pub fn value(&self) -> String {
        match self {
            DEST::null => String::from("000"),
        }
    }
}

pub enum JMP {
    null,
}
impl JMP {
    pub fn value(&self) -> String {
        match self {
            JMP::null => String::from("000"),
        }
    }

}


impl C_CMD {
    const OP_CODE: &str = "1";
    const FILLER: &str = "11";

    pub fn new(cmd: &str) -> Self {
        // parse just COMP but how to determine
        // jeigu yra = -> yea DEST -> nuo 0 iki `=` simbolio idx
        // jeigu yra ; -> yra JMP -> nuo `;` simbolio idx iki pabaigos
        Self {
            computation: COMP::from(cmd),
            destination: DEST::null,
            jump: JMP::null,
        }
    }

    pub fn is_c_command(command: &str) -> bool {
        !command.starts_with(A_CMD_PREFIX)
    }

    fn to_binary(&self) -> String {
        format!(
            "{}{}{}{}{}{}",
            C_CMD::OP_CODE,
            C_CMD::FILLER,
            self.computation.a_val(),
            self.computation.value(),
            self.destination.value(),
            self.jump.value()
        )
    }
}

#[cfg(test)]
mod test_struct {

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

    #[test]
    fn should_generate_sample_c_command() {
        //given
        let c_command: C_CMD = C_CMD::new("0");

        //when
        let result: String = c_command.to_binary();

        //then
        let expected: String = String::from("1110101010000000");
        assert_eq!(result, expected);
    }

    #[test]
    fn should_generate_c_command_from_string() {
        //given
        let c_command: C_CMD = C_CMD::new("A");

        //when
        let result: String = c_command.to_binary();

        //then
        let expected: String = String::from("1110110000000000");
        assert_eq!(result, expected);
    }
}
