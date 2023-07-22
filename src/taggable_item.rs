use std::{path::PathBuf,fs::{copy,canonicalize}};

use crate::types::types::{TaggableItem, TargetItem, Mp3MoveAction, Mp3MoveTag};

impl TaggableItem
{
    /// create new taggable item from target item
    pub fn new(targetItem:TargetItem)->Self
    {
        return Self {
            item:targetItem,
            moveAction:Mp3MoveAction {
                moveType:Mp3MoveTag::none
            },
            previewPath:None
        };
    }

    /// initialise the preview file of the taggable item
    pub fn initialisePreview(&mut self,previewsDir:&PathBuf)->()
    {
        if self.previewPath.is_some()
        {
            return;
        }

        self.previewPath=Some(makePreviewCopy(
            &self.item.path,
            &previewsDir
        ));
    }
}

/// given an item, copy it to the preview dir with a new hashed name. return the preview
/// copy's full path
fn makePreviewCopy(itemPath:&PathBuf,previewsDir:&PathBuf)->PathBuf
{
    let previewItemPath:PathBuf=previewsDir.join(itemPath.file_name().unwrap());

    if previewItemPath.is_file()
    {
        println!("preview already exists, skipping");
        return canonicalize(previewItemPath).unwrap();
    }

    println!("making copy of {:?}",itemPath);
    copy(&itemPath,&previewItemPath).unwrap();

    return canonicalize(previewItemPath).unwrap();
}