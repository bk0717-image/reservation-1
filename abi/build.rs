fn main() {
    // tonic_build::configure()
    //     .out_dir("src/pb")
    //     .compile(&["protos/reservation.proto"], &["protos"])
    //     .unwrap();

    protobuf_codegen_pure::Codegen::new()
        .out_dir("src/pb")
        .inputs(&["protos/reservation.proto"])
        .include("protos")
        .run()
        .unwrap();
}
