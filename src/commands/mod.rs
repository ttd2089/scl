pub mod scl;

mod version;
mod changelog;

use std::error::Error;
use std::ffi::OsString;
use std::marker::PhantomData;


// Instead of a generic trait, define associated types.
//
// pub(crate) trait Command<Iter, Item>
// where
//     Iter: IntoIterator<Item = Item>,
//     Item: Into<OsString> + Clone, {

//     fn run(&self, itr: Iter) -> Result<(), Box<dyn Error>>;
// }

// problem: It's not clear if/how you can supply these types when a generic type implements the trait.
//
pub(crate) trait Command {
    type Iter: IntoIterator<Item = Self::Item>;
    type Item: Into<OsString> + Clone;

    fn run(&self, itr: Self::Iter) -> Result<(), Box<dyn Error>>;
}

pub(super) struct ClapCommand<Options, BuildFn, ParseFn, RunFn, Iter, Item>
where
    BuildFn: Fn() -> clap::Command,
    ParseFn: Fn(&clap::ArgMatches) -> Options,
    RunFn: Fn(&Options, &clap::ArgMatches) -> Result<(), Box<dyn Error>>,
    Iter: IntoIterator<Item = Item>,
    Item: Into<OsString> + Clone, {

    build: BuildFn,
    parse: ParseFn,
    run: RunFn,

    _phantom: (PhantomData<Iter>, PhantomData<Item>),
}

impl<Options, BuildFn, ParseFn, RunFn, Iter, Item> ClapCommand<Options, BuildFn, ParseFn, RunFn, Iter, Item>
where
    BuildFn: Fn() -> clap::Command,
    ParseFn: Fn(&clap::ArgMatches) -> Options,
    RunFn: Fn(&Options, &clap::ArgMatches)-> Result<(), Box<dyn Error>>,
    Iter: IntoIterator<Item = Item>,
    Item: Into<OsString> + Clone, {

    pub(super) fn new(build: BuildFn, parse: ParseFn, run: RunFn) -> Self {
        ClapCommand { build, parse, run, _phantom: (PhantomData, PhantomData) }
    }
}

// attempt:
//
//     impl<Options, BuildFn, ParseFn, RunFn, Iter, Item> Command<Iter, Item> for ClapCommand<Options, BuildFn, ParseFn, RunFn>
//     where
//         BuildFn: Fn() -> clap::Command,
//         ParseFn: Fn(&clap::ArgMatches) -> Options,
//         RunFn: Fn(&Options, &clap::ArgMatches) -> Result<(), Box<dyn Error>>, 
//         Iter: IntoIterator<Item = Item>,
//         Item: Into<OsString> + Clone, {
//     
//         fn run(&self, itr: Iter) -> Result<(), Box<dyn Error>> {
//             let clap_command = (self.build)();
//             let matches = clap_command.try_get_matches_from(itr)?;
//             let opts = (self.parse)(&matches);
//             (self.run)(&opts, &matches)
//         }
//     }
//
//     (signature from working version with generic trait)
//
// problem:
//
//     This doesn't work because Command is not generic, i.e. Command<Iter, Item> is not valid. Makes sense.
//
// error:
//
//     error[E0107]: this trait takes 0 generic arguments but 2 generic arguments were supplied
//       --> src\commands\mod.rs:51:52
//        |
//     51 | impl<Options, BuildFn, ParseFn, RunFn, Iter, Item> Command<Iter, Item> for ClapCommand<Options, BuildFn, ParseFn, RunFn>
//        |                                                    ^^^^^^^ expected 0 generic arguments
//        |
//     note: trait defined here, with 0 generic parameters
//       --> src\commands\mod.rs:22:18
//        |
//     22 | pub(crate) trait Command {
//        |                  ^^^^^^^
//     help: replace the generic bounds with the associated types
//        |
//     51 | impl<Options, BuildFn, ParseFn, RunFn, Iter, Item> Command<Iter = Iter, Item = Item> for ClapCommand<Options, BuildFn, ParseFn, RunFn>
//        |                                                            ++++++       ++++++
//
//     For more information about this error, try `rustc --explain E0107`.

// attempt:
//
//      impl<Options, BuildFn, ParseFn, RunFn, Iter, Item> Command<Iter = Iter, Item = Item> for ClapCommand<Options, BuildFn, ParseFn, RunFn>
//      where
//          BuildFn: Fn() -> clap::Command,
//          ParseFn: Fn(&clap::ArgMatches) -> Options,
//          RunFn: Fn(&Options, &clap::ArgMatches) -> Result<(), Box<dyn Error>>, 
//          Iter: IntoIterator<Item = Item>,
//          Item: Into<OsString> + Clone, {
//       
//          fn run(&self, itr: Iter) -> Result<(), Box<dyn Error>> {
//              let clap_command = (self.build)();
//              let matches = clap_command.try_get_matches_from(itr)?;
//              let opts = (self.parse)(&matches);
//              (self.run)(&opts, &matches)
//          }
//      }
//
//      (suggestion from attempt #1)
//
// problem:
//
//     This one is surprising because it's a suggestion from the compiler but the error says it's not valid syntax.
//     Maybe it's something about name collisions with the generic arg in the impl<> and the associated type name, or
//     because the types for the params aren't specified by type args on the generic struct?
//
// error:
//
//     error[E0229]: associated type bindings are not allowed here
//       --> src\commands\mod.rs:95:60
//        |
//     95 | impl<Options, BuildFn, ParseFn, RunFn, Iter, Item> Command<Iter = Iter, Item = Item> for ClapCommand<Options, BuildFn, ParseFn, RunFn>
//        |                                                            ^^^^^^^^^^^ associated type not allowed here
//     
//     For more information about this error, try `rustc --explain E0229`.

