mod rough;

fn report(line: u32, location: String, message: String) {
    println!("[line {}] Error {} {} ", line, location, message);
}

fn error(line: u32, error: String) {
    report(line, "".to_string(), error);
}
