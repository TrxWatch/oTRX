fn main() {
    prost_build::Config::new()
        .type_attribute("proto.common.SmartContract.ABI", "#[derive(serde::Serialize)]")
        .type_attribute("proto.common.SmartContract.ABI.Entry", "#[derive(serde::Serialize)]")
        .type_attribute("proto.common.SmartContract.ABI.Param", "#[derive(serde::Serialize)]")
        .compile_protos(
            &[
                "proto/common.proto",
                "proto/discovery.proto",
                "proto/chain.proto",
                "proto/channel.proto",
                "proto/contract.proto",
                "proto/state.proto",
            ],
            &["proto/"],
        )
        .unwrap();
}
