mod apps;

#[async_std::main]
async fn main() -> std::io::Result<()> {
    let mut engine = apps::engine::Engine::new();

    engine.run().await?;

    println!("{:?}", engine);

    Ok(())
}
