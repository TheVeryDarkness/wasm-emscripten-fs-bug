use std::io::{self, BufRead, Read, Write};

/// Echo.
fn main() {
    let mut stdin = io::stdin().lock();
    let mut buf = String::new();
    loop {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        if buf.is_empty() {
            break;
        }
        let mut stdout = io::stdout().lock();
        stdout.write(buf.as_bytes()).unwrap();
    }
}
