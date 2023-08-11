use regex::Regex;

pub fn remove_comments(content: Vec<String>) -> Vec<String> {
    let re = Regex::new(r"//.*$").unwrap();
    let content: Vec<String> = content
        .into_iter()
        .map(|s| re.replace_all(&s, "").to_string())
        .collect();
    content
}

#[cfg(test)]
mod test {
    use crate::comment_remover::remove_comments;

    #[test]
    fn should_remove_line_comment() {
        //given
        let content: Vec<String> = vec!["// this is the start"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        //when
        println!("content: {:#?}", content);

        let result: Vec<String> = remove_comments(content);

        //then
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "".to_string());
    }

    #[test]
    fn should_remove_multiple_line_comments() {
        //given
        let content: Vec<String> = vec![
            "// this is the start",
            "someCMD",
            "otherCMD //comment",
            "idk     //",
            "   //    this is the end // and more more more",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect();

        //when
        let result = remove_comments(content);
        println!("result: {:?}", result);

        //then
        assert_eq!(result.len(), 5);
        assert_eq!(result[0], "".to_string());
        assert_eq!(result[1], "someCMD".to_string());
        assert_eq!(result[2], "otherCMD ".to_string());
        assert_eq!(result[3], "idk     ".to_string());
        assert_eq!(result[4], "   ".to_string());
    }
}
