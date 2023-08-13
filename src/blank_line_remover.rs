pub fn remove_blank_lines(content: Vec<String>) -> Vec<String> {
    let content: Vec<String> = content.into_iter().filter(|s| !s.eq("")).collect();
    content
}

#[cfg(test)]
mod test {

    use crate::blank_line_remover::remove_blank_lines;

    #[test]
    fn should_remove_all_blank_lines() {
        //given
        let content: Vec<String> = vec!["1abc", "", "3 lol", "", "", "6 raka maka foun"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        //when
        let result: Vec<String> = remove_blank_lines(content);

        //then
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "1abc".to_string());
        assert_eq!(result[1], "3 lol".to_string());
        assert_eq!(result[2], "6 raka maka foun".to_string());
    }
}
