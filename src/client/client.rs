/*
 *
 *  * This file is part of OpenTSDB.
 *  * Copyright (C) 2021  Yahoo.
 *  *
 *  * Licensed under the Apache License, Version 2.0 (the "License");
 *  * you may not use this file except in compliance with the License.
 *  * You may obtain a copy of the License at
 *  *
 *  *   http://www.apache.org/licenses/LICENSE-2.0
 *  *
 *  * Unless required by applicable law or agreed to in writing, software
 *  * distributed under the License is distributed on an "AS IS" BASIS,
 *  * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  * See the License for the specific language governing permissions and
 *  * limitations under the License.
 *
 */
use log::{error, info};
use std::pin::Pin;
use std::time::SystemTime;
use tokio_stream::Stream;
use tonic::transport::Server;
use tonic::{Request, Response, Status};
pub mod tonic_sample {
    tonic::include_proto!("tonicsample");
}
use rand::Rng;
use std::{thread, env};
use tokio::sync::mpsc::{Receiver, Sender};
use tonic_sample::sample_service_server::{SampleService, SampleServiceServer};
use tonic_sample::sample_service_client::SampleServiceClient;
use tonic_sample::QueryRequest;
use tonic_sample::SampleResponse;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let mut host = Box::new(String::from("http://"));
    host.push_str(args.get(1).unwrap());

    let h: &'static str = Box::leak(host);
    let channel = tonic::transport::Channel::from_static(h)
        .connect()
        .await
        .unwrap();
    let mut client = SampleServiceClient::new(channel).accept_gzip();

  //  let s = args.get(2).unwrap();
    let query_request = QueryRequest {
        query: String::from("hello"),
    };
    println!("Running query {:?}", query_request);
    let mut stream = client
        .get_response(Request::new(query_request))
        .await
        .unwrap();
    println!("Stream metadata {:?}", stream.metadata());
    let mut s = stream.into_inner();
    println!("{:?}", s);


    let mut count = 0;
    //
    let mut start_time = SystemTime::now();
    let mut curr_time = SystemTime::now();
    while let Some(resp) = s.message().await.unwrap() {
        println!("Got stream in {:?}", SystemTime::now().duration_since(curr_time).unwrap());
        curr_time = SystemTime::now();
        count += 1;
    }
    println!("Total time  {:?}", SystemTime::now().duration_since(start_time).unwrap());

}
