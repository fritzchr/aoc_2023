pub fn read_input(input_file: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(input_file)
}
