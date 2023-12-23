use std::fs::{create_dir, metadata, read_dir, write};

fn main() {
    create_dir("workspace");
    write("workspace/a.txt", "I'm a.\n").unwrap();
    write("workspace/b.txt", "This is b.\n").unwrap();
    for file in read_dir("workspace").unwrap() {
        let file = file.unwrap();
        println!("{:?}", file.path());
        let meta = file.metadata().unwrap();
        println!("{:?}", meta);
        // This seems to be the same with created time.
        println!("{}", meta.len());
    }
    let meta = metadata("workspace").unwrap();
    println!("{:?}", meta);
    // This seems to ve the same with permission code.
    println!("{:?}", meta.file_type());
    println!("{}", meta.is_dir());
    println!("{}", meta.is_file());
    println!("{}", meta.is_symlink());
}
