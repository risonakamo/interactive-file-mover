use serde::Deserialize;

/// request to search for mp3s
#[derive(Deserialize,Debug)]
pub struct SearchMp3Request
{
    pub searchPath:String,
    pub includeMaybe:bool
}