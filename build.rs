fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure().compile_protos(
        &[
            "proto/settings_api.proto",
            "proto/pref_api.proto",
            "proto/helper.proto",
            "proto/call_api.proto",
        ],
        &["proto/"],
    )?;

    Ok(())
}
