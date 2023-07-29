// use conventional::Simple;

// pub(super) fn build_subcommand() -> clap::Command {
//     clap::Command::new("version")
//         .about("Generates the version string")
//         .arg(clap::arg!(-v --"base-version" <BASE_VERSION> "The version to increment based on the commit types.")
//             .value_parser(VersionTagParser{})
//             .required(true))
// }

// pub(super) fn run(matches: &clap::ArgMatches, commits: &Vec<conventional::Commit>) {
//     let base_version = matches.get_one::<semver::Version>("base-version").unwrap();
//     println!("{}", get_next_version(base_version, commits));
// }
    
// fn get_next_version(base_version: &semver::Version, commits: &Vec<conventional::Commit>) -> semver::Version {
    
//     if base_version.major > 0 && commits.iter().any(|x| x.breaking()) {
//         return semver::Version::new(base_version.major + 1, 0, 0);
//     }
//     if commits.iter().any(|x| x.type_() == "feat") {
//         return semver::Version::new(base_version.major, base_version.minor + 1, 0);
//     }
//     if commits.iter().any(|x| x.type_() == "fix") {
//         return semver::Version::new(base_version.major, base_version.minor, base_version.patch + 1);
//     }
//     return base_version.to_owned();
// }

// #[derive(Clone)]
// struct VersionTagParser;

// impl clap::builder::TypedValueParser for VersionTagParser {

//     type Value = semver::Version;

//     fn parse_ref(
//         &self,
//         cmd: &clap::Command,
//         arg: Option<&clap::Arg>,
//         value: &std::ffi::OsStr,
//     ) -> Result<Self::Value, clap::Error> {
        
//         let arg_value = value.to_string_lossy();

//         semver::Version::parse(&arg_value).map_err(|x| {

//             let mut err = clap::Error::new(clap::error::ErrorKind::ValueValidation).with_cmd(cmd);
//             if let Some(arg) = arg {
//                 err.insert(clap::error::ContextKind::InvalidArg, clap::error::ContextValue::String(arg.to_string()));
//             }
//             err.insert(clap::error::ContextKind::InvalidValue, clap::error::ContextValue::String(arg_value.to_string()));
//             err.insert(clap::error::ContextKind::Custom, clap::error::ContextValue::String(x.to_string()));
//             err
//         })
//     }
// }
