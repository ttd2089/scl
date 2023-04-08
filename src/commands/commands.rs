use std::error::Error;
use std::ffi::OsString;

pub(crate) trait Command {

    fn run<I, T>(&self, itr: I) -> ResultFromThoseBois<I, T>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone;
}

pub(crate) struct ResultFromThoseBois<I, T>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone {

    itr: I,
}

impl<I, T> Into<Result<(), Box<dyn Error>>> for ResultFromThoseBois<I, T>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone {
        
    fn into(self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

pub(super) struct ClapCommand<Options, BuildFn, ParseFn, RunFn>
where
    BuildFn: Fn() -> clap::Command,
    ParseFn: Fn(&clap::ArgMatches) -> Options,
    RunFn: Fn(&Options) -> Result<(), Box<dyn Error>> {

    build_command: BuildFn,
    parse_options: ParseFn,
    run: RunFn,
}

impl<Options, BuildFn, ParseFn, RunFn> Command for ClapCommand<Options, BuildFn, ParseFn, RunFn>
where
    BuildFn: Fn() -> clap::Command,
    ParseFn: Fn(&clap::ArgMatches) -> Options,
    RunFn: Fn(&Options) -> Result<(), Box<dyn Error>> {

    fn run<I, T>(&self, itr: I) -> Result<(), Box<dyn Error>> {
            
        let clap_command = (self.build_command)();
        let matches = clap_command.try_get_matches_from(itr)?;
        let opts = (self.parse_options)(&matches);
        (self.run)(&opts)
    }
}
