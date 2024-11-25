use clap::ArgMatches;
pub trait Subcommand{
    fn command_name(&self) -> &str; 
    fn command_alias(&self) -> Option<&str> {  
        None  
    } 
    fn command_desc(&self) -> &str;  
    
    fn excute(&self);

    fn get_arg_matchs(&self) -> ArgMatches;

}