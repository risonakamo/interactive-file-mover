#![allow(non_snake_case)]

mod mp3_search;
mod types;

use warp::Filter;

use crate::types::api::SearchMp3Request;

#[tokio::main]
async fn main()
{
    let searchMp3s=warp::post()
        .and(warp::path("search-mp3"))
        .and(warp::body::json())
        .map(|searchMp3Request:SearchMp3Request| {
            println!("hello {:?}",searchMp3Request);
            return "huh";
        })
        .with(warp::log("warp"));

    warp::serve(searchMp3s).run((
        [0,0,0,0],
        4080
    )).await;
}