mod stack;
mod utils;
mod parser;
mod executer;
mod filenode;

fn main() -> std::io::Result<()> {
    // NOTE: INVALIDATE PATHS THAT START WITH A '/' TO NOT MESS UP WITH LINUX ROOT FOLDER
    // NOTE: WHEN THERE IS A 'INNER_SEPARATOR' JUST BEFORE CLOSING A PARENTHESIS, IT CRASHES

    // let exp = "(dir1.1,dir1.2)/(dir2.1,dir2.2)/dir3/(file1.1,file1.2,file1.3)";
    // println!("{:?}", executer::parse_and_execute(exp));

    let argv = std::env::args().collect::<Vec<String>>();

    let arg = argv[1..].iter()
        .map(|s| s.as_str())
        .collect::<Vec<&str>>()
        .join(" ");

    // println!("{:#?}", arg);

    executer::parse_and_execute(arg);

    Ok(())
}
