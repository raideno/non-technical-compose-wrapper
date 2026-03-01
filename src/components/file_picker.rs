use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::{
    ActiveTheme, IconName,
    button::{Button, ButtonVariants},
    h_flex,
};
use std::path::PathBuf;

pub struct FilePicker {
    selected_file: Option<PathBuf>,
}

impl FilePicker {
    pub fn new() -> Self {
        Self {
            selected_file: None,
        }
    }

    pub fn selected_path(&self) -> Option<&PathBuf> {
        self.selected_file.as_ref()
    }

    fn open_file_picker(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let receiver = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: false,
            multiple: false,
            prompt: Some("Select a file".into()),
        });

        cx.spawn_in(window, async move |this, cx| {
            if let Ok(Ok(Some(paths))) = receiver.await {
                if let Some(path) = paths.into_iter().next() {
                    let _ = this.update(
                        cx,
                        |picker: &mut FilePicker, cx: &mut Context<FilePicker>| {
                            picker.selected_file = Some(path);
                            cx.notify();
                        },
                    );
                }
            }
        })
        .detach();
    }
}

impl Render for FilePicker {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let has_file = self.selected_file.is_some();

        let label: SharedString = if let Some(path) = &self.selected_file {
            path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown file")
                .to_string()
                .into()
        } else {
            "Choose a file…".into()
        };

        h_flex().w_full().gap_2().child(
            Button::new("file-picker-btn")
                .icon(if has_file {
                    IconName::File
                } else {
                    IconName::FolderOpen
                })
                .outline()
                .cursor(CursorStyle::PointingHand)
                .w_full()
                .label(label)
                .on_click(cx.listener(|this, _, window, cx| {
                    this.open_file_picker(window, cx);
                })),
        )
    }
}
