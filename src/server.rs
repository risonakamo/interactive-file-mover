#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod mp3_search;
mod types;
mod taggable_item;

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use warp::Filter;
use warp::hyper::StatusCode;
use warp::reply::with_status;

use crate::mp3_search::search_mp3s;

use crate::types::api::{SearchMp3Request, SubmitItemsRequest, ServerError};
use crate::types::types::{TargetItem, ServerState, TaggableItem, Mp3MoveAction, Mp3MoveTag, ServerPhase};

#[tokio::main]
async fn main()
{
    pretty_env_logger::init();

    let stateArc=Arc::new(Mutex::new(ServerState {
        tagItems:vec![],
        phase:ServerPhase::ID,
        currentTagItem:0,
        previewDir:PathBuf::from("./previews")
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

    // search path for mp3s. return target items list
    // returns list of target items as json
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
    // returns single text string
    // errors are also text string
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

            println!("got {} items",submitItemsReq.items.len());

            state.tagItems=submitItemsReq.items.into_iter()
                .map(|item:TargetItem|->TaggableItem {
                    return TaggableItem::new(item);
                })
                .collect();

            state.phase=ServerPhase::TAG;

            return with_status(
                "success".to_string(),
                StatusCode::OK
            );
        });

    // get the current item to be tagged
    // errors are ServerError json
    let getCurrentItem=warp::path!("current-item")
        .and(warp::get())
        .and(stateFilter.clone())
        .map(|stateArc2:Arc<Mutex<ServerState>>| {
            let mut state=stateArc2.lock().unwrap();

            if state.phase!=ServerPhase::TAG
            {
                return with_status(
                    warp::reply::json(&ServerError {
                        detail:"can only use this in api phase".to_string()
                    }),
                    StatusCode::UNAUTHORIZED
                );
            }

            if state.currentTagItem>=state.tagItems.len()
            {
                return with_status(
                    warp::reply::json(&ServerError {
                        detail:"current item out of range".to_string()
                    }),
                    StatusCode::INTERNAL_SERVER_ERROR
                );
            }

            let currentItem:&mut TaggableItem=state.tagItems[state.currentTagItem];
            currentItem.initialisePreview(state.previewDir);

            return with_status(
                warp::reply::json(&currentItem),
                StatusCode::OK
            );
        });

    let fileroute=warp::path!("test")
        .and(warp::get())
        .and(warp::fs::file("C:\\Users\\ktkm2\\Desktop\\song jobs\\songs 2023-06-28\\todo\\Do It Yourself!! THEME SONGS mp3\\y\\02.続く話.mp3"));

    let routes=searchMp3s
        .or(submitItems)
        .or(fileroute)
        .or(getCurrentItem)
        .with(warp::log("warp"))
        .with(cors);

    warp::serve(routes).run((
        [0,0,0,0],
        4080
    )).await;
}