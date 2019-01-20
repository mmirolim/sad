use std::path::PathBuf;

#[derive(Debug)]
pub struct Cli {
    pub file: PathBuf,
    pub sort_by_start_date: bool,
    pub project: Option<String>,
}

impl Cli {
    pub fn new(args: &[String]) -> Result<Cli, &str> {
        let help_msg = r#"
Usage: sad [OPTION]... -File [FILE]...
Process and transform projects' information in tsv format.

Mandatory arguments:

-File <path>             full path to the input file

Operation modifiers:

-SortByStartDate         sort results by column "Start date" in ascending order
-Project <project id>    filter results by column "Project""#;
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let mut cli = Cli {
            file: PathBuf::new(),
            sort_by_start_date: false,
            project: None,
        };

        let mut iter = args.iter();
        // skip bin name
        iter.next();
        while let Some(arg) = iter.next() {
            match arg.as_ref() {
                "-File" => {
                    if let Some(path) = iter.next() {
                        cli.file = PathBuf::from(path);
                    } else {
                        return Err("input file not defined");
                    }
                }
                "-SortByStartDate" => {
                    cli.sort_by_start_date = true;
                }
                "-Project" => {
                    if let Some(id) = iter.next() {
                        cli.project = Some(id.to_owned());
                    } else {
                        return Err("project id missing");
                    }
                }
                "-help" => {
                    return Err(help_msg);
                }
                _ => {
                    return Err("unrecognized option {}\nTry '-help' for more information.");
                }
            }
        }

        Ok(cli)
    }
}
