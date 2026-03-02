use gpui::*;
use gpui_component::tab::{Tab, TabBar};
use gpui_component::{
    ActiveTheme,
    button::{Toggle, ToggleVariants},
    h_flex, v_flex,
};

use crate::components::service_card::{ServiceCard, ServiceStatus};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveTab {
    Home,
    Settings,
    Assistance,
}

pub struct ServicesScreen {
    services: Vec<String>,
    toggle_on: bool,
    active_tab: ActiveTab,
}

impl ServicesScreen {
    pub fn new(services: Vec<String>) -> Self {
        Self {
            services,
            toggle_on: false,
            active_tab: ActiveTab::Home,
        }
    }
}

impl Render for ServicesScreen {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let toggle_on = self.toggle_on;
        let active_tab = self.active_tab;

        let selected_index = match active_tab {
            ActiveTab::Home => 0,
            ActiveTab::Settings => 1,
            ActiveTab::Assistance => 2,
        };

        let tab_content: AnyElement = match active_tab {
            ActiveTab::Home => self.render_home(cx).into_any_element(),
            ActiveTab::Settings => self.render_settings(cx).into_any_element(),
            ActiveTab::Assistance => self.render_assistance(cx).into_any_element(),
        };

        v_flex()
            .size_full()
            .child(
                TabBar::new("main-tabs")
                    .selected_index(selected_index)
                    .child(
                        Tab::new()
                            .label("Home")
                            .cursor(CursorStyle::PointingHand)
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.active_tab = ActiveTab::Home;
                                cx.notify();
                            })),
                    )
                    .child(
                        Tab::new()
                            .label("Settings")
                            .cursor(CursorStyle::PointingHand)
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.active_tab = ActiveTab::Settings;
                                cx.notify();
                            })),
                    )
                    .child(
                        Tab::new()
                            .label("Assistance")
                            .cursor(CursorStyle::PointingHand)
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.active_tab = ActiveTab::Assistance;
                                cx.notify();
                            })),
                    ),
            )
            .child(div().flex_1().child(tab_content))
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

impl ServicesScreen {
    fn render_home(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .flex_1()
            .gap_8()
            .py_8()
            .child(
                v_flex()
                    .px_8()
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
            )
    }

    fn render_settings(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .flex_1()
            .size_full()
            .py_8()
            .px_8()
            .child(
                div()
                    .text_2xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(cx.theme().foreground)
                    .child("Settings"),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(cx.theme().muted_foreground)
                    .child("Configure your application preferences here."),
            )
    }

    fn render_assistance(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .flex_1()
            .size_full()
            .py_8()
            .px_8()
            .child(
                div()
                    .text_2xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(cx.theme().foreground)
                    .child("Assistance"),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(cx.theme().muted_foreground)
                    .child("Find help, documentation, and support resources."),
            )
    }
}
