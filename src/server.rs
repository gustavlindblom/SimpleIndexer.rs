use std::fs::File;

use tiny_http::{Header, Method, Request, Response};

fn serve_request(request: Request) -> Result<(), ()> {
    match (request.method(), request.url()) {
        (Method::Get, "/" | "/index.html") => request
            .respond(Response::from_string("hello world"))
            .unwrap(),

        (Method::Get, "/index.js") => request
            .respond(
                Response::from_file(File::open("/index.js").expect("could not load index.js file"))
                    .with_header(
                        Header::from_bytes("Content-Type", "")
                            .expect("we do not put garbage in the headers"),
                    ),
            )
            .unwrap(),

        _ => request.respond(Response::from_string("404")).unwrap(),
    }

    Ok(())
}

pub fn start_listening(address: &str) -> Result<(), ()> {
    let server = tiny_http::Server::http(address).unwrap();

    println!(
        "INFO: server starting at: http://{address}",
        address = server.server_addr()
    );
    for request in server.incoming_requests() {
        serve_request(request).ok();
    }

    eprintln!("ERROR: the server socket has shut down");

    Err(())
}
