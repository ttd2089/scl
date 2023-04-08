use std::ffi::OsString;


use super::{commands::{Command, ClapCommand}, version::VersionCommand};

// todo: Factor out a trait that hides the arg types for callers to use.

pub struct SclOptions {
    base: String,
    target: String,
}

pub(crate)  fn new<I,T>() -> Box<dyn Command<I,T>> where
I: IntoIterator<Item = T>,
T: Into<OsString> + Clone
{
    let mut puppies = "spooky";

    match puppies {
        "spooky" => Box::new(VersionCommand{}) as Box<dyn Command<I,T>>,
        _ => { Box::new(ClapCommand{
            build_command: || { clap::Command::new("doot")},
            parse_options: |_x| {32},
            run: |_s| {Ok(())}
        }) as Box<dyn Command<I,T>>}
        };
    
    return Box::new(VersionCommand{

    });
}
