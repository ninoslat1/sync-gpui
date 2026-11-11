use gpui_kit::{div, px, rgb, App, IntoElement, Rgba, RenderOnce, Styled, Window};

#[derive(IntoElement)]
pub struct Divider {
    thickness: f32,
    color: Rgba,
}

impl Divider {
    pub fn new() -> Self {
        Self {
            thickness: 1.0,
            color: rgb(0xd0d0d0),
        }
    }

    pub fn thickness(mut self, px_value: f32) -> Self {
        self.thickness = px_value;
        self
    }

    pub fn color(mut self, hex: u32) -> Self {
        self.color = rgb(hex);
        self
    }
}

impl RenderOnce for Divider {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .w_full()
            .h(px(self.thickness))
            .flex_none()
            .bg(self.color)
    }
}