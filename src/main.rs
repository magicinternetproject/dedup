use tokio::fs::*;
use tokio_uring::fs::{DirBuilder, File};

#[tokio::main]
async fn main() {
    // list files in "this" dir
    let mut dirs = read_dir(".").await.unwrap();
    // println!("{:?}", dirs);
    // queue dirs
    while let Some(f) = dirs.next_entry().await.unwrap() {
        // check for directory, async
        println!("{:?}", f.path());
    }
}

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     tokio_uring::start(async {
//	// Open a file
//	let file = File::open("Cargo.toml").await?;

//	let buf = vec![0; 4096];
//	// Read some data, the buffer is passed by ownership and
//	// submitted to the kernel. When the operation completes,
//	// we get the buffer back.
//	let (res, buf) = file.read_at(buf, 0).await;
//	let n = res?;

//	// Display the contents
//	println!("{:?}", &buf[..n]);

//	Ok(())
//     })
// }
