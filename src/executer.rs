use crate::parser::parse_expression;
use crate::filenode::FileNode;
use crate::utils::error_str;

use std::path::Path;

pub fn parse_and_execute(expression: String) {
    execute_multivec(parse_expression(expression));
}


fn execute_multivec(v: Vec<Vec<FileNode>>) {
    if v.is_empty() { return; }

    let head = v[0].clone();
    let tail = v[1..].to_vec();

    for file in head {

        match &file.create() {
            Ok(_) => (),
            Err(e) => {
                println!("{}", error_str(e));
                continue;
            }
        }

        if file.is_dir {
            let absolute_path_result = std::env::current_dir();

            if absolute_path_result.is_err() {
                println!("COULD NOT GET CURRENT DIRECTORY");
                continue;
            }

            let abscurr_path = absolute_path_result.unwrap();
            let absolute_path = Path::new(&abscurr_path).join(file.name.clone());

            match std::env::set_current_dir(absolute_path.clone()) {
                Ok(_) => {
                    execute_multivec(tail.clone());
                    let _ = std::env::set_current_dir(Path::new(&absolute_path).join(".."));
                }
                Err(_) => ()
            }
        }
    }
}
