
use crate::{constants::ui_settings::SHELL_WINDOW_SIZE, types::PosConfigForm};

use crate::components::Divider;

use gpui_kit::{base::{StyledExt}, *,component::{
    button::{Button, ButtonVariants},
    input::{Input, InputState},
}};

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
                    .placeholder("Database Username")
            }),

            pos_area_code: cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("Database Password")
            }),

            pos_grace_period: cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("Database Password")
            }),

            pos_ping_interval: cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("Database Password")
            }),
        }
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
                    .child(Input::new(&self.pos_location_code))
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