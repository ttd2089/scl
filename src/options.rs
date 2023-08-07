use std::{error::Error, ffi::OsString};

use clap::{Arg, ColorChoice};

pub(super) struct SclOptions {
    pub base_ref: String,
    pub base_version: semver::Version,
    pub target_ref: String,
    pub strict: bool,
    pub repo_path: String,
}

pub(super) fn get_options<Iter, Item>(itr: Iter) -> Result<SclOptions, Box<dyn Error>>
where
    Iter: IntoIterator<Item = Item>,
    Item: Into<OsString> + Clone,
{
    let cmd = clap::command!()
        .color(ColorChoice::Auto)
        .arg(
            Arg::new("base-ref")
                .long("base-ref")
                .value_name("GIT_REF")
                .help("The git ref to compare the target ref against")
                .required(true),
        )
        .arg(
            Arg::new("base-version")
                .long("base-version")
                .value_name("SEMVER")
                .help("The semantic version of --base-ref")
                .required(true)
                .value_parser(SemverParser {}),
        )
        .arg(
            Arg::new("target-ref")
                .long("target-ref")
                .value_name("GIT_REF")
                .help("The git ref to generate release info for")
                .default_value("HEAD"),
        )
        .arg(
            Arg::new("strict")
                .long("strict")
                .value_parser(["false", "true"])
                .help("Causes SCL to fail on unconventional commits")
                .num_args(0..=1)
                .default_missing_value("true")
                .require_equals(true)
                .default_value("false"),
        )
        .arg(
            Arg::new("repo-path")
                .value_name("REPO_PATH")
                .help("The path to the Git repository to generate the changelog for")
                .default_value("."),
        )
        .propagate_version(true)
        .arg_required_else_help(true)
        .max_term_width(100);

    let matches = cmd.try_get_matches_from(itr)?;

    Ok(SclOptions {
        base_ref: matches.get_one::<String>("base-ref").unwrap().to_owned(),
        base_version: matches
            .get_one::<semver::Version>("base-version")
            .unwrap()
            .clone(),
        target_ref: matches.get_one::<String>("target-ref").unwrap().to_owned(),
        strict: matches.get_one::<String>("strict").unwrap() == "true",
        repo_path: matches.get_one::<String>("repo-path").unwrap().to_owned(),
    })
}

#[cfg(test)]
mod test_get_options {

    use super::get_options;

    #[test]
    fn base_ref_is_required() {
        let args = vec!["scl", "--base-version=1.2.4"];

        let err = get_options(args)
            .err()
            .unwrap()
            .downcast::<clap::error::Error>()
            .unwrap();

        assert_eq!(err.kind(), clap::error::ErrorKind::MissingRequiredArgument);
        let invalid_arg = err
            .context()
            .find_map(|x| match x {
                (clap::error::ContextKind::InvalidArg, clap::error::ContextValue::Strings(x)) => {
                    Some(x)
                }
                _ => None,
            })
            .unwrap();
        assert_eq!(invalid_arg[0], "--base-ref <GIT_REF>");
    }

    #[test]
    fn base_version_is_required() {
        let args = vec!["scl", "--base-ref=HEAD~4"];

        let err = get_options(args)
            .err()
            .unwrap()
            .downcast::<clap::error::Error>()
            .unwrap();

        assert_eq!(err.kind(), clap::error::ErrorKind::MissingRequiredArgument);
        let invalid_arg = err
            .context()
            .find_map(|x| match x {
                (clap::error::ContextKind::InvalidArg, clap::error::ContextValue::Strings(x)) => {
                    Some(x)
                }
                _ => None,
            })
            .unwrap();
        assert_eq!(invalid_arg[0], "--base-version <SEMVER>")
    }

    #[test]
    fn base_version_must_be_a_semver() {
        let args = vec!["scl", "--base-ref=HEAD~4", "--base-version=1.2.3.4.5"];

        let err = get_options(args)
            .err()
            .unwrap()
            .downcast::<clap::error::Error>()
            .unwrap();

        assert_eq!(err.kind(), clap::error::ErrorKind::ValueValidation);
        let invalid_arg = err
            .context()
            .find_map(|x| match x {
                (clap::error::ContextKind::InvalidArg, clap::error::ContextValue::String(x)) => {
                    Some(x)
                }
                _ => None,
            })
            .unwrap();
        assert_eq!(invalid_arg, "--base-version <SEMVER>");
        let invalid_arg = err
            .context()
            .find_map(|x| match x {
                (clap::error::ContextKind::InvalidValue, clap::error::ContextValue::String(x)) => {
                    Some(x)
                }
                _ => None,
            })
            .unwrap();
        assert_eq!(invalid_arg, "1.2.3.4.5")
    }

