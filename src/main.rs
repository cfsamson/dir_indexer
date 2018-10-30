extern crate rayon;

use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;
use std::fs::*;
use std::io::*;
use std::iter::Iterator;
use std::sync::{Arc, Mutex};

fn main() {
    let output = File::create("index.txt").unwrap();
    let writer = BufWriter::new(output);
    // create a counter and a mutex for the counter
    let counter = 0;
    let counter_mutex = Arc::new(Mutex::new(counter));
    // create a mutex for our bufreader
    let writer_mutex = Arc::new(Mutex::new(writer));
    let basedir = "/";
    let dir_reader = read_dir(basedir).unwrap();
    index(dir_reader, &writer_mutex, &counter_mutex);
    let mut handle = writer_mutex.lock().unwrap();
    handle.flush().unwrap();
    
    let index_count = counter_mutex.lock().unwrap();
    println!("Finished Indexing {} files", *index_count);
}

fn index(
    dir_reader: ReadDir,
    writer: &Arc<Mutex<BufWriter<File>>>,
    counter: &Arc<Mutex<usize>>,
) {
    let entries: Vec<Result<DirEntry>> = dir_reader.collect();
    entries.into_par_iter().for_each(move |entry| {
        match &entry {
            Err(e) => println!("Cannot read dir/file: {}", e),
            Ok(entry) => {
                // check if the entry is a directory or a file
                if entry.metadata().unwrap().is_dir() {
                    println!("Searching: {:?}", entry.file_name());
                    let writer = writer.clone();
                    index(read_dir(entry.path()).unwrap(), &writer, counter);
                } else {
                    let path = entry.path();
                    let txt = path.to_str().unwrap_or("ERROR").as_bytes();
                    let mut writer = writer.lock().unwrap();
                    writer.write_all(txt).unwrap();
                    let mut c = counter.lock().unwrap();
                    *c += 1;
                }
            }
        }
    })
}
