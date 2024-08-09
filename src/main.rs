// Import necessary components from the actix_web crate
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use log::info;

// Handler for GET requests to "/hello"
async fn get_hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, GET!") // Respond with "Hello, GET!" and a 200 OK status
}

// Handler for POST requests to "/hello"
async fn post_hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, POST!") // Respond with "Hello, POST!" and a 200 OK status
}

// Handler for PUT requests to "/hello"
async fn put_hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, PUT!") // Respond with "Hello, PUT!" and a 200 OK status
}

// Handler for DELETE requests to "/hello"
async fn delete_hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, DELETE!") // Respond with "Hello, DELETE!" and a 200 OK status
}
// This attribute macro sets up the Actix runtime and designates the main function as the entry point for the Actix web server.
#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("Starting server at http://0.0.0.0:8080");
    
    HttpServer::new(|| {
        // Creates a new instance of the HTTP server.
        App::new() // Creates a new Actix web application.
            .route("/hello", web::get().to(get_hello)) // Defines a route for GET requests to "/hello"
            .route("/hello", web::post().to(post_hello)) // Defines a route for POST requests to "/hello"
            .route("/hello", web::put().to(put_hello)) // Defines a route for PUT requests to "/hello"
            .route("/hello", web::delete().to(delete_hello)) // Defines a route for DELETE requests to "/hello"
    })
    .bind("0.0.0.0:8080")? // Binds the server to the specified address and port.
    .run() // Runs the server.
    .await // Awaits the completion of the server's execution.
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_rt::test]
    async fn test_get_hello() {
        let mut app = test::init_service(App::new().route("/hello", web::get().to(get_hello))).await;
        let req = test::TestRequest::get().uri("/hello").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 200);
        let body = test::read_body(resp).await;
        assert_eq!(body, "Hello, GET!");
    }

    #[actix_rt::test]
    async fn test_post_hello() {
        let mut app = test::init_service(App::new().route("/hello", web::post().to(post_hello))).await;
        let req = test::TestRequest::post().uri("/hello").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 200);
        let body = test::read_body(resp).await;
        assert_eq!(body, "Hello, POST!");
    }

    #[actix_rt::test]
    async fn test_delete_hello() {
        let mut app = test::init_service(App::new().route("/hello", web::delete().to(delete_hello))).await;
        let req = test::TestRequest::delete().uri("/hello").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 200);
        let body = test::read_body(resp).await;
        assert_eq!(body, "Hello, DELETE!");
    }

    #[actix_rt::test]
    async fn test_put_hello() {
        let mut app = test::init_service(App::new().route("/hello", web::put().to(put_hello))).await;
        let req = test::TestRequest::put().uri("/hello").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 200);
        let body = test::read_body(resp).await;
        assert_eq!(body, "Hello, PUT!");
    }

}
