#![forbid(unsafe_code)]
#![warn(rust_2018_idioms, single_use_lifetimes, unreachable_pub)]
#![warn(clippy::all)]

use std::{
    fs::{self, File},
    io::{self, BufReader, Read},
};

#[cfg(not(windows))]
use std::env::args_os;
#[cfg(windows)]
use wild::args_os;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(1)
    }
}

fn try_main() -> io::Result<()> {
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
        if bytes[i..].starts_with(needle) {
            bytes.drain(i..i + needle.len());
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
                    bytes.splice(i..=i, AWAIT.iter().cloned());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_await() {
        let mut buf = b"await!(foo(await!(bar)))".to_vec();
        find(&mut buf);
        assert_eq!(&buf, b"foo(bar.await).await");
    }
}
