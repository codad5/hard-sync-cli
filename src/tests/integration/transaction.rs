use crate::{libs::helpers::get_transaction, modules::transaction::Transaction};

const BASE: &str = "c:/workspace";
const TARGET: &str = "c:/workspace";


#[test]
fn test_get_transaction(){
    let created_trans = get_transaction(BASE.to_owned(), TARGET.to_owned());
    let real_trans = Transaction::new(BASE.to_owned(), TARGET.to_owned());
    assert_eq!(created_trans, real_trans);
}