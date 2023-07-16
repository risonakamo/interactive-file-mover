#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod mp3_search;
mod types;

use std::sync::{Arc, Mutex};

use warp::Filter;
use warp::hyper::StatusCode;
use warp::reply::with_status;

use crate::mp3_search::search_mp3s;

use crate::types::api::{SearchMp3Request, SubmitItemsRequest};
use crate::types::types::{TargetItem, ServerState, TaggableItem, Mp3MoveAction, Mp3MoveTag, ServerPhase};

#[tokio::main]
async fn main()
{
    pretty_env_logger::init();

    let stateArc=Arc::new(Mutex::new(ServerState {
        tagItems:vec![],
        phase:ServerPhase::ID
    }));

    let stateFilter=warp::any().map(move || {
        return stateArc.clone();
    });

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

    // submit items to update state
    let submitItems=warp::path!("submit-items")
        .and(warp::post())
        .and(warp::body::json())
        .and(stateFilter.clone())
        .map(|submitItemsReq:SubmitItemsRequest,stateArc2:Arc<Mutex<ServerState>>| {
            let mut state=stateArc2.lock().unwrap();

            if state.phase!=ServerPhase::ID
            {
                return with_status(
                    "only allowed to use this api in ID mode".to_string(),
                    StatusCode::UNAUTHORIZED
                );
            }

            state.tagItems=submitItemsReq.items.into_iter()
                .map(|item:TargetItem|->TaggableItem {
                    return TaggableItem {
                        item:item,
                        moveAction:Mp3MoveAction {
                            moveType:Mp3MoveTag::none
                        }
                    };
                })
                .collect();

            return with_status(
                "success".to_string(),
                StatusCode::OK
            );
        });

    let routes=root
        .or(searchMp3s)
        .or(submitItems)
        .with(warp::log("warp"))
        .with(cors);

    warp::serve(routes).run((
        [0,0,0,0],
        4080
    )).await;
}