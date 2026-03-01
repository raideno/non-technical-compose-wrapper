use gpui::*;
use gpui_component::{
    ActiveTheme,
    h_flex, v_flex,
    tag::{Tag, TagVariant},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceStatus {
    Running,
    Stopped,
}

#[derive(IntoElement)]
pub struct ServiceCard {
    name: SharedString,
    description: SharedString,
    status: ServiceStatus,
}

impl ServiceCard {
    pub fn new(
        name: impl Into<SharedString>,
        description: impl Into<SharedString>,
        status: ServiceStatus,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            status,
        }
    }
}

impl RenderOnce for ServiceCard {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let status_tag = match self.status {
            ServiceStatus::Running => Tag::new()
                .with_variant(TagVariant::Success)
                .outline()
                .child("Running"),
            ServiceStatus::Stopped => Tag::new()
                .with_variant(TagVariant::Secondary)
                .outline()
                .child("Stopped"),
        };

        div()
            .w_full()
            .px_4()
            .py_3()
            .rounded_lg()
            .border_1()
            .border_color(cx.theme().border)
            .bg(cx.theme().background)
            .child(
                h_flex()
                    .w_full()
                    .items_center()
                    .justify_between()
                    .child(
                        v_flex()
                            .gap_0p5()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(cx.theme().foreground)
                                    .child(self.name),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(cx.theme().muted_foreground)
                                    .child(self.description),
                            ),
                    )
                    .child(status_tag),
            )
    }
}
