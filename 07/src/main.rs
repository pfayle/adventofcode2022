use std::io;
use std::io::Read;
use std::collections::HashMap;
use std::str::Lines;

const TOTAL_SIZE: u32 = 70000000;
const SIZE_REQUIRED: u32 = 30000000;

#[derive(Debug)]
struct Directory {
    subdirectories: HashMap<String, Box<Directory>>,
    files: HashMap<String, File>,
}

impl Directory {
    fn size(&self) -> u32 {
        let mut ret = 0;
        ret += self.files.values().map(|f| f.size).sum::<u32>();
        ret += self.subdirectories.values().map(|d| d.size()).sum::<u32>();
        ret
    }
}

#[derive(Debug)]
struct File {
    size: u32,
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).expect("Error reading stdin.");
    let fs = process_dir(&mut buf.lines());
    let dirs = get_all_dirs(&fs);
    let size_sum: u32 = dirs.iter().map(|d| d.size()).filter(|n| *n <= 100000).sum();
    let space_to_reclaim = SIZE_REQUIRED - (TOTAL_SIZE - fs.size());
    let smallest_dirsize = dirs.iter().map(|d| d.size()).filter(|n| *n >= space_to_reclaim)
        .min().unwrap();

    println!("Combined size of directories smaller than 100000: {}", size_sum);
    println!("Size of smallest directory that can be deleted to free enough space: {}", smallest_dirsize);
}

fn process_dir(lines: &mut Lines) -> Directory {
    let mut ret = Directory{
        subdirectories: HashMap::new(),
        files: HashMap::new(),
    };
    let iter = lines;
    while let Some(line) = iter.next() {
        let mut contents = line.split(" ");
        match contents.next() {
            Some("$") => {
                match contents.next() {
                    Some("cd") => {
                        match contents.next() {
                            Some("/") => {},
                            Some("..") => {
                                break;
                            },
                            Some(dir) => {
                                ret.subdirectories.insert(dir.to_string(), Box::new(process_dir(iter)));
                            },
                            None => unreachable!(),
                        }
                    },
                    Some("ls") => {
                        // do nothing
                    },
                    _ => unreachable!(),
                }
            },
            Some("dir") => {
                // do nothing
            },
            Some(size) => {
                if let Some(name) = contents.next() {
                    ret.files.insert(
                        name.to_string(),
                        File{
                            size: size.parse::<u32>().unwrap(),
                        }
                    );
                }
            },
            _ => unreachable!(),
        }
    }
    ret
}

fn get_all_dirs(dir: &Directory) -> Vec<&Directory> {
    let mut ret = vec![dir];
    for d in dir.subdirectories.values() {
        for d2 in get_all_dirs(&d) {
            ret.push(d2);
        }
    }
    ret
}
