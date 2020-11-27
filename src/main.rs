#![deny(warnings)]
use std::{net::IpAddr, path::Path};
use std::fs::File;
//use mini_fs::zip::ZipFs;
use warp::Filter;
//use mini_fs::prelude::*;
use mini_fs::{MiniFs, ZipFs, Store, Entry};

// warp wants it like this
const LOCALHOST: IpAddr = [127, 0, 0, 1].into();

#[tokio::main]
async fn main() {

    let ziparg = get_zippath().expect("no zipfile specified");
    let zippath = Path::new(&ziparg);
    let zipfs = get_zipfs(zippath)
        .map(|z| z.index().expect("unable to index zipfile"))
        .expect("zipfile not found");

    let minifs = MiniFs::new().mount("zip", zipfs);
        
    let readme = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("./README.md"));

    // dir already requires GET...
    let examples = warp::path("ex").and(warp::fs::dir("./examples/"));

    let zip = warp::get()
        .and(warp::path("zip"))
        .map(|| 
            get_zipentries_at_path(&minifs, Path::new("/zip/")).map_or_else(
       |_error | String::default(), 
            |files|
                files
                    .join("\n")
            )
        );

    // GET / => README.md
    // GET /ex/... => ./examples/..
    let routes = readme.or(examples).or(zip);

    warp::serve(routes).run((LOCALHOST, 3030)).await;
}




// read zip
// https://docs.rs/mini-fs/0.2.2/mini_fs/zip/struct.ZipFs.html

fn get_zipfs(zipfile: &Path) -> std::io::Result<ZipFs<File>>{
    let file = ZipFs::open(zipfile);
    file
}

use std::env;

fn get_zippath() -> Option<String> {
    let args = env::args().collect::<Vec<String>>();
    args.first().map(|a| String::from(a))
}

use std::io;

fn extract_entry_name(entry: &Entry) -> Option<String> {
    Some(entry.name.into_string().unwrap())
}

fn get_zipentries_at_path(minifs: &MiniFs, path: &Path) -> io::Result<Vec<String>> {
    minifs
        .entries_path(Path::new("/zip/")) // TODO: add path
        .map(|list|
            list
                .filter_map(|maybeEntry|
                    maybeEntry
                        .map(|entry| extract_entry_name(&entry))
                        .unwrap()
                )
        )
        .map(|fd| Ok(fd.collect::<Vec<String>>()))
        .unwrap()
}