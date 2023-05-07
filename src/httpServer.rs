mod getMyInfo;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Method, StatusCode};



async fn service(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());

    println!("nya");

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("nya");
        }
        (&Method::GET, "/myInfo") => {
            *response.body_mut() = Body::from(getMyInfo::get_my_info());
        }
        (&Method::POST, "/echo") => {
            // we'll be back
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    Ok(response)
}

pub async fn start_server() {
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(service))
    });
    let server = Server::bind(&addr).serve(make_svc);

    println!("server started");
    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}