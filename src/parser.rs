use crate::filenode::FileNode;
use crate::utils;

pub const OUTER_SEPARATOR: char = '/';
pub const INNER_SEPARATOR: char = ',';

fn expression_to_singlevec(expr: String) -> Vec<String> {
    let positions = utils::find_indexes(expr.as_str(), OUTER_SEPARATOR).unwrap();
    utils::split_at(expr, positions)
}

pub fn get(s: &str, i: usize) -> Option<char> { s.chars().nth(i) }
pub fn gets(s: String, i: usize) -> Option<char> { s.chars().nth(i) }

fn multiunit_to_singleunit(multiunit: &str) -> Option<Vec<FileNode>> {
    match (get(multiunit, 0), get(multiunit, multiunit.len() - 2), get(multiunit, multiunit.len() - 1)) {
        (Some('('), Some(')'), Some(OUTER_SEPARATOR)) => {
            let mut chars = multiunit.chars();
            chars.next_back();

            match multiunit_to_singleunit(chars.as_str()) {
                Some(v) => Some(v.iter().map(|file| FileNode::new(file.name.clone(), true)).collect()),
                None => None
            }
        }
        (Some('('), _, Some(')')) => {
            let stripped_multiunit = &multiunit[1..multiunit.len() - 1];
            let positions = utils::find_indexes(stripped_multiunit, INNER_SEPARATOR).unwrap();

            Some(
                utils::split_at(stripped_multiunit.to_string(), positions)
                .iter()
                    .filter(|s| !s.is_empty())
                    .map(|s| FileNode::new(String::from(s), true))
                    .collect()
            )
        },
        _ => None
    }
}


fn singlevec_to_multivec(singlevec: Vec<String>) -> Vec<Vec<FileNode>> {
    let mut tmp: Vec<Vec<FileNode>> = vec![];
    let mut result: Vec<Vec<FileNode>> = vec![];

    for s in singlevec {
        match multiunit_to_singleunit(s.as_str()) {
            Some(v) => tmp.push(v),
            None => {
                tmp.push(vec![FileNode::new(s, true)]);
            }
        }
    }



    for (i, filevec) in tmp.iter().enumerate() {
        if i == tmp.len() - 1 {
            result.push(
                filevec.iter().map(|file| {
                    FileNode::new(
                        file.name.clone(),
                        gets(file.name.clone(), file.name.len() - 1) == Some(OUTER_SEPARATOR)
                    )
                }).collect()
            );
        } else { result.push(filevec.to_vec()); }
    }

    result
}


pub fn parse_expression(expr: String) -> Vec<Vec<FileNode>> {
    let expression = expr.as_str();
    let first = expression_to_singlevec(expr.clone());
    let second = singlevec_to_multivec(first.clone());

    let mut result: Vec<Vec<FileNode>> = vec![];

    for (i, filevec) in second.iter().enumerate() {
        if i == second.len() - 1 {
            result.push(filevec.iter()
                .map(|file| FileNode::new(
                        file.name.clone(),
                        get(expression, expression.len() - 1) == Some(OUTER_SEPARATOR)
                    )
                )
                .collect()
            );
        } else { result.push(filevec.to_vec()) }
    }

    result
}
