use futures::future::join_all;
use std::{
    collections::VecDeque,
    io::{self, Write},
    path::{Path, PathBuf},
};
use tokio::fs::*;
use tokio_uring::fs::{DirBuilder, File};

fn main() {
    let mut pd: VecDeque<PathBuf> = VecDeque::new();
    pd.push_back(".".into());
    tokio_uring::start(real_main(&mut pd));
}
async fn real_main(pd: &mut VecDeque<PathBuf>) {
    let mut futuristic = vec![];
    while let Some(path) = pd.pop_front() {
	let mut dirs = read_dir(path).await.unwrap();
	while let Some(f) = dirs.next_entry().await.unwrap() {
	    if let Ok(ftype) = f.file_type().await {
		if ftype.is_file() {
		    futuristic.push(tokio_uring::spawn(printafile(f.path())));
		    // tokio_uring::spawn(printafile(f.path())).await.unwrap();
		} else if ftype.is_dir() {
		    pd.push_back(f.path());
		}
	    }
	}
    }
    let _huh = join_all(futuristic).await;
}

async fn printafile(f: PathBuf) {
    let out = io::stdout();
    let another_f = f.clone();

    // Open the file without blocking
    let file = File::open(f).await.unwrap();
    let mut buf = vec![0; 16 * 1_024];

    // Track the current position in the file;
    let mut pos = 0;
    let mut out = out.lock();
    let _whatever = out.write(another_f.to_str().unwrap().as_bytes());
    let _ = out.write(b"\n");

    loop {
	// Read a chunk
	let (res, b) = file.read_at(buf, pos).await;
	let n = res.unwrap();

	if n == 0 {
	    break;
	}

	// out.write_all(&b[..n]).unwrap();

	pos += n as u64;

	buf = b;
    }
}
