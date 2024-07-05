mod stack;
mod utils;
mod parser;
mod executer;
mod filenode;

fn main() -> std::io::Result<()> {
    // NOTE: INVALIDATE PATHS THAT START WITH A '/' TO NOT MESS UP WITH LINUX ROOT FOLDER
    // NOTE: WHEN THERE IS A 'INNER_SEPARATOR' JUST BEFORE CLOSING A PARENTHESIS, IT CRASHES

    let argv = std::env::args().collect::<Vec<String>>();

    if argv.len() == 1 { return Ok(()); }

    let arg = argv[1..].iter()
        .map(|s| s.as_str())
        .collect::<Vec<&str>>()
        .join(" ");

    executer::parse_and_execute(arg);

    Ok(())
}
