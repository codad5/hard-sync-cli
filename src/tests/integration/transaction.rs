use crate::{libs::helpers::{get_transaction, map_path_to_target}, modules::transaction::Transaction};

const BASE: &str = "c:/workspace";
const TARGET: &str = "c:/workspace";


#[test]
fn test_get_transaction(){
    let created_trans = get_transaction(BASE.to_owned(), TARGET.to_owned());
    let real_trans = Transaction::new(BASE.to_owned(), TARGET.to_owned());
    assert_eq!(created_trans, real_trans);
}

#[test] 
fn test_map_path_to_target()
{
    let target = "c:/workspace2";
    let file_to_copy: [String; 7] = [
        String::from("c:/workspace/code/index.js"),
        String::from("c:/workspace/code/index.ts"),
        String::from("c:/workspace/code/test/index.js"),
        String::from("c:/workspace/code/test/index.ts"),
        String::from("c:/workspace/code/test/test2/index.js"),
        String::from("c:/workspace/code/press/test2/index.ts"),
        String::from("c:/workspace/code/press/index.js"),
    ];
    let expected: Vec<(Vec<String>, String)> = Vec::from([
        (vec![String::from("c:/workspace/code/index.js"), String::from("c:/workspace/code/index.ts")], String::from("c:/workspace2/code")),
        (vec![String::from("c:/workspace/code/test/index.js"), String::from("c:/workspace/code/test/index.ts")], String::from("c:/workspace2/code/test")),
        (vec![String::from("c:/workspace/code/test/test2/index.js")], String::from("c:/workspace2/code/test/test2")),
        (vec![String::from("c:/workspace/code/press/test2/index.ts")], String::from("c:/workspace2/code/press/test2")),
        (vec![String::from("c:/workspace/code/press/index.js")], String::from("c:/workspace2/code/press")),
    ]);

    let mapped_files = map_path_to_target(file_to_copy.to_vec(), target.to_owned(), BASE.to_owned());
    assert_eq!(mapped_files, expected);

    let file_to_copy: [String; 7] = [
        String::from("code//index.js"),
        String::from("code//index.ts"),
        String::from("code//test//index.js"),
        String::from("code//test//index.ts"),
        String::from("code//test//test2//index.js"),
        String::from("code//press//test2//index.ts"),
        String::from("code//press//index.js"),
    ];
    let expected: Vec<(Vec<String>, String)> = Vec::from([
        (vec![String::from("code//index.js"), String::from("code//index.ts")], String::from("c:/workspace2/code")),
        (vec![String::from("code//test//index.js"), String::from("code//test//index.ts")], String::from("c:/workspace2/code/test")),
        (vec![String::from("code//test//test2//index.js")], String::from("c:/workspace2/code/test/test2")),
        (vec![String::from("code//press//test2//index.ts")], String::from("c:/workspace2/code/press/test2")),
        (vec![String::from("code//press//index.js")], String::from("c:/workspace2/code/press")),
    ]);

    let mapped_files = map_path_to_target(file_to_copy.to_vec(), target.to_owned(), BASE.to_owned());
    assert_eq!(mapped_files, expected);

    let base = "c:/workspace/find";
    let target = "c:/workspace/dive";
    let file_to_copy: [String; 7] = [
        String::from("find//find.txt"),
        String::from("find//forever"),
        String::from("find//forever//sup//th.txt"),
        String::from("find//index.txt"),
        String::from("find//pixel.txt"),
        String::from("find//forever//sup"),
        String::from("find//forever//bent.txt"),
        ];
    let expected: Vec<(Vec<String>, String)> = Vec::from([
        (vec![String::from("find//find.txt"), String::from("find//forever"), String::from("find//index.txt"), String::from("find//pixel.txt")], String::from("c:/workspace/dive")),
        (vec![String::from("find//forever//sup//th.txt")], String::from("c:/workspace/dive/forever/sup")),
        (vec![String::from("find//forever//sup"), String::from("find//forever//bent.txt")], String::from("c:/workspace/dive/forever")),
        ]);
    let mapped_files = map_path_to_target(file_to_copy.to_vec(), target.to_owned(), base.to_owned());
    assert_eq!(mapped_files, expected);
        
        
    let base = "c:/workspace";
    let file_to_copy: [String; 7] = [
        String::from("find//find.txt"),
        String::from("find//forever"),
        String::from("find//forever//sup//th.txt"),
        String::from("find//index.txt"),
        String::from("find//pixel.txt"),
        String::from("find//forever//sup"),
        String::from("find//forever//bent.txt"),
    ];
    let expected: Vec<(Vec<String>, String)> = Vec::from([
        (vec![String::from("find//find.txt"), String::from("find//forever"), String::from("find//index.txt"), String::from("find//pixel.txt")], String::from("c:/workspace/dive/find")),
        (vec![String::from("find//forever//sup//th.txt")], String::from("c:/workspace/dive/find/forever/sup")),
        (vec![String::from("find//forever//sup"), String::from("find//forever//bent.txt")], String::from("c:/workspace/dive/find/forever")),
    ]);
    let mapped_files = map_path_to_target(file_to_copy.to_vec(), target.to_owned(), base.to_owned());
    assert_eq!(mapped_files, expected);
    
    
}