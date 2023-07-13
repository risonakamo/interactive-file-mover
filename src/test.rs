#![allow(non_snake_case)]

mod mp3_search;
mod types;

use crate::mp3_search::search_mp3s;

fn main()
{
    let res=search_mp3s(
        "C:\\Users\\ktkm2\\Desktop\\song jobs\\songs 2023-06-28\\todo".to_string(),
        false
    );

    for item in res
    {
        println!("{:?}",item);
    }

    // println!("{:?}",res);
}