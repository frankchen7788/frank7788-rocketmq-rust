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

use clap::Parser;
use rocketmq_common::common::namesrv::namesrv_config::NamesrvConfig;
use rocketmq_common::common::server::config::ServerConfig;
use rocketmq_common::EnvUtils::EnvUtils;
use rocketmq_common::ParseConfigFile;
use rocketmq_rust::rocketmq;
use tracing::info;

#[rocketmq::main]
async fn main() -> anyhow::Result<()> {
    rocketmq_common::log::init_logger();
    //let args = Args::parse();
    let home = EnvUtils::get_rocketmq_home();

    // 获取命令行参数的向量
    let args: Vec<String> = env::args().collect();

    let argNum = args.len();
     
    println!("args num== {}",argNum);
    //the first arg is program itself.

    match argNum {
        1 => print_help(),
        3 => {
            if args[1] == "help" {
                println!("The sub command help")
            } else {
                println!("The sub command {} not exist.", args[1])
            }
        }
        _ => {
            println!("more than 3 args")
        }
    }

    Ok(())
}

fn print_help() {
    println!("The help of mqadmin.");
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
