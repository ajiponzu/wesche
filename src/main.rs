#![windows_subsystem = "windows"]
use std::sync::Arc;

use async_std::sync::Mutex;
use async_std::task;

mod apps;
use apps::controller::{self, AsyncLoopInterface};

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

    let opening_viewer_handle = {
        let application_controller = application_controller.clone();
        task::spawn(async move { application_controller.wait_for_open_viewer().await })
    };

    let mut system_tray = systray::Application::new().expect("Failed to create system tray");
    {
        {
            let application_controller = application_controller.clone();
            system_tray
                .set_icon_from_file(
                    application_controller
                        .lock()
                        .await
                        .get_icon_file_path()
                        .as_str(),
                )
                .expect("Failed to set icon");
        }
        {
            let application_controller = application_controller.clone();
            system_tray
                .add_menu_item("Open Task Viewer", move |_| {
                    async_std::task::block_on(async {
                        let mut application_controller = application_controller.lock().await;
                        application_controller.open_viewer();
                        Ok::<_, systray::Error>(())
                    })
                })
                .expect("Failed to add menu item");
        }
        {
            let application_controller = application_controller.clone();
            system_tray
                .add_menu_item("Quit", move |_| {
                    async_std::task::block_on(async {
                        let mut application_controller = application_controller.lock().await;
                        application_controller.shutdown();
                        Ok::<_, systray::Error>(())
                    })
                })
                .expect("Failed to add menu item");
        }
    }

    async_std::task::spawn(async move {
        system_tray
            .wait_for_message()
            .expect("Failed to wait for sytem_tray message");

        system_tray
            .shutdown()
            .expect("Failed to shutdown system tray");
    });

    application_loop_handle.await;
    opening_viewer_handle.await;

    Ok(())
}
