use logos::Source;

const OUTER_SEPARATOR: char = '/';
const INNER_SEPARATOR: char = ',';

#[derive(Debug, Clone)]
pub struct FileNode {
    pub name: String,
    pub is_dir: bool,
}

impl FileNode {
    pub fn new(name: String, is_dir: bool) -> Self {
        // println!("[FILENODE] is_dir={}", is_dir);

        let len = name.len();
        let mut processed = name.clone();

        if gets(name, len-1) == Some(OUTER_SEPARATOR) { processed.truncate(len-1); }

        FileNode {
            name: processed,
            is_dir,
        }

        // FileNode { name, is_dir }
    }
}



fn expression_to_singlevec(expression: &str) -> Vec<String> {
    /* Converts an expression to a singlevec
     * 
     * Example:
     * "dir1/(dir2.1,dir2.2)/(file1.1,file1.2)" => vec!["dir1", "(dir2.1,dir2.2)", "(file1.1,file1.2)"]
     * "dir1/(dir2.1,dir2.2)/(file1.1,file1.2)/" => vec!["dir1", "(dir2.1,dir2.2)", "(file1.1,file1.2)/"]
     */

    let tmp: Vec<&str> = expression.split(OUTER_SEPARATOR).filter(|s| !s.is_empty()).collect();
    let mut result: Vec<String> = vec![];

    // NOTE: copy and modify the last one to check if it is a dir or not
    for (i, s) in tmp.iter().enumerate() {
        if i == tmp.len() - 1 {
            result.push(
                if get(expression, expression.len() - 1) != Some(OUTER_SEPARATOR) { String::from(*s) }
                else { format!("{}{}", s, OUTER_SEPARATOR) }
            );
        }
        else { result.push(String::from(*s)); }
    }

    result
}

fn get(s: &str, i: usize) -> Option<char> { s.chars().nth(i) }
fn gets(s: String, i: usize) -> Option<char> { s.chars().nth(i) }

fn multiunit_to_singleunit(multiunit: &str) -> Option<Vec<FileNode>> {
    // NOTE: check if it is between parenthesis, then split by 'INNER_SEPARATOR' then collect to a
    // vec

    match (get(multiunit, 0), get(multiunit, multiunit.len() - 2), get(multiunit, multiunit.len() - 1)) {
        (Some('('), Some(')'), Some(OUTER_SEPARATOR)) => {
            let mut chars = multiunit.chars();
            chars.next_back();

            multiunit_to_singleunit(chars.as_str())
        }
        (Some('('), _, Some(')')) => {
            let stripped_multiunit = &multiunit[1..multiunit.len() - 1];

            Some(
                stripped_multiunit
                    .split(INNER_SEPARATOR)
                    // .map(|s| FileNode { name: String::from(s), is_dir: true })
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
                // tmp.push(vec![FileNode { name: s, is_dir: true }]);
                tmp.push(vec![FileNode::new(s, true)]);
            }
        }
    }



    for (i, filevec) in tmp.iter().enumerate() {
        if i == tmp.len() - 1 {
            // println!("[DEBUG.singlevec_to_multivec()] {:?}", filevec);

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


pub fn parse_expression(expression: &'static str) -> Vec<Vec<FileNode>> {
    let first = expression_to_singlevec(expression);
    let second = singlevec_to_multivec(first.clone());

    let mut result: Vec<Vec<FileNode>> = vec![];

    // println!("[DEBUG.parse_expression().first] {:?}", first);
    // println!("[DEBUG.parse_expression().second] {:?}", second);

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
