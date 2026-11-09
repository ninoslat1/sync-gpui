use gpui_kit::{base::StyledExt, *};

use crate::types::PosConfigForm;

use gpui_kit::component::{
    button::{Button, ButtonVariants},
    input::{Input, InputState},
};

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

            pos_type: cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("Tipe Pos")
            }),

            db_username: cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("Database Username")
            }),

            db_password: cx.new(|cx| {
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
                    .w(px(600.0))
                    .h(px(600.0))
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
                    .child("Database Host")
                    .child(Input::new(&self.db_host))
                    .child("Database Port")
                    .child(Input::new(&self.db_port))
                    .child("Database Name")
                    .child(Input::new(&self.db_name))
                    .child("Database Username")
                    .child(Input::new(&self.db_username))
                    .child("Database Password")
                    .child(Input::new(&self.db_password))
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
                                                let host = this.db_host
                                                    .read(cx)
                                                    .value()
                                                    .to_string();
            
                                                let port = this.db_port.read(cx).value().to_string();
                                                let name = this.db_name.read(cx).value().to_string();
                                                let username = this.db_username.read(cx).value().to_string();
                                                let password = this.db_password.read(cx).value().to_string();
            
                                                cx.notify();
                                            },
                                        ))
                                )
                            )
                    )
            )
    }
}