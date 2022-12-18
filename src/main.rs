use warp::Filter;

use std::collections::HashMap;

async fn hello(param: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(format!("{:#?}", param))
}

#[tokio::main]
async fn main() {
    /*
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Let's say hi to: {}!", name));
    */

    let hello = warp::get()
        .and(warp::path("hello"))
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::path::end())
        .and_then(hello);

    println!("Listening on http://127.0.0.1/3030");

    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
