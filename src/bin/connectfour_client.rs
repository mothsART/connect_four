extern crate connectfour;

use std::process::Command;
use connectfour::conf::port;

fn main () {
    let url = format!("http://127.0.0.1:{}/", port());
    println!("Launch firefox with URL {}", url);
    Command::new("/usr/bin/firefox")
    .arg(url)
    .output()
    .expect("failed to execute process");
}
