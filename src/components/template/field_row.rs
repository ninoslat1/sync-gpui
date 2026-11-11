use gpui_kit::*;

#[derive(IntoElement)]
pub struct FieldRow {
    label: SharedString,
    input: AnyElement,
    input_width: f32,
}

impl FieldRow {
    pub fn new(label: impl Into<SharedString>, input: impl IntoElement) -> Self {
        Self {
            label: label.into(),
            input: input.into_any_element(),
            input_width: 250.0,
        }
    }

    /// Width of the input area in pixels
    pub fn input_width(mut self, width: f32) -> Self {
        self.input_width = width;
        self
    }
}

impl RenderOnce for FieldRow {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .child(self.label)
            .child(
                div()
                    .w(px(self.input_width))
                    .flex_none()
                    .child(self.input),
            )
    }
}