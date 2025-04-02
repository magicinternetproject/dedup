use futures::future::join_all;
use std::{
    collections::VecDeque,
    path::PathBuf,
};
use std::cmp::max;
use tokio::fs::*;
use tokio_uring::fs::File;
use std::collections::HashMap;

//
// next steps:
// - can we "dupe as we go" and when inserting "the next thing" answer "is it the same as anything in the list already?"
// - ...?
// - profit!
//
// or_insert(new()) versus or_insert_with(|| new()) --> does one get called more?


#[derive(Debug, Clone)]
struct DupeFile {
    path: PathBuf,
    size: i64,
    first: Vec<u8>,
    last: Vec<u8>,
}

struct Foo {
    size: i64,
}

fn new_foo() -> Foo {
    println!("new foo");
    Foo{size: 5}
}


fn main() {

/*
// "Why is ther or_insert() and or_insert_with()"?
    let mut stuff: HashMap<i64, Foo> = HashMap::new();

    stuff.entry(1234).or_insert(new_foo());
    stuff.entry(1234).or_insert(new_foo());
    stuff.entry(1234).or_insert(new_foo());

//    stuff.entry(1234).or_insert_with(|| new_foo());
//    stuff.entry(1234).or_insert_with(|| new_foo());
//    stuff.entry(1234).or_insert_with(|| new_foo());

    println!("{:?}", stuff.len());
*/

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
                    let size = f.metadata().await.unwrap().len() as i64;
		    futuristic.push(tokio_uring::spawn(create_dupe_file(f.path(), size)));
		    // tokio_uring::spawn(printafile(f.path())).await.unwrap();
		} else if ftype.is_dir() {
		    pd.push_back(f.path());
		}
	    }
	}
    }
    println!("{:?}", futuristic.len());
    let dupes = join_all(futuristic).await;

    let mut stuff: HashMap<i64, Vec<DupeFile>> = HashMap::new();
    for d in dupes {
        let d = d.unwrap().clone();
        //let &mut me: Vec<DupeFile> = stuff.entry(d.size).or_default();
        let me = stuff.entry(d.size).or_insert_with(|| Vec::new());
        me.push(d);
    }
    for x in stuff.keys() {
        if let Some(f) = stuff.get(x) {
            if f.len() > 1 {
                if let Some(head) = f.get(0) {
                    for y in f.iter().skip(1) {
                        if head.first == y.first && head.last == y.last {
                            println!("{:?}", y.path);
                        }
                    }
                }
            }
        }
    }
}


async fn create_dupe_file(f: PathBuf, size: i64) -> DupeFile {
    // Open the file without blocking
    let newf = f.clone();
    let file = File::open(f).await.unwrap();
    let buf0 = vec![0; 4 * 1_024];
    let buf1 = vec![0; 4 * 1_024];

    let (res, buf0) = file.read_at(buf0, 0).await;
    let r = res.unwrap() as i64;
    if r < 4096 && r < size {
        println!("weird thing {:?}", r);
    }

    let start = max(0, size - (4*1024)) as u64;
    let (_, buf1) = file.read_at(buf1, start).await;

    DupeFile{
        path: newf,
        size,
        first: buf0,
        last: buf1,
    }
}
