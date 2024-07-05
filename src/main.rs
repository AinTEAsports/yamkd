mod stack;
mod utils;
mod parser;

fn main() {
    let exp = "dir1/(dir2.1,dir2.2)/(dir3.1,dir3.2)/";
    // let exp = "(dir1.1,dir1.2)/";

    println!("{:?}", parser::parse_expression(exp));
}
