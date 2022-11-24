mod batterycommand;
mod config;
mod settings;
use batterycommand::set_battery;
use notify_rust::Notification;
use settings::AvailableSetting;
use slint::{Model, SharedString, VecModel};
use std::rc::Rc;
slint::include_modules!();

impl From<AvailableSetting> for AvaSetting {
    fn from(value: AvailableSetting) -> Self {
        AvaSetting {
            name: value.name.into(),
            current: value.currentselected.clone().into(),
            tochange: value.currentselected.into(),
            selects: Rc::new(VecModel::from(
                value
                    .selects
                    .into_iter()
                    .map(|unit| unit.into())
                    .collect::<Vec<SharedString>>(),
            ))
            .into(),
            doc: value.doc.into(),
        }
    }
}

fn init_slots(ui: &AppWindow) {
    let battery_callbacks = BatteryCallbacks::get(ui);
    let ui_handle = ui.as_weak();
    battery_callbacks.on_ChangeBattery(move || {
        let ui = ui_handle.unwrap();
        let global = AvaSettings::get(&ui);
        let globals = global.get_settings();
        for glob in globals.iter() {
            if let Err(e) = set_battery(&glob.name, &glob.tochange) {
                log::error!("Error: {e}");
                let _ = Notification::new()
                    .summary("Performance Error")
                    .body(&format!("Error: {e}"))
                    .icon("Green_Dam_Girl")
                    .timeout(10000)
                    .show();
            } else {
                log::info!("{} change to {}", glob.name, glob.tochange);
                let _ = Notification::new()
                    .summary("Performance Changed")
                    .body(&format!("{} change to {}", glob.name, glob.tochange))
                    .icon("Green_Dam_Girl")
                    .timeout(10000)
                    .show();
            };
        }
        let all_settings = settings::get_all_settings();
        global.set_settings(
            Rc::new(VecModel::from(
                all_settings
                    .into_iter()
                    .map(|unit| unit.into())
                    .collect::<Vec<AvaSetting>>(),
            ))
            .into(),
        );
    });
    let ui_handle = ui.as_weak();

    battery_callbacks.on_Refresh(move || {
        let ui = ui_handle.unwrap();
        let global = AvaSettings::get(&ui);
        let all_settings = settings::get_all_settings();
        global.set_settings(
            Rc::new(VecModel::from(
                all_settings
                    .into_iter()
                    .map(|unit| unit.into())
                    .collect::<Vec<AvaSetting>>(),
            ))
            .into(),
        );
    });

    battery_callbacks.on_Exit(|| {
        let _ = slint::quit_event_loop();
    });
}

fn run_main() {
    let all_settings = settings::get_all_settings();
    let ui = AppWindow::new();
    let globals = AvaSettings::get(&ui);
    globals.set_settings(
        Rc::new(VecModel::from(
            all_settings
                .into_iter()
                .map(|unit| unit.into())
                .collect::<Vec<AvaSetting>>(),
        ))
        .into(),
    );
    globals.set_about(include_str!("../misc/about/aboutapp.md").into());

    init_slots(&ui);

    ui.run();
}

fn main() {
    #[cfg(feature = "tray")]
    {
        use tray_item::TrayItem;
        gtk::init().unwrap();

        let mut tray = TrayItem::new("Green_Dam_Girl", "Green_Dam_Girl").unwrap();

        tray.add_label("Select").unwrap();

        tray.add_menu_item("Show", || {
            run_main();
        })
        .unwrap();

        tray.add_menu_item("Hide", || {
            let _ = slint::quit_event_loop();
        })
        .unwrap();

        gtk::main();
    }

    #[cfg(not(feature = "tray"))]
    {
        run_main();
    }
}
