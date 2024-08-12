use std::sync::Arc;

use apps::controller::{self, AsyncLoopInterface};
use async_std::sync::Mutex;
use async_std::task;
use std::time::Duration;

mod apps;

#[async_std::main]
async fn main() -> std::io::Result<()> {
    let application_controller = Arc::new(Mutex::new(controller::Application::new()));

    {
        let mut application_controller = application_controller.lock().await;
        application_controller.load_schedule().await?;
    }

    let application_loop_handle = {
        let application_controller = application_controller.clone();
        task::spawn(async move { application_controller.async_loop().await })
    };

    task::sleep(Duration::from_secs(1)).await;
    application_controller.lock().await.shutdown();

    application_loop_handle.await;

    Ok(())
}
