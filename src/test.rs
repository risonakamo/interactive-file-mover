#![allow(non_snake_case)]

use glob::glob;
use std::path::PathBuf;

#[derive(Debug)]
struct TargetItem
{
    /// full path of item
    pub path:PathBuf,

    /// name of parent folder relative to the original search folder (not path)
    pub parent:String,

    /// name of item
    pub itemName:String
}

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

/// search for mp3s, except for ones that are in folders with certain names
fn search_mp3s(target_dir:String,include_maybe:bool)->Vec<TargetItem>
{
    return glob((target_dir.clone()+"\\**\\*.mp3").as_str()).unwrap()
    .filter_map(|itemResult|->Option<TargetItem> {
        let item:PathBuf=itemResult.unwrap();

        let mut parent:PathBuf=item.clone();
        parent.pop();

        let parentStem:String=match parent.file_stem() {
            None => "".to_string(),
            Some(res) => res.to_str().unwrap().to_string()
        };

        // if the parent is certain folder names
        if mp3IgnoreDir(&parentStem,include_maybe)
        {
            return None;
        }

        let parentPath:String=parent
            .strip_prefix(&target_dir).unwrap()
            .to_path_buf()
            .to_str().unwrap()
            .to_string();

        let itemName:String=item.file_name().unwrap().to_str().unwrap().to_string();

        return Some(TargetItem {
            path:item,
            parent:parentPath,
            itemName:itemName
        });
    })
    .collect();
}

/// return if the dir name is an ignored dir
fn mp3IgnoreDir(dirName:&String,includeMaybe:bool)->bool
{
    if dirName=="y"
        || dirName=="n"
        || dirName=="yes"
        || dirName=="no"
        // only consider m folder if include maybe is enabled
        || (
            includeMaybe
            && (
                dirName=="m"
                || dirName=="maybe"
            )
        )
    {
        return true;
    }

    return false;
}