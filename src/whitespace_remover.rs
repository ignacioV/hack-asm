use regex::Regex;

pub fn remove_whitespaces(content: Vec<String>) -> Vec<String> {
    let re = Regex::new(r"\s").unwrap();
    let content: Vec<String> = content
        .into_iter()
        .map(|s| re.replace_all(&s, "").to_string())
        .collect();
    content
}

#[cfg(test)]
mod test {

    use crate::whitespace_remover::remove_whitespaces;

    #[test]
    fn should_remove_single_row_whitespaces() {
        //given
        let content: String = "this has some white    spaces".to_string();

        //when
        let result: Vec<String> = remove_whitespaces(vec![content]);

        //then
        assert_eq!(result[0], "thishassomewhitespaces".to_string())
    }

    #[test]
    fn should_remove_multiple_row_whitespaces() {
        //given
        let content: Vec<String> = vec![
            "this is the start".to_string(),
            "nowhitespace".to_string(),
            " some ".to_string(),
            "idk     ".to_string(),
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect();

        //when
        let result: Vec<String> = remove_whitespaces(content);

        //then
        assert_eq!(result.len(), 4);
        assert_eq!(result[0], "thisisthestart".to_string());
        assert_eq!(result[1], "nowhitespace".to_string());
        assert_eq!(result[2], "some".to_string());
        assert_eq!(result[3], "idk".to_string());
    }
}
