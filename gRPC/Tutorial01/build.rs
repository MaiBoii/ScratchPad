// https://www.youtube.com/watch?v=JkSa-qA2jnY

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("/home/chan/OnDev/gRPC_tutorial/src/proto/payments.proto")?;
    Ok(())
}
