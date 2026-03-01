use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::Disableable;
use gpui_component::button::Button;
use gpui_component::v_flex;

use crate::components::file_picker::FilePicker;
use crate::screens::router::NavigationEvent;

pub struct EntryScreen {
    default_path: String,
    file_picker: Option<Entity<FilePicker>>,
}

impl EventEmitter<NavigationEvent> for EntryScreen {}

impl EntryScreen {
    pub fn new(default_path: String) -> Self {
        Self {
            default_path: default_path,
            file_picker: None,
        }
    }
}

impl Render for EntryScreen {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.file_picker.is_none() {
            self.file_picker = Some(cx.new(|_cx| FilePicker::new()));
        }

        let file_picker = self.file_picker.as_ref().unwrap();

        let has_file = file_picker.read(cx).selected_path().is_some();

        v_flex()
            .justify_center()
            .items_center()
            .size_full()
            .p_4()
            .gap_6()
            .child(
                div()
                    .text_2xl()
                    .font_weight(FontWeight::BOLD)
                    .child("Entry Screen"),
            )
            .child(
                v_flex().gap_4().w_96().child(file_picker.clone()).child(
                    Button::new("submit-button")
                        .label("Submit")
                        .when(!has_file, |b| b.disabled(true))
                        .when(!has_file, |b| b.cursor(CursorStyle::OperationNotAllowed))
                        .when(has_file, |b| b.cursor(CursorStyle::PointingHand))
                        .on_click(cx.listener(|this, _, _window, cx| {
                            cx.emit(NavigationEvent {
                                from: "entry".to_string(),
                            });
                        })),
                ),
            )
    }
}
