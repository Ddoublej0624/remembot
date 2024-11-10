use warp::Filter;
use std::net::SocketAddr;
use tokio;

#[tokio::main]
async fn main() {
    println!("Hello World");
    let index_html = include_str!("index.html");

    let index_route = 
        warp::path("index.html").map(move ||  {
            warp::http::Response::builder()
            .header("Content-Type", "text/html")
            .body(index_html)
    });

    let routes = index_route;

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    warp::serve(routes)
        .run(addr)
        .await; 
}