// attempt:
//
//     impl<Options, BuildFn, ParseFn, RunFn, I, T> Command<Iter = I, Item = T> for ClapCommand<Options, BuildFn, ParseFn, RunFn>
//     where
//         BuildFn: Fn() -> clap::Command,
//         ParseFn: Fn(&clap::ArgMatches) -> Options,
//         RunFn: Fn(&Options, &clap::ArgMatches) -> Result<(), Box<dyn Error>>, 
//         I: IntoIterator<Item = T>,
//         T: Into<OsString> + Clone, {
//     
//         fn run(&self, itr: I) -> Result<(), Box<dyn Error>> {
//             let clap_command = (self.build)();
//             let matches = clap_command.try_get_matches_from(itr)?;
//             let opts = (self.parse)(&matches);
//             (self.run)(&opts, &matches)
//         }
//     }
//
//     (Renaming type params in impl<> to differ from associated types)
//
//
// problem:
//
//     Same as attempt #2, the problem isn't names colliding.
//
// error:
//
//     error[E0229]: associated type bindings are not allowed here
//        --> src\commands\mod.rs:131:54
//         |
//     131 | impl<Options, BuildFn, ParseFn, RunFn, I, T> Command<Iter = I, Item = T> for ClapCommand<Options, BuildFn, ParseFn, RunFn>
//         |                                                      ^^^^^^^^ associated type not allowed here
//     
//     For more information about this error, try `rustc --explain E0229`.

// attempt:
//
//     pub(super) struct ClapCommand<Options, BuildFn, ParseFn, RunFn, Iter, Item>
//     where
//         BuildFn: Fn() -> clap::Command,
//         ParseFn: Fn(&clap::ArgMatches) -> Options,
//         RunFn: Fn(&Options, &clap::ArgMatches) -> Result<(), Box<dyn Error>>,
//         Iter: IntoIterator<Item = Item>,
//         Item: Into<OsString> + Clone, {
//     
//         build: BuildFn,
//         parse: ParseFn,
//         run: RunFn,
//     }
//     
//     impl<Options, BuildFn, ParseFn, RunFn, Iter, Item> ClapCommand<Options, BuildFn, ParseFn, RunFn, Iter, Item>
//     where
//         BuildFn: Fn() -> clap::Command,
//         ParseFn: Fn(&clap::ArgMatches) -> Options,
//         RunFn: Fn(&Options, &clap::ArgMatches)-> Result<(), Box<dyn Error>>,
//         Iter: IntoIterator<Item = Item>,
//         Item: Into<OsString> + Clone, {
//     
//         pub(super) fn new(build: BuildFn, parse: ParseFn, run: RunFn) -> Self {
//             ClapCommand { build, parse, run }
//         }
//     }
//     
//     impl<Options, BuildFn, ParseFn, RunFn, Iter, Item> Command<Iter = Iter, Item = Item> for ClapCommand<Options, BuildFn, ParseFn, RunFn, Iter, Item>
//     where
//         BuildFn: Fn() -> clap::Command,
//         ParseFn: Fn(&clap::ArgMatches) -> Options,
//         RunFn: Fn(&Options, &clap::ArgMatches) -> Result<(), Box<dyn Error>>,
//         Iter: IntoIterator<Item = Item>,
//         Item: Into<OsString> + Clone, {
//              
//         fn run(&self, itr: Iter) -> Result<(), Box<dyn Error>> {
//             let clap_command = (self.build)();
//             let matches = clap_command.try_get_matches_from(itr)?;
//             let opts = (self.parse)(&matches);
//             (self.run)(&opts, &matches)
//         }
//     }
//
//     (Adding type params to generic struct)
//
// problem:
//
//     Same as attempt #2, the problem isn't that the generic struct doesn't have the params.
//
// error:
//
//     error[E0229]: associated type bindings are not allowed here
//        --> src\commands\mod.rs:167:60
//         |
//     170 | impl<Options, BuildFn, ParseFn, RunFn, Iter, Item> Command<Iter = Iter, Item = Item> for ClapCommand<Options, BuildFn, ParseFn, RunFn, It...
//         |                                                            ^^^^^^^^^^^ associated type not allowed here
//     
//     For more information about this error, try `rustc --explain E0229`.

impl<Options, BuildFn, ParseFn, RunFn, Iter, Item> Command for ClapCommand<Options, BuildFn, ParseFn, RunFn, Iter, Item>
where
    BuildFn: Fn() -> clap::Command,
    ParseFn: Fn(&clap::ArgMatches) -> Options,
    RunFn: Fn(&Options, &clap::ArgMatches) -> Result<(), Box<dyn Error>>,
    Iter: IntoIterator<Item = Item>,
    Item: Into<OsString> + Clone, {

    type Iter = Iter;
    type Item = Item;

    fn run(&self, itr: Iter) -> Result<(), Box<dyn Error>> {
        let clap_command = (self.build)();
        let matches = clap_command.try_get_matches_from(itr)?;
        let opts = (self.parse)(&matches);
        (self.run)(&opts, &matches)
    }
}
