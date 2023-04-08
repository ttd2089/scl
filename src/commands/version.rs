use std::ffi::OsString;

// use clap::Command;
use super::commands::Command;



pub struct VersionCommand {

}

impl<I,T> Command<I,T> for VersionCommand where
I: IntoIterator<Item = T>,
T: Into<OsString> + Clone
{
    fn run(&self, _itr: I) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}

