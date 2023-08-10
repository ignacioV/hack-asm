// gaunu strings[] , dabar kiekvienas is ju turi, jei turi // <comment>, man viska reikia isstrinti

pub fn remove_comments(content: Vec<String>) -> Vec<String> {
    // todo find all `// <any length ... >` and replace it with empty string
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
    fn sample_strings() {
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
        println!("content: {:?}", content);

        //then
        assert_eq!(content.len(), 5);
    }
}
