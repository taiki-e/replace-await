use std::{
    fs::{self, File},
    io::{self, BufReader, Read},
};

#[cfg(not(windows))]
use std::env::args_os;
#[cfg(windows)]
use wild::args_os;

fn main() -> io::Result<()> {
    let mut buf = Vec::new();
    args_os().skip(1).try_for_each(|file| {
        let mut r = BufReader::new(File::open(&file)?);
        r.read_to_end(&mut buf)?;
        find(&mut buf);
        fs::write(file, &buf)?;
        buf.clear();
        Ok(())
    })
}

fn find(bytes: &mut Vec<u8>) {
    const MACRO: &[&[u8]] = &[b"await!(", b"r#await!("];
    const FEATURE: &[&[u8]] = &[b", await_macro", b"await_macro, ", b"await_macro"];

    let mut i = 0;
    while i < bytes.len() {
        if remove(bytes, i, MACRO) {
            replace(bytes, i);
        } else {
            let _ = remove(bytes, i, FEATURE);
            i += 1;
        }
    }
}

fn remove(bytes: &mut Vec<u8>, i: usize, needles: &[&[u8]]) -> bool {
    for needle in needles {
        if (&bytes[i..]).starts_with(needle) {
            (0..needle.len()).for_each(|_| drop(bytes.remove(i)));
            return true;
        }
    }
    false
}

fn replace(bytes: &mut Vec<u8>, mut i: usize) {
    const AWAIT: &[u8] = b".await";

    let mut count = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'(' => count += 1,
            b')' => {
                if count == 0 {
                    bytes.remove(i);
                    AWAIT.iter().enumerate().for_each(|(j, &c)| bytes.insert(i + j, c));
                    return;
                } else {
                    count -= 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
}
