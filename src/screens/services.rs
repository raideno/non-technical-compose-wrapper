use gpui::*;
use gpui_component::{
    ActiveTheme,
    button::{Toggle, ToggleVariants},
    h_flex, v_flex,
};

use crate::components::service_card::{ServiceCard, ServiceStatus};

pub struct ServicesScreen {
    services: Vec<String>,
    toggle_on: bool,
}

impl ServicesScreen {
    pub fn new(services: Vec<String>) -> Self {
        Self {
            services,
            toggle_on: false,
        }
    }
}

impl Render for ServicesScreen {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let toggle_on = self.toggle_on;

        v_flex()
            .size_full()
            .child(
                div()
                    .bg(black())
                    .w_full()
                    .h_12()
                    .border_b_1()
                    .border_color(white()),
            )
            .child(
                v_flex()
                    .flex_1()
                    .gap_8()
                    .py_8()
                    .child(
                        v_flex()
                            .px_8()
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
                        h_flex().px_8().w_full().flex_wrap().gap_3().children(
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
                    ),
            )
            .child(
                div()
                    .bg(black())
                    .w_full()
                    .h_12()
                    .border_t_1()
                    .border_color(white())
                    .flex()
                    .items_center()
                    .px_4()
                    .child(
                        Toggle::new("footer-toggle")
                            .label(if toggle_on { "ON" } else { "OFF" })
                            .checked(toggle_on)
                            .outline()
                            .w_full()
                            .on_click(cx.listener(move |this, new_state, _window, _cx| {
                                this.toggle_on = *new_state;
                                println!(
                                    "Toggle button clicked — state is now: {}",
                                    if this.toggle_on { "ON" } else { "OFF" }
                                );
                            })),
                    ),
            )
    }
}
