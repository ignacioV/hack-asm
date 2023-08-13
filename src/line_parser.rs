pub fn parse_new_lines(context: String) -> Vec<String> {
    let ws_parts: Vec<&str> = context.lines().collect();
    let parts: Vec<String> = ws_parts.into_iter().map(|s| s.to_string()).collect();
    parts
}

#[cfg(test)]
mod test {
    use crate::line_parser::parse_new_lines;

    #[test]
    fn should_parse_by_newlines_multiple_lines() {
        //given
        let context_3_lines = "line1\nline2\nline3".to_string();

        //when
        let result: Vec<String> = parse_new_lines(context_3_lines);

        //then
        assert_eq!(result.len(), 3);
        assert_eq!(&result[0], "line1");
        assert_eq!(&result[1], "line2");
        assert_eq!(&result[2], "line3");
    }

    #[test]
    fn should_parse_by_newlines_single_line() {
        //given
        let context_1_line = "line123".to_string();

        //when
        let result: Vec<String> = parse_new_lines(context_1_line);

        //then
        assert_eq!(result.len(), 1);
        assert_eq!(&result[0], "line123");
    }
}
