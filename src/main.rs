mod stack;
mod utils;
mod parser;
mod executer;
mod filenode;

fn main() {
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
