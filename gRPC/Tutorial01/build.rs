// https://www.youtube.com/watch?v=JkSa-qA2jnY

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("./src/proto/payments.proto")?;
    Ok(())
}
