#![feature(fs_try_exists)]
mod batterycommand;
mod config;
use batterycommand::set_battery;
use config::AvailableSetting;
use notify_rust::Notification;
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

fn main() {
    let all_settings = config::get_all_settings();
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

    let ui_handle = ui.as_weak();
    ui.on_ChangeBattery(move || {
        let ui = ui_handle.unwrap();
        let global = AvaSettings::get(&ui);
        let globals = global.get_settings();
        for glob in globals.iter() {
            if let Err(e) = set_battery(&glob.name, &glob.tochange) {
                let _ = Notification::new()
                    .summary("Performance Error")
                    .body(&format!("Error: {e}"))
                    .icon("gdgbattersaver")
                    .timeout(10000)
                    .show();
            } else {
                let _ = Notification::new()
                    .summary("Performance Changed")
                    .body(&format!("{} change to {}", glob.name, glob.tochange))
                    .icon("gdgbattersaver")
                    .timeout(10000)
                    .show();
            };
        }
        let all_settings = config::get_all_settings();
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

    ui.on_Refresh(move || {
        let ui = ui_handle.unwrap();
        let global = AvaSettings::get(&ui);
        let all_settings = config::get_all_settings();
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

    ui.on_Exit(|| {
        let _ = slint::quit_event_loop();
    });
    //ui.on_request_increase_value(move || {
    //    let ui = ui_handle.unwrap();
    //    ui.set_counter(ui.get_counter() + 1);
    //});

    ui.run();
}
