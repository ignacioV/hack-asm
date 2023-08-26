use std::fs;
use std::fs::File;
use std::io::Write;
pub fn write_to(file_path: &str, commands: Vec<String>) -> std::io::Result<()> {
    let mut file = File::create(file_path)?;
    let output = commands.join("\n");
    file.write_all(output.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod file_writer_test {

    use crate::hack_file_writer::write_to;
    use std::fs;

    #[test]
    fn should_write_vector_string_to_file() {
        //given
        let cmd1: &str = "someIDKcommand";
        let cmd2: &str = "do=minuifnitiy;jumop";
        let cmd3: &str = "@1lol345";
        let commands: Vec<String> = vec![cmd1.to_string(), cmd2.to_string(), cmd3.to_string()];
        let output_file: &str = "test-hack.asm";

        //when
        write_to(output_file, commands);

        //then
        let result: String = fs::read_to_string(output_file.to_string()).expect("Can't read file");
        assert_eq!(result, "someIDKcommand\ndo=minuifnitiy;jumop\n@1lol345");

        //cleanup
        cleanup_test_file(output_file).unwrap();
    }

    fn cleanup_test_file(file_path: &str) -> std::io::Result<()> {
        std::fs::remove_file(file_path)?;
        Ok(())
    }
}
