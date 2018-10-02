#[macro_use]
extern crate rouille;
extern crate connectfour;

use std::fs::File;
use connectfour::conf::port;

fn main() {
    let port = port();
    println!("Now listening on localhost:{}", port);
    rouille::start_server(
        format!("localhost:{}", port),
        move |request| {
        {
            let response = rouille::match_assets(&request, ".");
            if response.is_success() {
                return response;
            }
        }
        router!(request,
            (GET) (/) => {
                let file = File::open("static/index.html").unwrap();
                rouille::Response::from_file("text/html", file)
            },
            _ => rouille::Response::empty_404()
        )
    });
}
