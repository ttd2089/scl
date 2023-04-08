use conventional::Simple;

pub(super) fn build_subcommand() -> clap::Command {
    clap::Command::new("changelog")
        .about("Generates the changelog")
        .arg(clap::arg!(-f --format <FORMAT> "The format to output the changelog in.")
            .value_parser(clap::builder::PossibleValuesParser::new(["markdown", "json"]))
            .default_value("markdown"))
}

pub(super) fn run(matches: &clap::ArgMatches, commits: &Vec<conventional::Commit>) {

    let categories = categorize_commits(commits);
    match matches.get_one::<String>("format").unwrap().as_str() {
        "markdown" => format_markdown(&categories),
        "json" => format_json(&categories),
        x => unreachable!("unimplemented format type '{}'", x),
    };
}

struct Category<'a> {
    pub name: String,
    pub commits: Vec<&'a conventional::Commit<'a>>,
}

// todo: Use a config file to specify commit type to section name mappings.
fn categorize_commits<'a>(commits: &'a Vec<conventional::Commit<'a>>) -> Vec<Category<'a>> {

    let breaking_changes = commits
        .iter()
        .filter(|x| x.breaking())
        .collect::<Vec<_>>();
    
    let features = commits
        .iter()
        .filter(|x| !x.breaking() && x.type_() == "feat")
        .collect::<Vec<_>>();
    
    let fixes = commits
        .iter()
        .filter(|x| !x.breaking() && x.type_() == "fix")
        .collect::<Vec<_>>();

    vec![
        Category{ name: "BREAKING CHANGES".to_string(), commits: breaking_changes, },
        Category{ name: "Features".to_string(), commits: features, },
        Category{ name: "Bug Fixes".to_string(), commits: fixes, },
    ]
        .into_iter()
        .filter(|x| x.commits.len() > 0)
        .collect()
}

fn format_markdown(categories: &Vec<Category>) {

    let mut first = true;

    for category in categories {
        
        if !first {
            println!("")
        }

        println!("### {}\n", category.name);
        for commit in &category.commits {
            println!("- **{}:** {}", commit.type_(), commit.description());
        }
        
        first = false;
    }
}

fn format_json(categories: &Vec<Category>) {
    let mut first = true;

    print!("[");
    for category in categories {
        
        if !first {
            print!(",")
        }

        print!("{{\"name\": \"{}\",\"commits\":[", category.name);

        {
            let mut first = true;
            for commit in &category.commits {

                if !first {
                    print!(",")
                }

                print!("{{\"type\": \"{}\", \"subject\": \"{}\"}}", commit.type_(), commit.description());

                first = false;
            }
        }

        print!("]}}");
        
        first = false;
    }
    print!("]");
}

