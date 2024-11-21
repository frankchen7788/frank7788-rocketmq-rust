/*
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::env;
use std::path::PathBuf;
use std::ptr::null;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use clap::builder::Str;
use clap::Parser;
use rocketmq_common::common::namesrv::namesrv_config::NamesrvConfig;
use rocketmq_common::common::server::config::ServerConfig;
use rocketmq_common::utils::network_util::NetworkUtil;
use rocketmq_common::EnvUtils::EnvUtils;
use rocketmq_common::ParseConfigFile;
use rocketmq_rust::rocketmq;
use rocketmq_tools::command::sub_command::Subcommand;
use rocketmq_tools::command::topic::topic_list_sub_command::TopicListSubCommand;
use tracing::info;

static mut SUBCOMMANDS: Vec<Box<dyn Subcommand>> = Vec::new();

fn init_command(item: Box<dyn Subcommand>) {
    unsafe { SUBCOMMANDS.push(item) }; // 添加内容
}

fn init_all_command() {
    init_command(Box::new(TopicListSubCommand {}));
    /*
    unsafe {
        if let Some(itemss) = SUBCOMMANDS.get(0) {
            itemss.excute();
        }
    }*/
}

fn find_sub_command(arg: &str) -> Option<&Box<dyn Subcommand>> {
    unsafe {
        for i in 0..SUBCOMMANDS.len() {
            if let Some(scmd) = SUBCOMMANDS.get(i) {
                if scmd.command_name().eq(arg) {
                    return Some(scmd);
                }
            }
        }
    }
    None
}

fn parse_sub_args(args: &[String]) -> Option<Vec<String>> {  
    if args.len() > 1 {  
        let result: Vec<String> = args[1..].to_vec(); // 从索引 1 开始切片，并转换为 Vec  
        return Some(result);  
    }  
    None // 如果长度不大于 1，返回 None  
}  


#[rocketmq::main]
async fn main() -> anyhow::Result<()> {
    rocketmq_common::log::init_logger();

    let args = vec!["arg0".to_string(), "arg1".to_string(), "arg2".to_string()];  
    match parse_sub_args(&args) {  
        Some(sub_args) => println!("{:?}", sub_args),  
        None => println!("No sub arguments found."),  
    }  


    //let args = Args::parse();
    let ROCKETMQ_HOME = EnvUtils::get_rocketmq_home();

    // 获取命令行参数的向量
    let args: Vec<String> = env::args().collect();

    let argNum = args.len();

    println!("args num== {}", argNum);
    //the first arg is program itself.

    init_all_command();

    match argNum {
        1 => print_help(),
        3 => {
            if args[1] == "help" {
                if let Some(sumcommand) = find_sub_command("topicList") {
                    //build options and print help
                } else {
                    println!("The sub command {} not exist.", args[2])
                }
            }
        }
        _ => {
            println!("more than 3 args");

            //parse sub arg

            if let Some(sumcommand) = find_sub_command("topicList") {
                //build options and excute
                sumcommand.excute();
            } else {
                println!("The sub command {} not exist.", args[1])
            }
        }
    }

    Ok(())
}

fn print_help() {
    println!("The most commonly used mqadmin commands are:");

    println!("See 'mqadmin help <command>' for more information on a specific command.");
}

#[derive(Parser, Debug)]
#[command(author = "mxsm", version = "0.2.0", about = "RocketMQ  mqadmin(Rust)")]
struct Args {
    /// rocketmq name remoting_server port
    #[arg(
        short,
        long,
        value_name = "PORT",
        default_missing_value = "9876",
        default_value = "9876",
        required = false
    )]
    port: u32,

    /// rocketmq name remoting_server ip
    #[arg(
        short,
        long,
        value_name = "IP",
        default_value = "0.0.0.0",
        required = false
    )]
    ip: String,
    /// rocketmq name remoting_server config file
    #[arg(short, long, value_name = "FILE", default_missing_value = "None")]
    config: Option<PathBuf>,
}
