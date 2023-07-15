#![allow(non_snake_case)]

mod mp3_search;
mod types;

use warp::Filter;

use crate::types::api::SearchMp3Request;

#[tokio::main]
async fn main()
{
    pretty_env_logger::init();

    let cors=warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[
            warp::http::Method::POST,
            warp::http::Method::OPTIONS,
            warp::http::Method::GET
        ]);

    let root=warp::get()
        .map(|| {
            return "huh";
        });

    let searchMp3s=warp::path!("search-mp3")
        .and(warp::post())
        .and(warp::body::json())
        .map(|searchMp3Request:SearchMp3Request| {
            println!("hello {:?}",searchMp3Request);
            return "huh2";
        });

    let routes=root
        .or(searchMp3s)
        .with(warp::log("warp"))
        .with(cors);

    warp::serve(routes).run((
        [0,0,0,0],
        4080
    )).await;
}