use std::sync::Arc;

use apps::controller::{self, AsyncLoopInterface};
use async_std::sync::Mutex;
use async_std::task;
use std::time::Duration;

mod apps;

#[async_std::main]
async fn main() -> std::io::Result<()> {
    let engine = Arc::new(Mutex::new(controller::Application::new()));

    {
        let mut engine = engine.lock().await;
        engine.load_schedule().await?;
    }

    let engine_handle = {
        let engine = engine.clone();
        task::spawn(async move { engine.run().await })
    };

    task::sleep(Duration::from_secs(1)).await;
    engine.lock().await.shutdown();

    engine_handle.await; // runタスクの終了を待つ

    Ok(())
}
