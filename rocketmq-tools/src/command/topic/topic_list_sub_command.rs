use clap::arg;
use clap::command;
use clap::value_parser;
use clap::Arg;
use clap::ArgMatches;
use clap::Command;

use crate::command::sub_command::Subcommand;

pub struct TopicListSubCommand {}

//subcommand for mqadmin topicList
impl Subcommand for TopicListSubCommand {
    fn command_name(&self) -> &str {
        "topicList"
    }

    fn command_desc(&self) -> &str {
        "Fetch all topic list from name server."
    }

    fn excute(&self) {
        println!(
            "{:<20}  {:<48}  {:<48}",
            "#Cluster Name", "#Topic", "#Consumer Group"
        );
        // todo!()
    }

    fn get_arg_matchs(&self) -> ArgMatches {
        let matches =
            Command::new("topicList")
                .subcommands([Command::new("topicList")
                    .arg(
                        Arg::new("cluster")
                            .action(clap::ArgAction::SetTrue) // Action when flag is used
                            .short('c')
                            .long("clusterModel")
                            .help("clusterModel"),
                    )
                    .arg(Arg::new("namesrv").short('n').long("namesrvAddr").help(
                        "Name server address list, eg: '192.168.0.1:9876;192.168.0.2:9876'",
                    ))])
                .help_template("{name} ({version}) - {usage}")
                .get_matches();
        matches
    }
}