    #[test]
    fn option_values_are_set() {
        let base_ref = "HEAD~4";
        let base_version = semver::Version::parse("1.2.4").unwrap();
        let target_ref = "HEAD~1";
        let repo_path = "/src/github.com/ttd2089/scl";
        let args = vec![
            "scl".to_owned(),
            format!("--base-ref={}", base_ref),
            format!("--base-version={}", base_version),
            format!("--target-ref={}", target_ref),
            repo_path.to_owned(),
        ];

        let options = get_options(args).unwrap();

        assert_eq!(options.base_ref, base_ref);
        assert_eq!(options.base_version, base_version);
        assert_eq!(options.target_ref, target_ref);
        assert_eq!(options.repo_path, repo_path);
    }

    #[test]
    fn target_ref_defaults_to_head() {
        let args = vec!["scl", "--base-ref=HEAD~4", "--base-version=1.2.4"];

        let options = get_options(args).unwrap();

        assert_eq!("HEAD", options.target_ref);
    }

    #[test]
    fn repo_path_defaults_to_dot() {
        let args = vec!["scl", "--base-ref=HEAD~4", "--base-version=1.2.4"];

        let options = get_options(args).unwrap();

        assert_eq!(".", options.repo_path);
    }

    #[test]
    fn strict_defaults_to_false() {
        let args = vec!["scl", "--base-ref=HEAD~4", "--base-version=1.2.4"];

        let options = get_options(args).unwrap();

        assert_eq!(false, options.strict);
    }

    #[test]
    fn strict_default_value_for_flag_is_true() {
        let args = vec![
            "scl",
            "--base-ref=HEAD~4",
            "--base-version=1.2.4",
            "--strict",
        ];

        let options = get_options(args).unwrap();

        assert_eq!(true, options.strict);
    }

    #[test]
    fn strict_accepts_explict_false() {
        let args = vec![
            "scl",
            "--base-ref=HEAD~4",
            "--base-version=1.2.4",
            "--strict=false",
        ];

        let options = get_options(args).unwrap();

        assert_eq!(false, options.strict);
    }

    #[test]
    fn strict_accepts_explict_true() {
        let args = vec![
            "scl",
            "--base-ref=HEAD~4",
            "--base-version=1.2.4",
            "--strict=true",
        ];

        let options = get_options(args).unwrap();

        assert_eq!(true, options.strict);
    }

    #[test]
    fn strict_must_be_true_or_false() {
        let args = vec![
            "scl",
            "--base-ref=HEAD~4",
            "--base-version=1.2.4",
            "--strict=maybe",
        ];

        let err = get_options(args)
            .err()
            .unwrap()
            .downcast::<clap::error::Error>()
            .unwrap();

        assert_eq!(err.kind(), clap::error::ErrorKind::InvalidValue);
        let invalid_arg = err
            .context()
            .find_map(|x| match x {
                (clap::error::ContextKind::InvalidArg, clap::error::ContextValue::String(x)) => {
                    Some(x)
                }
                _ => None,
            })
            .unwrap();
        assert_eq!(invalid_arg, "--strict[=<strict>]");
        let invalid_value = err
            .context()
            .find_map(|x| match x {
                (clap::error::ContextKind::InvalidValue, clap::error::ContextValue::String(x)) => {
                    Some(x)
                }
                _ => None,
            })
            .unwrap();
        assert_eq!(invalid_value, "maybe");
    }
}

#[derive(Clone)]
struct SemverParser;

impl clap::builder::TypedValueParser for SemverParser {
    type Value = semver::Version;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let arg_value = value.to_string_lossy();

        semver::Version::parse(&arg_value).map_err(|x| {
            let mut err = clap::Error::new(clap::error::ErrorKind::ValueValidation).with_cmd(cmd);
            if let Some(arg) = arg {
                err.insert(
                    clap::error::ContextKind::InvalidArg,
                    clap::error::ContextValue::String(arg.to_string()),
                );
            }
            err.insert(
                clap::error::ContextKind::InvalidValue,
                clap::error::ContextValue::String(arg_value.into_owned()),
            );
            err.insert(
                clap::error::ContextKind::Custom,
                clap::error::ContextValue::String(x.to_string()),
            );
            err
        })
    }
}
