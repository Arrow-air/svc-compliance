//! build script to generate .rs from .proto

///generates .rs files in src directory
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_dir = "../proto";
    let proto_file = &format!("{}/grpc.proto", proto_dir);

    let server_config = tonic_build::configure()
        .type_attribute("ReadyRequest", "#[derive(Eq, Copy)]")
        .type_attribute("ReadyResponse", "#[derive(Eq, Copy)]")
        .type_attribute("Coordinates", "#[derive(Copy)]")
        .type_attribute("RestrictionsRequest", "#[derive(Copy)]")
        .type_attribute("WaypointsRequest", "#[derive(Copy)]")
        .type_attribute("FlightPlanRequest", "#[derive(serde::Serialize)]");

    let client_config = server_config.clone();

    client_config
        .client_mod_attribute("grpc", "#[cfg(not(tarpaulin_include))]")
        .build_server(false)
        .out_dir("../client-grpc/src/")
        .compile(&[proto_file], &[proto_dir])?;

    // Build the Server
    server_config
        .build_client(false)
        .compile(&[proto_file], &[proto_dir])?;

    println!("cargo:rerun-if-changed={}", proto_file);

    Ok(())
}
