use std::fs;

struct FileReader {
    file_path: String,
}

impl FileReader {
    pub fn new(file_path: &str) -> Self {
        Self {
            file_path: file_path.to_string(),
        }
    }

    fn read(&self) -> String {
        fs::read_to_string(self.file_path.clone()).expect("Can't read file")
    }
}

#[cfg(test)]
mod tests {
    use crate::file_reader::FileReader;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn should_open_file_and_return_contents_as_string() {
        //given
        let file_path = "test.asm";
        setup_test_file(file_path).unwrap();
        let file_reader = FileReader::new(file_path);

        //when
        let contents = file_reader.read();
        println!("read contents: {}", contents);

        //then
        assert!(!contents.is_empty());

        //cleanup
        cleanup_test_file(file_path).unwrap();
    }

    fn setup_test_file(file_path: &str) -> std::io::Result<()> {
        let mut file = File::create(file_path)?;
        file.write_all(b"test1\n")?;
        file.write_all(b"test2")?;
        Ok(())
    }

    fn cleanup_test_file(file_path: &str) -> std::io::Result<()> {
        std::fs::remove_file(file_path)?;
        Ok(())
    }
}
