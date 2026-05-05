use anyhow::Result;
use clap::Command;

pub fn build_cli() -> Command {
    Command::new("amid")
        .about("Ami Daemon")
        .subcommand_required(false)
        .arg_required_else_help(false)
        .allow_external_subcommands(true)
    // .subcommand(
    //     Command::new("clean").about("Remove FLAC files generated from format conversion."),
    // )
    // .subcommand(Command::new("log").about("Print the path to the log file."))
    // .subcommand(Command::new("playlist").about("Print the path to playlist files."))
    // .subcommand(Command::new("config").about("Print the path to the configuration file."))
    // .subcommand(
    //     Command::new("with")
    //         .about("Open with paths of audio files or directories enqueued.")
    //         .arg(
    //             Arg::new("paths")
    //                 .help("Paths of directories or audio files, works with glob patterns.")
    //                 .required(true)
    //                 .num_args(1..)
    //                 .value_parser(clap::value_parser!(String)),
    //         ),
    // )
}

/// Handle command line interface commands and return bool of whether the program is to exit.
pub fn handle_cli_commands() -> Result<bool> {
    let matches = build_cli().get_matches();

    match matches.subcommand() {
        Some(("start", _)) => Ok(true),
        // Some(("with", args)) => {
        //     if let Some(path_strs) = args.get_many::<String>("paths") {
        //         let paths: Vec<PathBuf> = path_strs.map(PathBuf::from).collect();
        //         let valid_paths: Vec<PathBuf> =
        //             paths.into_iter().filter(|path| path.exists()).collect();
        //         if valid_paths.is_empty() {
        //             eprintln!("No path is valid.");
        //             Ok(true)
        //         } else {
        //             app.senders
        //                 .msg
        //                 .send(Message::Queue(QueueMessage::QueuePaths(valid_paths)))
        //                 .expect("Error sending queue.");
        //             Ok(false)
        //         }
        //     } else {
        //         eprintln!("No path was given.");
        //         Ok(true)
        //     }
        // }
        _ => Ok(false),
    }
}
