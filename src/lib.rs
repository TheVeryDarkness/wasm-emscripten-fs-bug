use std::{fs, path::Path};

pub fn tree1(path: impl AsRef<Path>, depth: usize) {
    print!("{} {:?} ", "  ".repeat(depth), path.as_ref());
    if path.as_ref().is_dir() {
        println!("{:?}", path.as_ref());
        let files = fs::read_dir(path).unwrap();
        for file in files {
            tree1(file.unwrap().path(), depth + 1);
        }
    } else if path.as_ref().is_file() {
        println!("{:?}", path.as_ref());
    } else {
        println!("{:?}", path.as_ref().metadata().unwrap());
    }
}

pub fn tree2(path: impl AsRef<Path>, depth: usize) {
    print!("{}", "  ".repeat(depth));
    if let Ok(files) = path.as_ref().read_dir() {
        println!("{:?}", path.as_ref());
        for file in files {
            tree2(file.unwrap().path(), depth + 1);
        }
    } else {
        println!("{:?}", path.as_ref());
    }
}
