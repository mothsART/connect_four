extern crate connectfour;

use connectfour::{ConnectFourDataBaseStruct, ConnectFourDataBase};

fn main () {
    let mut db = ConnectFourDataBaseStruct::new();
    db.refresh();
}
