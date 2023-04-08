use std::error::Error;
use std::ffi::OsString;

pub(crate) trait Command<I,T>     where
I: IntoIterator<Item = T>,
T: Into<OsString> + Clone
{
    fn run(&self, itr: I) -> Result<(), Box<dyn Error>>;
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

    // Just making pub for now 
    pub build_command: BuildFn,
    pub parse_options: ParseFn,
    pub run: RunFn,
}

impl<I,T,O,B,P,R> Command<I,T> for ClapCommand<O,B,P,R>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
    B: Fn() -> clap::Command,
    P: Fn(&clap::ArgMatches) -> O,
    R: Fn(&O) -> Result<(), Box<dyn Error>>
{
    fn run(&self, iter: I) -> Result<(), Box<dyn Error>> {
        let clap_command = (self.build_command)();
        let matches = clap_command.try_get_matches_from(iter)?;
        let opts = (self.parse_options)(&matches);
        (self.run)(&opts)
    }
}
