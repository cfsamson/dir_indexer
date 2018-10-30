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
    let counter: u64 = 0;
    let counter_mutex = Arc::new(Mutex::new(counter));

    // create a mutex for our bufreader
    let writer_mutex = Arc::new(Mutex::new(writer));
    let basedir = "/";
    let dir_reader = read_dir(basedir).unwrap();

    println!("Starting indexing...");

    // This is our parallel indexer
    index(dir_reader, &writer_mutex, &counter_mutex);
    let mut handle = writer_mutex.lock().unwrap();
    handle.flush().unwrap();

    let index_count = counter_mutex.lock().unwrap();
    println!("Finished!");
    println!("Indexed {} files", *index_count);
}

/// A parallel indexer. Returns the number of files indexed.
/// Should not panic under normal curcumstances, but prints messages
/// to the stodout when it encounters a problem
fn index(dir_reader: ReadDir, writer: &Arc<Mutex<BufWriter<File>>>, counter: &Arc<Mutex<u64>>) {
    
    // we collect the entries in each directory to a vector to use
    // the rayon parallel iterator for easy parallel execution
    let entries: Vec<Result<DirEntry>> = dir_reader.collect();
    entries.into_par_iter().for_each(move |entry| {
        match &entry {
            Err(e) => println!("Cannot read dir/file: {}", e),
            Ok(entry) => {
                // check if the entry is a directory or a file
                match entry.metadata() {
                    Err(e) => println!("Cannot access: {}", e),
                    Ok(entr) => {
                        // check if the entry is a directory
                        if entr.is_dir() {
                            match read_dir(entry.path()) {
                                Err(_) => {
                                    // an error reading the direcotry is in most cases
                                    // caused by denied access. This is a simplification though
                                    // so you could inspect the error message if you want more
                                    // "correct" diagnostics printed
                                    let path_buf = entry.path();
                                    let path = path_buf.to_str().unwrap_or("Invalid path");
                                    println!("Can't access: {}", path);
                                    },
                                // if it is a path, recursively go through the entries in parallel
                                Ok(path) => index(path, &writer, counter),
                            }
                        // if it's not a directory it's a file
                        } else {
                            // get the file name. If the path somehow does contain invalid
                            // UTF8 caracters, we index the file as ERROR
                            let path = entry.path();
                            let txt = path.to_str().unwrap_or("ERROR");

                            // take a lock on the mutex
                            let mut writer = writer.lock().unwrap();

                            // write the text and a newline. There are several ways of doing this.
                            writeln!(writer, "{}", txt).unwrap();

                            // take a lock on the mutex for keeping count and increase the index_count by 1
                            let mut c = counter.lock().unwrap();
                            *c += 1;
                        }
                    }
                }
            }
        }
    })
}
