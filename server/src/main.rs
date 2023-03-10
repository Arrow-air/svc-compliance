//! gRPC server implementation

mod region_interface;
mod regions;

///module svc_storage generated from svc-storage.proto
pub mod svc_compliance {
    #![allow(unused_qualifications, missing_docs)]
    include!("grpc.rs");
}

use crate::region_interface::RegionInterface;
use crate::svc_compliance::{
    FlightPlanRequest, FlightPlanResponse, FlightReleaseRequest, FlightReleaseResponse,
};
use dotenv::dotenv;
use svc_compliance::compliance_rpc_server::{ComplianceRpc, ComplianceRpcServer};
use svc_compliance::{QueryIsReady, ReadyResponse};
use tonic::{transport::Server, Request, Response, Status};

///Implementation of gRPC endpoints
#[derive(Debug, Default, Copy, Clone)]
pub struct ComplianceImpl {}

#[tonic::async_trait]
impl ComplianceRpc for ComplianceImpl {
    /// Returns ready:true when service is available
    async fn is_ready(
        &self,
        _request: Request<QueryIsReady>,
    ) -> Result<Response<ReadyResponse>, Status> {
        let response = ReadyResponse { ready: true };
        Ok(Response::new(response))
    }

    async fn submit_flight_plan(
        &self,
        request: Request<FlightPlanRequest>,
    ) -> Result<Response<FlightPlanResponse>, Status> {
        get_region_impl().submit_flight_plan(request)
    }

    async fn request_flight_release(
        &self,
        request: Request<FlightReleaseRequest>,
    ) -> Result<Response<FlightReleaseResponse>, Status> {
        get_region_impl().request_flight_release(request)
    }
}

///Returns region implementation based on REGION_CODE environment variable
fn get_region_impl() -> Box<dyn RegionInterface> {
    let region = std::env::var("REGION_CODE").unwrap_or_else(|_| "us".to_string());
    match region.as_str() {
        "us" => Box::new(regions::us::USImpl {}),
        "ne" => Box::new(regions::ne::NEImpl {}),
        _ => {
            panic!("Unknown region: {}", region);
        }
    }
}

///Main entry point: starts gRPC Server on specified address and port
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //initialize dotenv library which reads .env file
    dotenv().ok();
    //initialize logger
    let log_cfg: &str = "log4rs.yaml";
    if let Err(e) = log4rs::init_file(log_cfg, Default::default()) {
        println!("(logger) could not parse {}. {}", log_cfg, e);
        panic!();
    }
    // check region implementation and panic if region code is unknown
    get_region_impl();
    // GRPC Server
    let grpc_port = std::env::var("DOCKER_PORT_GRPC")
        .unwrap_or_else(|_| "50051".to_string())
        .parse::<u16>()
        .unwrap_or(50051);

    let full_grpc_addr = format!("[::]:{}", grpc_port).parse()?;

    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    let imp = ComplianceImpl::default();
    health_reporter
        .set_serving::<ComplianceRpcServer<ComplianceImpl>>()
        .await;

    //start server
    println!("Starting gRPC server at: {}", full_grpc_addr);
    Server::builder()
        .add_service(health_service)
        .add_service(ComplianceRpcServer::new(imp))
        .serve(full_grpc_addr)
        .await?;
    println!("gRPC server running at: {}", full_grpc_addr);

    Ok(())
}
