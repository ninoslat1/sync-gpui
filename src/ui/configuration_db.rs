
use std::time::Duration;

use crate::types::DBConfigForm;

use crate::components::{Divider, FieldRow};

use gpui_kit::component::WindowExt;
use gpui_kit::component::notification::Notification;
use gpui_kit::{base::{StyledExt, Root}, component::{
    button::{Button, ButtonVariants},
    input::{Input, InputState},
}, *};

impl DBConfigForm {
    pub fn new(
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            db_host: cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("Harap input nama host")
                    .default_value("localhost")
            }),

            db_port: cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("Harap input port database")
                    .pattern(regex::Regex::new(r"^\d*$").unwrap())
                    .default_value("3306")
                    
            }),

            db_name: cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("Harap input nama database")
            }),

            db_username: cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("Harap input database username")
                    .default_value("root")
            }),

            db_password: cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("Harap input password database")
                    .masked(true)
            }),
        }
    }
}

impl Render for DBConfigForm {
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
            .gap_6()
            .bg(rgb(0xf4f4f5)) // light gray page background
            .text_color(rgb(0x09090b))
            // Title, outside the card
            .child(
                div()
                    .w(px(560.0))
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_2xl()
                            .font_bold()
                            .child("Konfigurasi Database Aplikasi Sync Flaplock"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x71717a)) // muted text
                            .child("Atur koneksi database untuk sinkronisasi."),
                    ),
            )
            // The card
            .child(
                div()
                    .w(px(560.0))
                    .flex()
                    .flex_col()
                    .gap_3()
                    .p_6()
                    .bg(rgb(0xffffff))
                    .border_1()
                    .border_color(rgb(0xe4e4e7))
                    .rounded_xl()
                    .shadow_sm()
                    .child(div().font_bold().child("Konfigurasi Database"))
                    .child(Divider::new().thickness(1.0).color(0xe4e4e7))
                    .child(FieldRow::new("Host", Input::new(&self.db_host)))
                    .child(FieldRow::new("Port", Input::new(&self.db_port)))
                    .child(FieldRow::new("Database Name", Input::new(&self.db_name)))
                    .child(div().pt_3().font_bold().child("Konfigurasi Kredensial Database"))
                    .child(Divider::new().thickness(1.0).color(0xe4e4e7))
                    .child(FieldRow::new("Username", Input::new(&self.db_username)))
                    .child(FieldRow::new(
                        "Password",
                        Input::new(&self.db_password).mask_toggle(),
                    ))
                    // Buttons
                    .child(
                        div()
                            .pt_4()
                            .flex()
                            .flex_row()
                            .gap_3()
                            .child(
                                div().flex_1().child(
                                    Button::new("test")
                                        .label("Test Koneksi")
                                        .w_full()
                                        .cursor_pointer()
                                        .on_click(cx.listener(|this, _, window, cx| {
                                            // read values here
                                            window.push_notification(
                                                Notification::success("Koneksi database berhasil").title("Berhasil").autohide(true),
                                                cx,
                                            );
                                        })),
                                ),
                            )
                            .child(
                                div().flex_1().child(
                                    Button::new("submit")
                                        .label("Simpan Konfigurasi")
                                        .primary()
                                        .w_full()
                                        .cursor_pointer()
                                        .on_click(cx.listener(|this, _, _, cx| {
                                            cx.notify();
                                        })),
                                ),
                            ),
                    ),
            )
    }
}