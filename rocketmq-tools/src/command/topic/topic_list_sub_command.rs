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
}
