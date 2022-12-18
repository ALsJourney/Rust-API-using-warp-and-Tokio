use warp::Filter;

#[tokio::main]
async fn main() {
    // Get /hello/warp => 200 Ok with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    println!("Listening on http://127.0.0.1/3030");
    

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}