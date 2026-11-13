mod types;
mod ui;
mod components;
mod db;
mod libs;
mod constants;
mod repository;

use gpui_kit::{base::Root, *};

use crate::{ui::app_shell::AppShell};

fn main() {
    gpui_kit::application()
        .with_assets(gpui_kit::assets::Assets)
        .run(|cx| {
            gpui_kit::init(cx);

            let bounds = Bounds::centered(
                None,
                size(px(600.0), px(600.0)),
                cx,
            );

            gpui_kit::open_window(
                WindowOptions {
                    window_bounds: Some(
                        WindowBounds::Windowed(bounds)
                    ),
                    titlebar: Some(TitlebarOptions {
                        title: Some("Sync Flaplock".into()),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                cx,
                |window, cx| {
                    let shell = cx.new(|cx| AppShell::new(window, cx));
                    cx.new(|cx| Root::new(shell, window, cx))
                },
            )
            .expect("Failed to open window");
        });
}