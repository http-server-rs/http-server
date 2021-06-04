use anyhow::Result;
use std::io::{self, Write};

pub struct Log;

impl Log {
    pub fn print(&self, buff: &[u8]) -> Result<()> {
        let stdout = io::stdout();
        let mut handle = stdout.lock();

        handle.write_all(buff)?;
        handle.flush()?;

        Ok(())
    }
}
