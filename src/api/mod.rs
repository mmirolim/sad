	use std::path::PathBuf;
	use std::process;
	
	#[derive(Debug)]
	pub struct Cli {
		pub file: PathBuf,
		pub sort_by_start_date: bool,
		pub project: Option<String>,
	}

	impl Cli {
		pub fn new(args: &[String]) -> Cli {
		    if args.len() < 2 {
		        // TODO improve message use usage text
		        panic!("not enough arguments");
		    }
		    let mut iter = args.iter();
		    let mut cli = Cli {
		        file: PathBuf::new(),
		        sort_by_start_date: false,
		        project: None,
		    };
		    // skip bin name
		    iter.next();
		    // TODO use consts
		    while let Some(arg) = iter.next() {
		        match arg.as_ref() {
		            "-File" => {
		                if let Some(path) = iter.next() {
		                    cli.file = PathBuf::from(path);                    
		                } else {
		                    println!("input file not defined");
		                    process::exit(1);
		                }

		            },
		            "-SortByStartDate" => {
		                cli.sort_by_start_date = true;
		            },
		            "-Project" => {
		                if let Some(id) = iter.next() {
		                    cli.project = Some(id.to_owned());
		                } else {
		                    panic!("project id missing");
		                }
		            },
		            "-help" => {
		            	let help_message = r#"
	Usage: sad [OPTION]... -File [FILE]...
	Process and transform projects' information in tsv format.

	Mandatory arguments:

	-File <path>             full path to the input file

	Operation modifiers:

	-SortByStartDate         sort results by column "Start date" in ascending order
	-Project <project id>    filter results by column "Project"
		            	"#;
		            	println!("{}", help_message);
		            	process::exit(0);
		            },
		            _ => {
		            	println!("unrecognized option {}", arg);
		            	println!("Try '-help' for more information.");
		            	process::exit(1);
		            },
		        }
		    }
		    
		    cli
		}
	}
