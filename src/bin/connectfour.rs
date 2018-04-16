#[macro_use]
extern crate rouille;

use std::fs::File;

fn main() {
    println!("Now listening on localhost:8000");
    rouille::start_server("localhost:8000", move |request| {
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
