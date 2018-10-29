extern crate rayon;

use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;
use std::fs::*;
use std::io::*;
use std::iter::Iterator;
use std::error::Error;

fn main() {
    let output = File::create("index.txt").unwrap();
    let mut writer = BufWriter::new(output);

    let basedir = "/";
    let dir_reader = read_dir(basedir).unwrap();
    let mut indexer = Indexer::new();
    indexer.index(dir_reader, &mut writer);
    writer.flush().unwrap();

    println!("Finished Indexing {} files", indexer.get_count());
}

struct Indexer {
    counter: usize,
}

impl Indexer {
    fn index(&mut self, dir_reader: ReadDir, mut writer: &mut BufWriter<File>) {
        let entries: Result<Vec<DirEntry>> = dir_reader.collect();
        for entry in entries.par_iter() {
            let entry: DirEntry = entry.unwrap();
            // check if the entry is a directory or a file
            if entry.metadata().unwrap().is_dir() {
                self.index(read_dir(entry.path()).unwrap(), &mut writer);
            } else {
                let path = entry.path();
                let txt = path.to_str().unwrap_or("ERROR").as_bytes();
                writer.write_all(txt).unwrap();
                self.counter += 1;
            }
        }
    }
    fn get_count(&self) -> usize {
        self.counter
    }

    fn new() -> Self {
        Indexer {
            counter: 0,
        }
    }
}
