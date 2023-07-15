#![allow(non_snake_case)]

mod mp3_search;
mod types;

use warp::Filter;

use crate::mp3_search::search_mp3s;

use crate::types::api::SearchMp3Request;
use crate::types::types::TargetItem;

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

    // search path for mp3s. return target items list
    let searchMp3s=warp::path!("search-mp3")
        .and(warp::post())
        .and(warp::body::json())
        .map(|searchMp3Request:SearchMp3Request| {
            let result:Vec<TargetItem>=search_mp3s(
                searchMp3Request.searchPath,
                searchMp3Request.includeMaybe
            );

            return warp::reply::json(&result);
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