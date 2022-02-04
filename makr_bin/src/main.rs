use std::{fs::OpenOptions, io::Read};

fn main() {
    let app = clap::App::new("MAKr")
        .arg(clap::Arg::new("INPUT_FILE")
            .index(1)
            .required(true))
        .get_matches();
    
    let file_name = app.value_of("INPUT_FILE").unwrap();
    let mut file = OpenOptions::new()
        .read(true)
        .open(file_name)
        .expect(format!("Could not open file {}!", file_name).as_str());
    
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect("Could not read file!");

    let transpiled = makr_transpiler::transpile(buffer);
    
}
