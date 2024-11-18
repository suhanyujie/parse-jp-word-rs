use crate::prelude::*;

pub mod dir_entry_from;

pub fn lib1() -> Result<String> {
    Ok("lib1 is called...".into())
}
