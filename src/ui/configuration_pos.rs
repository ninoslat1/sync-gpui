
use crate::types::Location;
use crate::types::config::StoredPosConfig;
use crate::{constants::ui_settings::SHELL_WINDOW_SIZE, types::PosConfigForm};

use crate::components::Divider;

use gpui_kit::component::WindowExt;
use gpui_kit::{base::{StyledExt}, *,component::{
    button::{Button, ButtonVariants},
    input::{Input, InputState},
}};
use regex::Regex;

const MOCK_LOCATIONS: [Location; 3] = [
    Location { id: 1, name: "Jakarta Pusat" },
    Location { id: 2, name: "Bandung" },
    Location { id: 3, name: "Surabaya" },
];

impl PosConfigForm {
    pub fn new(
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
           pos_api_url: cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("API URL Pos")
            }),

            pos_token: cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("Token Akses API URL Pos")
            }),

            pos_location_code: cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("Pilih Kode Lokasi")
            }),

            pos_area_code: cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("Pilih Kode Area")
            }),

            pos_grace_period: cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("Input menit waktu grace period")
                    .pattern(Regex::new(r"^\d*$").unwrap())
            }),

            pos_ping_interval: cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("Input menit waktu interval ping")
                    .pattern(Regex::new(r"^\d*$").unwrap())
            }),

            location_id: None,
            area_id: None,
            
        }
    }

    pub fn collect_config(&self, cx: &App) -> Result<StoredPosConfig, String> {
        let api_url = self.pos_api_url.read(cx).value().trim().to_string();
        let token = self.pos_token.read(cx).value().trim().to_string();

        if api_url.is_empty() || token.is_empty() {
            return Err("API URL dan token wajib diisi".into());
        }
        let Some(location_code) = self.location_id else {
            return Err("Pilih lokasi terlebih dahulu".into());
        };
        let Some(area_code) = self.area_id else {
            return Err("Pilih area terlebih dahulu".into());
        };

        let parse = |s: &Entity<InputState>, label: &str| -> Result<i32, String> {
            s.read(cx)
                .value()
                .trim()
                .parse::<i32>()
                .ok()
                .filter(|v| *v > 0)
                .ok_or_else(|| format!("{label} harus berupa angka lebih dari 0"))
        };

        Ok(StoredPosConfig {
            api_url,
            token,
            location_code: Some(location_code),
            area_code: Some(area_code),
            grace_period: Some(parse(&self.pos_grace_period, "Grace period")?),
            ping_interval: Some(parse(&self.pos_ping_interval, "Interval ping")?),
        })
    }

    pub fn apply_config(
        &mut self,
        cfg: &StoredPosConfig,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.pos_api_url.update(cx, |s, cx| s.set_value(cfg.api_url.clone(), window, cx));
        self.pos_token.update(cx, |s, cx| s.set_value(cfg.token.clone(), window, cx));

        self.location_id = cfg.location_code;
        self.area_id = cfg.area_code;

        // Only the id is stored, so look the name up to show it
        if let Some(id) = cfg.location_code {
            if let Some(loc) = MOCK_LOCATIONS.iter().find(|l| l.id == id) {
                self.pos_location_code.update(cx, |s, cx| s.set_value(loc.name, window, cx));
            }
        }

        if let Some(v) = cfg.grace_period {
            self.pos_grace_period.update(cx, |s, cx| s.set_value(v.to_string(), window, cx));
        }
        if let Some(v) = cfg.ping_interval {
            self.pos_ping_interval.update(cx, |s, cx| s.set_value(v.to_string(), window, cx));
        }
        cx.notify();
    }
}

impl Render for PosConfigForm {
    fn render(
        &mut self,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
           .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .bg(rgb(0x1e1e2e))
            .text_color(rgb(0xffffff))
            .child(
                div()
                    .w(px(SHELL_WINDOW_SIZE))
                    .h(px(SHELL_WINDOW_SIZE))
                    .flex()
                    .flex_col()
                    .gap_3()
                    .p_4()
                    .bg(rgb(0x313244))
                    .rounded_lg()
                    .child(
                        div()
                            .text_center()
                            .text_2xl()
                            .font_bold()
                            .child("Konfigurasi Aplikasi Sync Flaplock")
                    )
                    .child(Divider::new().thickness(1.0).color(0x000000))
                    .child("API URL")
                    .child(Input::new(&self.pos_api_url))
                    .child("API URL Token")
                    .child(Input::new(&self.pos_token))
                    .child("Kode Lokasi")
                    .child({
                        let view = cx.entity(); // handle to this form, so the dialog can update it

                        div()
                            .flex()
                            .flex_row()
                            .gap_2()
                            .items_center()
                            .child(
                                div()
                                    .flex_1()
                                    .child(Input::new(&self.pos_location_code).disabled(true)),
                            )
                            .child(
                                Button::new("pick-location")
                                    .label("Pilih")
                                    .cursor_pointer()
                                    .on_click(move |_, window, cx| {
                                        let view = view.clone();
                                        window.open_dialog(cx, move |dialog, _window, _cx| {
                                            let view = view.clone();
                                            dialog.title("Pilih Lokasi").child(
                                                div().flex().flex_col().gap_2().children(
                                                    MOCK_LOCATIONS.iter().enumerate().map(|(i, loc)| {
                                                        let view = view.clone();
                                                        Button::new(("loc", i))
                                                            .label(format!("{} - {}", loc.id, loc.name))
                                                            .w_full()
                                                            .cursor_pointer()
                                                            .on_click(move |_, window, cx| {
                                                                view.update(cx, |this, cx| {
                                                                    this.location_id = Some(loc.id);
                                                                    this.pos_location_code.update(cx, |s, cx| {
                                                                        s.set_value(loc.name, window, cx);
                                                                    });
                                                                    cx.notify();
                                                                });
                                                                window.close_dialog(cx);
                                                            })
                                                    }),
                                                ),
                                            )
                                        });
                                    }),
                            )}
                    )
                    .child("Kode Area")
                    .child(Input::new(&self.pos_area_code))
                    .child("Grace Period")
                    .child(Input::new(&self.pos_grace_period))
                    .child("Ping Interval")
                    .child(Input::new(&self.pos_ping_interval))
                    .child(
                        div()
                            .py_8()
                            .child(
                                div()
                                .child(
                                    Button::new("submit")
                                        .label("Simpan Konfigurasi")
                                        .primary()
                                        .on_click(cx.listener(
                                            |this, _, _, cx| {
                                                // let host = this.db_host
                                                //     .read(cx)
                                                //     .value()
                                                //     .to_string();
            
                                                // let port = this.db_port.read(cx).value().to_string();
                                                // let name = this.db_name.read(cx).value().to_string();
                                                // let username = this.db_username.read(cx).value().to_string();
                                                // let password = this.db_password.read(cx).value().to_string();
            
                                                // cx.notify();
                                            },
                                        ))
                                )
                            )
                    )
            )
    }
}