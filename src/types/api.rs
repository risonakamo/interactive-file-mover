use serde::{Deserialize,Serialize};

use crate::types::types::TargetItem;

/// request to search for mp3s
#[derive(Deserialize,Debug)]
pub struct SearchMp3Request
{
    pub searchPath:String,
    pub includeMaybe:bool
}

#[derive(Debug,Deserialize)]
pub struct SubmitItemsRequest
{
    pub items:Vec<TargetItem>
}