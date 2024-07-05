mod stack;
mod utils;
mod parser;
mod executer;
mod filenode;

fn main() {
    // NOTE: dir1/dir2/(file1,dir3/file2) doesn't let me create 'dir3/file2', solutions:
    // - find a 'std::fs::create_dir_all' for file
    // - split the filename by 'OUTER_SEPARATOR', create the dirs with 'std::fs::create_dir_all'
    // and then cd into it to create the file

    let argv = std::env::args().collect::<Vec<String>>();

    if argv.len() == 1 { return; }

    let arg = argv[1..].iter()
        .map(|s| s.as_str())
        .collect::<Vec<&str>>()
        .join(" ");

    // println!("{:#?}", parser::parse_expression(arg));

    match utils::is_valid_expression(arg.as_str()) {
        Ok(_) => executer::parse_and_execute(arg),
        Err(e) => println!("[ERROR] {}", e)
    }
}
