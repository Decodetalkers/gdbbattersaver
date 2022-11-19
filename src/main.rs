#![feature(fs_try_exists)]
mod config;
use config::AvailableSetting;
use slint::{SharedString, VecModel};
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

    //let ui_handle = ui.as_weak();
    //ui.on_request_increase_value(move || {
    //    let ui = ui_handle.unwrap();
    //    ui.set_counter(ui.get_counter() + 1);
    //});

    ui.run();
}
