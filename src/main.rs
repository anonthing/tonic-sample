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
use std::thread;
use tokio::sync::mpsc::{Receiver, Sender};
use tonic_sample::sample_service_server::{SampleService, SampleServiceServer};
use tonic_sample::QueryRequest;
use tonic_sample::SampleResponse;
use std::sync::Arc;

#[derive(Default)]
pub struct SampleTonicService;

async fn generate_response(sender: tokio::sync::mpsc::Sender<Result<SampleResponse, tonic::Status>>) {
    let mut rng = rand::thread_rng();
    let mut hashes = Vec::new();
    for j in 0..400_000 {
        // 400k i64's. 3.2MB per stream. 96MB total
        hashes.push(rng.gen::<i64>());
    }

    for i in 0..30 {
        // 30 streams
        let hashes_clone = hashes.clone();
        let sender_clone = sender.clone();
        tokio::spawn(async move {
            let response = SampleResponse { hash: hashes_clone };
            let curr_time = SystemTime::now();
            sender_clone.send(Ok(response)).await.unwrap();
            println!("Time took to send {:?}", SystemTime::now().duration_since(curr_time).unwrap());
        });
    }
}

#[tonic::async_trait]
impl SampleService for SampleTonicService {
    type GetResponseStream =
        Pin<Box<dyn Stream<Item = Result<SampleResponse, Status>> + Send + Sync>>;

    async fn get_response(
        &self,
        request: Request<QueryRequest>,
    ) -> Result<Response<Self::GetResponseStream>, tonic::Status> {
        let r = request.into_inner();
        let (sender, receiver) = tokio::sync::mpsc::channel(30);
        let curr_time = SystemTime::now();

        generate_response(sender).await;
        println!(
            "Time took to generate response {:?}",
            SystemTime::now().duration_since(curr_time).unwrap()
        );
        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(receiver),
        )))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    start_grpc_server().await.unwrap();

    Ok(())
}

async fn start_grpc_server() -> Result<(), Box<dyn std::error::Error>> {
    let mut hostname = local_ipaddress::get().unwrap();
    hostname.push_str(":9999");
    let addr = hostname.parse()?;

    let sample_service = SampleTonicService::default();

    let svc = SampleServiceServer::new(sample_service).send_gzip();
    println!("Starting server on {:?}", hostname);

    Server::builder().add_service(svc).serve(addr).await?;

    println!("Started server on {:?}", hostname);
    Ok(())
}
