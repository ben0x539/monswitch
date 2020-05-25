use crate::display::monitor::{Monitor, MonitorManager};
use systray;

#[derive(Debug, Clone)]
pub struct AppError(&'static str);

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "App Error: {}", self.0)
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

pub struct App {
    manager: MonitorManager,
    tray: systray::Application,
}

impl App {
    pub fn create_with_monitor_manager(manager: MonitorManager) -> Result<App, AppError> {
        let mut tray = systray::Application::new().unwrap();

        for monitor in &manager.monitors {
            let caps = monitor.capabilities.as_ref().unwrap();
            let display_type = &caps.display_model;

            tray.add_menu_item(&display_type, |_| {}).unwrap();

            tray.add_menu_separator().unwrap();

            for input in &monitor.inputs {
                tray.add_menu_item(&input.to_string(), move |_| {
                    monitor.set_input(input.clone());
                })
                .unwrap();
            }

            tray.add_menu_separator().unwrap();
        }

        tray.add_menu_item(&"Quit".to_string(), |window| {
            window.quit();
        })
        .unwrap();

        let app = App { manager, tray };

        Ok(app)
    }

    pub fn run(&mut self) -> Result<(), AppError> {
        let tray = &mut self.tray;

        tray.wait_for_message();

        Ok(())
    }
}
