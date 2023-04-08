mod commands;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let command = commands::scl::new();
    let res = command.run(std::env::args_os());
    res.into()
}



// use conventional::Simple;

// fn main() -> Result<(), Box<dyn std::error::Error>> {

//     let matches = build_command().get_matches();

//     let base_ref = matches.get_one::<String>("base").unwrap();
//     let target_ref = matches.get_one::<String>("target").unwrap();
//     let repo = git2::Repository::discover(".")?;

//     let mut revwalk = repo.revwalk()?;
//     revwalk.push_range(&format!("{}..{}", base_ref, target_ref))?;

//     let commits = revwalk
//         .collect::<Result<Vec<git2::Oid>, _>>()?
//         .iter()
//         .map(|x| repo.find_commit(*x))
//         .collect::<Result<Vec<git2::Commit>, _>>()?;

//     let conventional_commits = commits
//         .iter()
//         .map(|x| match x.message() {
//             // A subject with trailing newlines is technically not a valid conventional commit
//             // because it has an empty body instead of no body -- we can be more lenient than that.
//             Some(x) => Ok(x.trim_end_matches(&['\r', '\n'])),
//             _ => Err(format!("invalid commit {}: message it not a valid UTF-8 atring", x.id())),
//         })
//         .collect::<Result<Vec<&str>, _>>()?
//         .iter()
//         .filter_map(|x| conventional::Commit::new(x).ok())
//         .collect();
    
//     match matches.subcommand() {
//         Some(("version", matches)) => version_command(matches, &conventional_commits),
//         Some(("changelog", matches)) => changelog_command(matches, &conventional_commits),
//         _ => unreachable!("Oh nooo"),
//     };

//     Ok(())
// }

// fn build_command() -> clap::Command {
//     clap::command!()
//         .arg(clap::arg!(--base <BASE_REF> "The git ref to compare the target ref against.").required(true))
//         .arg(clap::arg!(--target <TARGET_REF> "The git ref to generate release info for.").default_value("HEAD"))
//         // todo: investigate the impact of propagate_version -- do subcommands need to have versions?
//         .subcommand(
//             clap::Command::new("version")
//                 .about("Generates the version string")
//                 .arg(clap::arg!(-v --"base-version" <BASE_VERSION> "The version to increment based on the commit types.")
//                     .value_parser(VersionTagParser{})
//                     .required(true))
//         )
//         .subcommand(
//             clap::Command::new("changelog")
//                 .about("Generates the changelog")
//                 .arg(clap::arg!(-f --format <FORMAT> "The format to output the changelog in.")
//                     .value_parser(clap::builder::PossibleValuesParser::new(["markdown", "json"]))
//                     .default_value("markdown"))
//         )
//         .arg_required_else_help(true)
//         .max_term_width(100)
//         .help_expected(true)
// }

// fn version_command(matches: &clap::ArgMatches, commits: &Vec<conventional::Commit>) {

//     let base_version = matches.get_one::<semver::Version>("base-version").unwrap();
//     println!("{}", get_next_version(base_version, commits));
// }

// fn get_next_version(base_version: &semver::Version, commits: &Vec<conventional::Commit>) -> semver::Version {
    
//     if commits.iter().any(|x| x.breaking()) {
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


// fn changelog_command(matches: &clap::ArgMatches, commits: &Vec<conventional::Commit>) {
    
// }
