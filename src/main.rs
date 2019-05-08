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
    const MACRO_1: &[u8] = b"await!(";
    const MACRO_2: &[u8] = b"r#await!(";
    const FEATURE_1: &[u8] = b", await_macro";
    const FEATURE_2: &[u8] = b"await_macro, ";
    const FEATURE_3: &[u8] = b"await_macro";

    let mut i = 0;
    while i < bytes.len() {
        if (&bytes[i..]).starts_with(MACRO_1) {
            (0..MACRO_1.len()).for_each(|_| drop(bytes.remove(i)));
            replace(bytes, i);
            continue;
        } else if (&bytes[i..]).starts_with(MACRO_2) {
            (0..MACRO_2.len()).for_each(|_| drop(bytes.remove(i)));
            replace(bytes, i);
            continue;
        }
        if (&bytes[i..]).starts_with(FEATURE_1) {
            (0..FEATURE_1.len()).for_each(|_| drop(bytes.remove(i)));
        } else if (&bytes[i..]).starts_with(FEATURE_2) {
            (0..FEATURE_2.len()).for_each(|_| drop(bytes.remove(i)));
        } else if (&bytes[i..]).starts_with(FEATURE_3) {
            (0..FEATURE_3.len()).for_each(|_| drop(bytes.remove(i)));
        }
        i += 1;
    }
}

fn replace(bytes: &mut Vec<u8>, offset: usize) {
    const AWAIT: &[u8] = b".await";

    let mut count = 0;
    let mut i = offset;
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
