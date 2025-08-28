use hyper::{Body, Request, Response, Server};

use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;


async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let path = req.uri().path();
    match path {
        "/" => Ok(Response::new(Body::from("Hello, World!"))),
        "/api/name"=>Ok(Response::new(Body::from("My name is Deep Panchal"))),
         "/api/age"=>Ok(Response::new(Body::from("My age is 20"))),  
        _ => Ok(Response::new(Body::from("Not Found"))),
    }
}



#[tokio::main]
async fn main() {
    // Address to bind to
    let addr = ([127, 0, 0, 1], 3000).into();

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle))
    });

    let server = Server::bind(&addr).serve(make_svc);


    if let Err(e) = server.await {
    eprintln!("server error: {}", e);
}

}