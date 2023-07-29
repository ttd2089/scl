use super::command::Command;
use std::{error::Error, ffi::OsString};

/// A wrapper for a [`clap::Command`] that uses strongly-typed
/// 
/// # Type Parameters
/// 
/// * `BuildFn`: The type of the function that builds the [`clap::Command`] the [`Cwrapper`] wraps.
/// 
/// * `Options`: The type that represents the command's options. When the commnand is invoked the provided arguments
///              will be parsed into an instance of `Options`.
/// 
/// * `ParseFn`: The type of the function that maps [`clap::ArgMatches`] into a an instance of `Options`.
/// 
/// * `Context`: The type of the context that will be provided to the command function or subcommand that handles the
///              command invocation. An instance of `Context` is built from the instance of `Options` at runtime before
///              the command function is executed.
/// 
/// * `BuildContextFn`: The type of the function that builds an instance of `Context` from an instance of `Options`.
/// 
/// * `CmdFn`: The type of the function that implements the command.
pub(crate) struct Cwrapper<BuildFn, Options, ParseFn, Context, BuildContextFn, CmdFn>
where
    BuildFn: Fn() -> clap::Command,
    ParseFn: Fn(&clap::ArgMatches) -> Options,
    BuildContextFn: Fn(&Options) -> Result<Context, Box<dyn Error>>,
    CmdFn: Fn(&Context) -> Result<(), Box<dyn Error>>, {

    build: BuildFn,
    parse: ParseFn,
    build_context: BuildContextFn,
    cmd: Option<CmdFn>,
    // subcommands: HashMap<String, Box<dyn ClapSubcommand<Context>>>,
}

impl<BuildFn, Options, ParseFn, Context, BuildContextFn, CmdFn>
Cwrapper<BuildFn, Options, ParseFn, Context, BuildContextFn, CmdFn>
where
    BuildFn: Fn() -> clap::Command,
    ParseFn: Fn(&clap::ArgMatches) -> Options,
    BuildContextFn: Fn(&Options) -> Result<Context, Box<dyn Error>>,
    CmdFn: Fn(&Context)-> Result<(), Box<dyn Error>>, {

    pub(crate) fn new(
        build: BuildFn,
        parse: ParseFn,
        build_context: BuildContextFn,
        cmd: Option<CmdFn>,
    ) -> Self {
        Cwrapper { build, parse, build_context, cmd }
    }

    fn build(&self) -> clap::Command {
        // let mut cmd = (self.build)();
        // for (_, subcommand) in self.subcommands {
        //     cmd.subcommand(subcommand.build());
        // }
        // cmd
        (self.build)()
    }

    fn run(&self, matches: &clap::ArgMatches) -> Result<(), Box<dyn Error>> {

        let opts = (self.parse)(&matches);

        let context = (self.build_context)(&opts)?;
        
        if let Some((subcommand, submatches)) = matches.subcommand() {
            println!("{:#?}: {:#?}", subcommand, submatches);
            return Ok(());
        }

        if let Some(command) = &self.cmd {
            return command(&context);
        }

        unreachable!("no subcommand given");
    }
}

impl<BuildFn, Options, ParseFn, Context, BuildContextFn, CmdFn, Iter, Item> Command<Iter, Item>
for Cwrapper<BuildFn, Options, ParseFn, Context, BuildContextFn, CmdFn>
where
    BuildFn: Fn() -> clap::Command,
    ParseFn: Fn(&clap::ArgMatches) -> Options,
    BuildContextFn: Fn(&Options) -> Result<Context, Box<dyn Error>>,
    CmdFn: Fn(&Context) -> Result<(), Box<dyn Error>>, 
    Iter: IntoIterator<Item = Item>,
    Item: Into<OsString> + Clone, {

    fn run(&self, itr: Iter) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
        let clap_command = self.build();
        match clap_command.try_get_matches_from(itr) {
            Ok(matches) => self.run(&matches),
            Err(_) => Ok(()), // how to map this?
        }
    }
}
