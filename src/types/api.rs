use serde::{Deserialize,Serialize};

/// request to search for mp3s
#[derive(Deserialize,Serialize,Debug)]
pub struct SearchMp3Request
{
    pub searchPath:String,
    pub includeMaybe:bool
}