use crate::a_command_parser::A_CMD_PREFIX;

pub struct C_CMD {
    computation: COMP,
    destination: DEST,
    jump: JMP,
}

pub enum COMP {
    _0,
    _1,
    A,
}
impl COMP {
    pub fn from(comp_command: &str) -> Self {
        match comp_command {
            "0" => COMP::_0,
            "1" => COMP::_1,
            "A" => COMP::A,
            _any => todo!("[{}] was not expected", _any),
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
    M,
}
impl DEST {
    pub fn from(dest_command: &str) -> Self {
        match dest_command {
            "M" => DEST::M,
            "" => DEST::null,
            _any => todo!("[{}] was not expected", _any),
        }
    }

    pub fn value(&self) -> String {
        match self {
            DEST::null => String::from("000"),
            DEST::M => String::from("001"),
        }
    }
}

pub enum JMP {
    null,
}
impl JMP {
    pub fn from(jmp_command: &str) -> Self {
        match jmp_command {
            "" => JMP::null,
            _any => todo!("[{}] was not expected", _any),
        }
    }

    pub fn value(&self) -> String {
        match self {
            JMP::null => String::from("000"),
        }
    }
}

fn parse_into_parts(cmd: &str) -> (&str, &str, &str) {
    let dest_comp: Vec<&str> = cmd.split("=").collect();
    if dest_comp.len() == 1 {

        let comp_jmp: Vec<&str> = dest_comp[0].split(";").collect();
        if comp_jmp.len() == 1 {
            return ("", comp_jmp[0], "");
        } else {
            return ("", comp_jmp[0], comp_jmp[1]);
        }

    }
    let dest = dest_comp[0];
    let comp_jmp: Vec<&str> = dest_comp[1].split(";").collect();
    if comp_jmp.len() == 1 {
        return (dest, comp_jmp[0], "");
    }
    (dest, comp_jmp[0], comp_jmp[1])
}

impl C_CMD {
    const OP_CODE: &str = "1";
    const FILLER: &str = "11";

    pub fn new(cmd: &str) -> Self {
        let (dest, comp, jmp) = parse_into_parts(cmd);
        Self {
            computation: COMP::from(comp),
            destination: DEST::from(dest),
            jump: JMP::from(jmp),
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
    fn should_generate_c_command_with_just_comp() {
        //given
        let c_command: C_CMD = C_CMD::new("A");

        //when
        let result: String = c_command.to_binary();

        //then
        let expected: String = String::from("1110110000000000");
        assert_eq!(result, expected);
    }

    #[test]
    fn should_generate_c_command_with_dest_and_comp() {
        //given
        let c_command: C_CMD = C_CMD::new("M=A");

        //when
        let result: String = c_command.to_binary();

        //then
        let expected: String = String::from("1110110000001000");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_string_split() {
        //given
        let value = "D=D+A;JEQ";

        //when
        let parts: Vec<&str> = value.split("=").collect();

        //then
        assert_eq!(2, parts.len());
        assert_eq!("D", parts[0]);
        assert_eq!("D+A;JEQ", parts[1]);
    }

    use crate::c_command_parser::parse_into_parts;
    #[test]
    fn should_parse_into_parts_just_comp() {
        //given
        let cmd = "D+A";

        //when
        let (dest, comp, jmp) = parse_into_parts(cmd);

        //then
        assert_eq!("", dest);
        assert_eq!("D+A", comp);
        assert_eq!("", jmp);
    }

    #[test]
    fn should_parse_into_parts_dest_and_comp() {
        //given
        let cmd = "D=D+A";

        //when
        let (dest, comp, jmp) = parse_into_parts(cmd);

        //then
        assert_eq!("D", dest);
        assert_eq!("D+A", comp);
        assert_eq!("", jmp);
    }

    #[test]
    fn should_parse_into_parts_full_cmd() {
        //given
        let cmd = "D=D+A;JEQ";

        //when
        let (dest, comp, jmp) = parse_into_parts(cmd);

        //then
        assert_eq!("D", dest);
        assert_eq!("D+A", comp);
        assert_eq!("JEQ", jmp);
    }

    #[test]
    fn should_parse_into_parts_comp_jump() {
        //given
        let cmd = "D+A;JEQ";

        //when
        let (dest, comp, jmp) = parse_into_parts(cmd);

        //then
        assert_eq!("", dest);
        assert_eq!("D+A", comp);
        assert_eq!("JEQ", jmp);
    }
}
