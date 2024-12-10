use std::{  io::BufRead};
use itertools::repeat_n;

/// Diskspace has a length and a file ID
#[derive (Clone, Copy, Debug)]
struct Diskspace {
    length: usize,
    file_id: FileId
}

impl Diskspace {
    fn to_blocks(&self) -> Vec<FileId> {
        repeat_n(self.file_id, self.length).collect()
    }

    fn new_file(length: char, file_id: usize) -> Self {
        Self {
            length: length.to_string().parse().unwrap(),
            file_id: FileId::File(file_id)
        }
    }

    fn new_empty(length: char) -> Self {
        Self {
            length: length.to_string().parse().unwrap(),
            file_id: FileId::Empty
        }
    }

    fn insert_into_space(&self, space: Self) -> Vec<Self> {
        match (self.file_id, space.file_id) {
            (FileId::Empty, FileId::Empty) => {
                panic!("Cannot insert empty into empty!");
            },
            (FileId::Empty, FileId::File(_)) => {
                panic!("Cannot insert empty into file!");
            },
            (FileId::File(_), FileId::File(_)) => {
                panic!("Cannot insert file into file!");
            },
            (_, _) => {
                if self.length > space.length {
                    panic!("Space is too small to insert file");
                }

                if self.length == space.length {
                    return vec![
                        *self
                    ]
                }

                return vec![
                    *self, 
                    Self {
                        length: space.length - self.length,
                        file_id: FileId::Empty
                    }
                ]
            }
        }
    }

    /// Creates a function for testing if other empty spaces are big enough to house self
    fn find_big_enough_factory(&self) -> impl Fn(&Self) -> bool + use<'_> {
        return |item: &Self| -> bool {
            item.file_id == FileId::Empty && item.length >= self.length
        };
    }

    /// Creates a function for finding a file by ID. Necessary as order in Vector may change
    fn find_by_id(&self) -> impl Fn(&Self) -> bool + use<'_> {
        return |item: &Self| -> bool {
            item.file_id == self.file_id
        };
    }

    /// Replaces itself with an empty diskspace. It's not necessary to merge these as files are only moved left
    fn empty(&self) -> Self {
        Self {
            length: self.length,
            file_id: FileId::Empty
        }
    }
}

/// FileId can either be a file id  or empty
#[derive (Clone, Copy, Debug, PartialEq)]
enum FileId {
    File(usize),
    Empty
}

pub fn run(file: Box<dyn BufRead>) -> Result<(), Box<dyn std::error::Error>> {
    let mut file_system: Vec<Diskspace> = vec![];
    // Without spaces, for iterating through files
    let mut files: Vec<Diskspace> = vec![];
    
    // This should be just one line
    let input = file.lines().next().unwrap().unwrap();

    // Read file system
    let mut input_iter = input.chars();
    let mut file_index: usize = 0;

    loop {
        // First read file
        match input_iter.next() {
            None => {
                break;
            }
            Some(length) => {
                files.push(Diskspace::new_file(length, file_index));
                file_system.push(Diskspace::new_file(length, file_index));
                
                file_index += 1;
            }
        }
        // Next read space to next file
        match input_iter.next() {
            None => {
                break;
            }
            Some(length) => {
                file_system.push(Diskspace::new_empty(length));
            }
        }
    }

    // Make files read only
    let files = files;

    for file in files.iter().rev() {
        match file_system.iter().position(file.find_big_enough_factory()) {
            None => {
                // Do nothing. No space is big enough
            }
            Some(i) => {
                // i is index of the space that's big enough
                // j is index of the file that is to be moved
                let j = file_system.iter().position(file.find_by_id()).unwrap();
                print!("\nMoving file {:?} at {} to {}",file.file_id, j, i);
                // Cancel if file comes before space
                if j < i {
                    print!(" - Cancelled!");
                    continue;
                }

                let tmp = file_system[j];
                file_system[j] = tmp.empty();

                let ins = tmp.insert_into_space(file_system.remove(i));
                // Reverse result to presere ordering
                for diskspace in ins.into_iter().rev() {
                    file_system.insert(i, diskspace);
                }
            }
        }
    }

    // Make file system read only
    let file_system = file_system;

    let expanded_file_system: Vec<FileId> = file_system.iter()
        .flat_map(|ds| ds.to_blocks())
        .collect();
    
    let checksum = expanded_file_system.iter().enumerate().fold(
        0, | acc, (i, n)| {
            acc + match n {
                FileId::Empty => { 0 },
                FileId::File(file_id) => { file_id * i}
            }
        });

    println!("\nChecksum: {}", checksum);

    Ok(())
}
