use clap::{ Arg, App };

pub struct Command {
    pub with_password: bool,
    pub report: String,
}

pub fn read_command() -> Command {
    let matches = App::new("jiraq")
        .version("0.1.0")
        .author("J/A <archer884@gmail.com>")
        .about("Queries Jira if you're nice enough")
        .arg(Arg::with_name("REPORT")
            .help("sets the report to run")
            .required(true)
            .index(1))
        .arg(Arg::with_name("password")
            .short("p")
            .help("accept password from std in"))
        .get_matches();

    Command {
        with_password: matches.is_present("password"),
        report: matches.value_of("REPORT").unwrap().to_owned(),
    }
}
