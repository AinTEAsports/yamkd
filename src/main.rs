use self::utils::error_str;

mod stack;
mod utils;
mod parser;
mod executer;
mod filenode;

fn main() {
    let argv = &std::env::args().collect::<Vec<String>>()[1..];

    if argv.len() == 0 { return; }

    for arg in argv {
        match utils::is_valid_expression(arg) {
            Ok(_) => executer::parse_and_execute(arg.clone()),
            Err(e) => println!("{}", error_str(e))
        }
    }
}
