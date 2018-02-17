extern crate connect_four;

use connect_four::{ConnectFourDataBaseStruct, ConnectFourDataBase};

fn main () {
    let mut db = ConnectFourDataBaseStruct::new();
    db.refresh();
}
