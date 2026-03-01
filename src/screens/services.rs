use gpui::*;
use gpui_component::{ActiveTheme, h_flex, v_flex};

use crate::components::service_card::{ServiceCard, ServiceStatus};

pub struct ServicesScreen {
    services: Vec<String>,
}

impl ServicesScreen {
    pub fn new(services: Vec<String>) -> Self {
        Self { services }
    }
}

impl Render for ServicesScreen {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .p_8()
            .gap_8()
            .child(
                v_flex()
                    .gap_1()
                    .child(
                        div()
                            .text_2xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(cx.theme().foreground)
                            .child("Services"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(cx.theme().muted_foreground)
                            .child("Manage and monitor your running containers."),
                    ),
            )
            .child(
                h_flex().w_full().flex_wrap().gap_3().children(
                    self.services
                        .iter()
                        .enumerate()
                        .map(|(i, name)| {
                            let status = if i % 2 == 0 {
                                ServiceStatus::Running
                            } else {
                                ServiceStatus::Stopped
                            };

                            ServiceCard::new(
                                name.clone(),
                                format!("Manages the {} container", name),
                                status,
                            )
                            .into_any_element()
                        })
                        .collect::<Vec<gpui::AnyElement>>(),
                ),
            )
    }
}
