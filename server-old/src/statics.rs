use rocket::response::NamedFile;
use std::io;
use std::path::{Path, PathBuf};


#[get("/")]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open(Path::new("static/index.html"))
}

#[get("/<path..>")]
pub fn static_files(path: PathBuf) -> io::Result<NamedFile> {
    NamedFile::open(Path::new("static/").join(path))
}
