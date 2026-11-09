mod types;
mod ui;

use gpui_kit::*;

use crate::types::DBConfigForm;

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
                    cx.new(|cx| {
                        DBConfigForm::new(window, cx)
                    })
                },
            )
            .expect("Failed to open window");
        });
}