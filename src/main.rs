#![deny(warnings)]
use std::path::Path;
use std::fs::File;
use mini_fs::zip::ZipFs;
use warp::Filter;

#[tokio::main]
async fn main() {

    let ziparg = get_zippath().expect("no zipfile specified");
    let zippath = Path::new(&ziparg);
    let _zipfs = get_zipfs(zippath)
        .map(|z| z.index())
        .expect("zipfile {} not found");

    let readme = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("./README.md"));

    // dir already requires GET...
    let examples = warp::path("ex").and(warp::fs::dir("./examples/"));

    let zip = warp::get()
        .and(warp::path("zip"))
        .map(|| "zip");
//        .and(zipfs.entries());

    // GET / => README.md
    // GET /ex/... => ./examples/..
    let routes = readme.or(examples).or(zip);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
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
