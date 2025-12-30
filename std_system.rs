/// Standard library implementation of SystemOps
/// This module is only available when building with std
extern crate alloc;
use alloc::string::String;
use crate::system::SystemOps;

pub struct StdSystemOps {
    readline_fn: Option<fn(&str) -> Option<String>>,
}

impl StdSystemOps {
    pub fn new(readline_fn: Option<fn(&str) -> Option<String>>) -> Self {
        StdSystemOps { readline_fn }
    }
}

impl SystemOps for StdSystemOps {
    fn read_file(&self, path: &str) -> Result<String, String> {
        use std::fs::File;
        use std::io::Read;

        let mut s = String::new();
        File::open(path)
            .and_then(|mut f| f.read_to_string(&mut s))
            .map(|_| s)
            .map_err(|e| e.to_string())
    }

    fn time_ms(&self) -> Result<i64, String> {
        use std::time::{SystemTime, UNIX_EPOCH};

        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i64 * 1000 + d.subsec_nanos() as i64 / 1_000_000)
            .map_err(|e| alloc::format!("{:?}", e))
    }

    fn readline(&self, prompt: &str) -> Option<String> {
        match self.readline_fn {
            Some(f) => f(prompt),
            None => None,
        }
    }
}
