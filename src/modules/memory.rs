use serde::{Deserialize, Serialize};
use sysinfo::System;

use std::error::Error;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum MemoryOpts {
    Used,
    Free,
    PercUsed,
    PercFree,
}

pub fn memory_usage(opt: &MemoryOpts) -> Result<String, Box<dyn Error>> {
    let mut system = System::new();
    system.refresh_memory();
    let free = system.free_memory() as f64;
    let used = system.used_memory() as f64;
    let total = free + used;

    let output = match opt {
        MemoryOpts::PercUsed => (used / total) * 100.0,
        MemoryOpts::PercFree => (free / total) * 100.0,
        MemoryOpts::Used => used,
        MemoryOpts::Free => free,
    }
    .to_string()
    .split('.')
    .next()
    .ok_or("")?
    .into();

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_usage() {
        assert!(memory_usage(&MemoryOpts::Used).is_ok());
        assert!(memory_usage(&MemoryOpts::Free).is_ok());
        assert!(memory_usage(&MemoryOpts::PercUsed).is_ok());
        assert!(memory_usage(&MemoryOpts::PercFree).is_ok());
    }
}
