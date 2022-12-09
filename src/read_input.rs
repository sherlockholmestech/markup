pub mod read_args {
    pub fn read_input_file() -> String {
        use std::env;
        use std::fs;
        let args: Vec<String> = env::args().collect();
        let filename = &args[1];
        let input = fs::read_to_string(filename).expect("Expected input");
        return input;
    }
}