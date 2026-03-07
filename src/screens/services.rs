use gpui::*;
use gpui_component::button::Button;
use gpui_component::input::InputState;
use gpui_component::tab::{Tab, TabBar};
use gpui_component::{h_flex, v_flex};

use crate::components::auto_form::{AutoForm, Field, FormEvent, FormValues};
use crate::components::page_header::PageHeader;
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
    // Settings — initialized lazily on first render of the settings tab
    settings_form: Option<Entity<AutoForm>>,
    form_values: Option<FormValues>,
    _form_subscription: Option<Subscription>,
}

impl ServicesScreen {
    pub fn new(services: Vec<String>) -> Self {
        Self {
            services,
            toggle_on: false,
            active_tab: ActiveTab::Home,
            settings_form: None,
            form_values: None,
            _form_subscription: None,
        }
    }

    fn ensure_settings_form(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.settings_form.is_some() {
            return;
        }

        let host = cx.new(|cx| InputState::new(window, cx));
        let port = cx.new(|cx| InputState::new(window, cx));

        let form = cx.new(|cx| {
            AutoForm::new(
                vec![
                    Field::text("Host", host)
                        .description("The hostname or IP address of the Docker daemon."),
                    Field::text("Port", port)
                        .description("The port the Docker daemon is listening on."),
                    Field::switch("Dark Mode", false)
                        .description("Toggle the dark color theme."),
                    Field::switch("Notifications", true)
                        .description("Receive alerts when a service changes state."),
                ],
                cx,
            )
        });

        let sub = cx.subscribe(&form, |this, _, event: &FormEvent, cx| {
            if let FormEvent::Change(values) = event {
                this.form_values = Some(values.clone());
                println!("[settings] host={:?} port={:?} dark_mode={:?} notifications={:?}",
                    this.form_values.as_ref().and_then(|v| v.text("Host")),
                    this.form_values.as_ref().and_then(|v| v.text("Port")),
                    this.form_values.as_ref().and_then(|v| v.switch("Dark Mode")),
                    this.form_values.as_ref().and_then(|v| v.switch("Notifications")),
                );
                cx.notify();
            }
        });

        self.settings_form = Some(form);
        self._form_subscription = Some(sub);
    }
}

impl Render for ServicesScreen {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let toggle_on = self.toggle_on;
        let active_tab = self.active_tab;

        if active_tab == ActiveTab::Settings {
            self.ensure_settings_form(window, cx);
        }

        let selected_index = match active_tab {
            ActiveTab::Home => 0,
            ActiveTab::Settings => 1,
            ActiveTab::Assistance => 2,
        };

        let tab_content: AnyElement = match active_tab {
            ActiveTab::Home => self.render_home(cx).into_any_element(),
            ActiveTab::Settings => self.render_settings(cx).into_any_element(),
            ActiveTab::Assistance => self.render_assistance().into_any_element(),
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
                        Button::new("footer-toggle")
                            .label(if toggle_on { "ON" } else { "OFF" })
                            .outline()
                            .w_full()
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.toggle_on = !this.toggle_on;
                                println!(
                                    "Toggle button clicked — state is now: {}",
                                    if this.toggle_on { "ON" } else { "OFF" }
                                );
                                cx.notify();
                            })),
                    ),
            )
    }
}

impl ServicesScreen {
    fn render_home(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .flex_1()
            .gap_8()
            .py_8()
            .child(PageHeader::new(
                "Services".to_string(),
                "Manage and monitor your running services".to_string(),
            ))
            .child(
                h_flex()
                    .px_8()
                    .w_full()
                    .flex_wrap()
                    .gap_3()
                    .children(
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
                            .collect::<Vec<AnyElement>>(),
                    ),
            )
    }

    fn render_settings(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .flex_1()
            .gap_8()
            .py_8()
            .child(PageHeader::new(
                "Settings".to_string(),
                "Configure your application preferences here.".to_string(),
            ))
            .child(
                div()
                    .px_8()
                    .child(self.settings_form.clone().unwrap())
            )
    }

    fn render_assistance(&self) -> impl IntoElement {
        v_flex()
            .flex_1()
            .gap_8()
            .py_8()
            .child(PageHeader::new(
                "Assistance".to_string(),
                "Find help, documentation and assistance.".to_string(),
            ))
    }
}
