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
use rocketmq_common::common::message::message_single::Message;

use crate::producer::local_transaction_state::LocalTransactionState;

pub struct EndTransactionContext<'a> {
    pub producer_group: String,
    pub broker_addr: String,
    pub message: &'a Message,
    pub msg_id: String,
    pub transaction_id: String,
    pub transaction_state: LocalTransactionState,
    pub from_transaction_check: bool,
}
