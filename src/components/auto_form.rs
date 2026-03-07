use std::collections::HashMap;

use gpui::*;
use gpui_component::{
    form::{field, v_form},
    input::{Input, InputEvent, InputState},
    switch::Switch,
};

#[derive(Debug, Clone)]
pub enum FieldValue {
    Text(SharedString),
    Switch(bool),
}

#[derive(Debug, Clone)]
pub struct FormValues(pub HashMap<String, FieldValue>);

impl FormValues {
    pub fn text(&self, key: &str) -> Option<&SharedString> {
        if let Some(FieldValue::Text(v)) = self.0.get(key) { Some(v) } else { None }
    }

    pub fn switch(&self, key: &str) -> Option<bool> {
        if let Some(FieldValue::Switch(v)) = self.0.get(key) { Some(*v) } else { None }
    }
}

pub enum FormEvent {
    Change(FormValues),
}

pub enum FieldType {
    Text(Entity<InputState>),
    Switch(bool),
}

pub struct Field {
    pub label: String,
    pub description: Option<String>,
    pub field_type: FieldType,
}

impl Field {
    pub fn text(label: impl Into<String>, state: Entity<InputState>) -> Self {
        Self { label: label.into(), description: None, field_type: FieldType::Text(state) }
    }

    pub fn switch(label: impl Into<String>, checked: bool) -> Self {
        Self { label: label.into(), description: None, field_type: FieldType::Switch(checked) }
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
}

enum StoredField {
    Text { label: String, description: Option<String>, state: Entity<InputState> },
    Switch { label: String, description: Option<String>, checked: bool },
}

impl StoredField {
    fn label(&self) -> &str {
        match self {
            StoredField::Text { label, .. } => label,
            StoredField::Switch { label, .. } => label,
        }
    }
}

pub struct AutoForm {
    fields: Vec<StoredField>,
    _subscriptions: Vec<Subscription>,
}

impl EventEmitter<FormEvent> for AutoForm {}

impl AutoForm {
    pub fn new(fields: Vec<Field>, cx: &mut Context<Self>) -> Self {
        let mut stored: Vec<StoredField> = Vec::with_capacity(fields.len());
        let mut subs: Vec<Subscription> = Vec::new();

        for f in fields {
            match f.field_type {
                FieldType::Text(state) => {
                    let sub = cx.subscribe(&state, |this, _, event: &InputEvent, cx| {
                        if matches!(event, InputEvent::Change) {
                            this.emit_values(cx);
                        }
                    });
                    subs.push(sub);
                    stored.push(StoredField::Text {
                        label: f.label,
                        description: f.description,
                        state,
                    });
                }
                FieldType::Switch(checked) => {
                    stored.push(StoredField::Switch {
                        label: f.label,
                        description: f.description,
                        checked,
                    });
                }
            }
        }

        Self { fields: stored, _subscriptions: subs }
    }

    pub fn values(&self, cx: &App) -> FormValues {
        let map = self
            .fields
            .iter()
            .map(|f| match f {
                StoredField::Text { label, state, .. } => {
                    (label.clone(), FieldValue::Text(state.read(cx).value()))
                }
                StoredField::Switch { label, checked, .. } => {
                    (label.clone(), FieldValue::Switch(*checked))
                }
            })
            .collect();
        FormValues(map)
    }

    fn emit_values(&self, cx: &mut Context<Self>) {
        let values = self.values(cx);
        cx.emit(FormEvent::Change(values));
    }
}

impl Render for AutoForm {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let form = v_form();

        self.fields.iter().enumerate().fold(form, |form, (i, stored)| {
            let entry = match stored {
                StoredField::Text { label, description, state } => {
                    let mut e = field().label(label.clone());
                    if let Some(d) = description { e = e.description(d.clone()); }
                    e.child(Input::new(state))
                }
                StoredField::Switch { label, description, checked } => {
                    let mut e = field().label(label.clone());
                    if let Some(d) = description { e = e.description(d.clone()); }
                    e.items_center().child(
                        Switch::new(SharedString::from(format!("auto-form-switch-{i}")))
                            .checked(*checked)
                            .on_click(cx.listener(move |this, val: &bool, _window, cx| {
                                if let Some(StoredField::Switch { checked, .. }) =
                                    this.fields.get_mut(i)
                                {
                                    *checked = *val;
                                }
                                this.emit_values(cx);
                            })),
                    )
                }
            };
            form.child(entry)
        })
    }
}
