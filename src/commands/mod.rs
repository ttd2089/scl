pub mod scl;

mod version;
mod changelog;

use std::error::Error;
use std::ffi::OsString;

pub(crate) trait Command<Iter, Item>
where
    Iter: IntoIterator<Item = Item>,
    Item: Into<OsString> + Clone, {

    fn run(&self, itr: Iter) -> Result<(), Box<dyn Error>>;
}

pub(super) struct ClapCommand<Options, BuildFn, ParseFn, RunFn>
where
    BuildFn: Fn() -> clap::Command,
    ParseFn: Fn(&clap::ArgMatches) -> Options,
    RunFn: Fn(&Options, &clap::ArgMatches) -> Result<(), Box<dyn Error>>, {

    build: BuildFn,
    parse: ParseFn,
    run: RunFn,
}

impl<Options, BuildFn, ParseFn, RunFn> ClapCommand<Options, BuildFn, ParseFn, RunFn>
where
    BuildFn: Fn() -> clap::Command,
    ParseFn: Fn(&clap::ArgMatches) -> Options,
    RunFn: Fn(&Options, &clap::ArgMatches)-> Result<(), Box<dyn Error>>, {

    pub(super) fn new(build: BuildFn, parse: ParseFn, run: RunFn) -> Self {
        ClapCommand { build, parse, run }
    }
}

impl<Options, BuildFn, ParseFn, RunFn, Iter, Item> Command<Iter, Item> for ClapCommand<Options, BuildFn, ParseFn, RunFn>
where
    BuildFn: Fn() -> clap::Command,
    ParseFn: Fn(&clap::ArgMatches) -> Options,
    RunFn: Fn(&Options, &clap::ArgMatches) -> Result<(), Box<dyn Error>>, 
    Iter: IntoIterator<Item = Item>,
    Item: Into<OsString> + Clone, {

    fn run(&self, itr: Iter) -> Result<(), Box<dyn Error>> {
        let clap_command = (self.build)();
        let matches = clap_command.try_get_matches_from(itr)?;
        let opts = (self.parse)(&matches);
        (self.run)(&opts, &matches)
    }
}
