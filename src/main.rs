use actix_web::{web, App, HttpResponse, HttpServer, dev::Server};

fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web.to(get_index))
    });

    println!("Serving on http://localhost:3000...");
    Server
        .bind("127.0.0.1:3000").expect("Error binding server to address")
        .run().expect("Error running server");
}

fn get_index() -> HttpResponse{
    HttpResponse::Ok()
    .content_type("text/html")
    .body(
        r#"
            <title>GCD Calculator</tile>
            <form action="/gcd" method="post">
            <input type="text" name="n"/>
            <input type="text" name="m"/>
            <button type="submit">Compute GCD</button>
            </form>
        "#,
    )   
}