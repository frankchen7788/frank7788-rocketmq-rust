pub trait Subcommand{
    fn command_name(&self) -> &str; 
    fn command_alias(&self) -> Option<&str> {  
        None  
    } 
    fn command_desc(&self) -> &str;  
    
    fn excute(&self);

}