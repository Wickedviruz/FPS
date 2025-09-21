use std::fs;
use std::path::Path;

fn main() {
    let src = Path::new("assets_raw"); // utgår från tools/xtask/
    let dst = Path::new("assets");

    // rensa gamla assets
    if dst.exists() {
        fs::remove_dir_all(dst).unwrap();
    }

    // kopiera om alla assets
    fs_extra::dir::copy(
        src,
        dst,
        &fs_extra::dir::CopyOptions::new().copy_inside(true),
    )
    .unwrap();

    println!("Assets copied from assets_raw → assets/");
}
