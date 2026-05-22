use std::env;
use std::io::{self, Read};

pub const DEFAULT_MAX_INPUT_BYTES: usize = 10 * 1024 * 1024;

pub fn resolve_max_bytes(cli_override: Option<usize>) -> usize {
    if let Some(n) = cli_override {
        return n;
    }
    if let Ok(raw) = env::var("ROSETTA_MAX_INPUT_BYTES") {
        if let Ok(n) = raw.parse::<usize>() {
            return n;
        }
    }
    DEFAULT_MAX_INPUT_BYTES
}

pub fn read_bounded<R: Read>(mut reader: R, max_bytes: usize) -> io::Result<String> {
    let mut buf = Vec::new();
    let mut chunk = [0u8; 8192];
    loop {
        let n = reader.read(&mut chunk)?;
        if n == 0 {
            break;
        }
        if buf.len() + n > max_bytes {
            return Err(io::Error::other(format!(
                "input exceeds maximum of {max_bytes} bytes"
            )));
        }
        buf.extend_from_slice(&chunk[..n]);
    }
    Ok(String::from_utf8_lossy(&buf).into_owned())
}
