use std::path::PathBuf;
use serde::Serialize;

#[derive(Debug,Serialize)]
pub struct TargetItem
{
    /// full path of item
    pub path:PathBuf,

    /// name of parent folder relative to the original search folder (not path)
    pub parent:String,

    /// name of item
    pub itemName:String
}