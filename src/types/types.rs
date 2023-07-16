use std::path::PathBuf;
use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize)]
pub enum Mp3MoveTag
{
    none,
    yes,
    no,
    maybe
}

#[derive(Debug,PartialEq)]
pub enum ServerPhase
{
    ID,
    TAG
}

/// item to be handled
#[derive(Debug,Serialize,Deserialize)]
pub struct TargetItem
{
    /// full path of item
    pub path:PathBuf,

    /// name of parent folder relative to the original search folder (not path)
    pub parent:String,

    /// name of item
    pub itemName:String
}

/// item with tag states
#[derive(Debug,Serialize)]
pub struct TaggableItem
{
    pub item:TargetItem,

    pub moveAction:Mp3MoveAction,

    pub previewPath:Option<PathBuf>
}

#[derive(Debug,Serialize)]
pub struct Mp3MoveAction
{
    pub moveType:Mp3MoveTag
}

#[derive(Debug)]
pub struct ServerState
{
    pub tagItems:Vec<TaggableItem>,
    pub phase:ServerPhase,

    pub currentTagItem:usize,

    pub previewDir:PathBuf
}