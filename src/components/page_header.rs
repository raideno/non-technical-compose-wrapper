use gpui::*;
use gpui_component::{
    ActiveTheme,
    v_flex,
};

#[derive(IntoElement)]
pub struct PageHeader {
    title: String,
    description: String,
}

impl PageHeader {
    pub fn new(title: String, description: String) -> Self {
        Self { title, description }
    }
}

impl RenderOnce for PageHeader {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        v_flex()
            .px_8()
            .child(
                div()
                    .text_2xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(cx.theme().foreground)
                    .child(self.title),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(cx.theme().muted_foreground)
                    .child(self.description),
            )
    }
}
