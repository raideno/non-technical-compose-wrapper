use anyhow::Error;
use gpui::*;
use gpui_component::{ActiveTheme, Root};

use docker_compose_types::Compose;
use screens::entry::EntryScreen;
use screens::router::NavigationEvent;
use screens::services::ServicesScreen;

mod components;
mod screens;

enum Screen {
    Entry(Entity<EntryScreen>),
    Services(Entity<ServicesScreen>),
}

struct NavigatorView {
    screen: Screen,
    services: Vec<String>,
}

impl NavigatorView {
    fn new(services: Vec<String>, context: &mut Context<Self>) -> Self {
        let entry = context.new(|_| EntryScreen::new("docker-compose.yaml".to_string()));

        context
            .subscribe(&entry, |this, _entity, event: &NavigationEvent, cx| {
                println!("From {}", event.from);
                let services_screen = cx.new(|_| ServicesScreen::new(this.services.clone()));
                this.screen = Screen::Services(services_screen);
                cx.notify();
            })
            .detach();

        Self {
            screen: Screen::Entry(entry),
            services,
        }
    }
}

impl Render for NavigatorView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let dialog_layer = Root::render_dialog_layer(window, cx);

        let content: AnyElement = match &self.screen {
            Screen::Entry(entry) => entry.clone().into_any_element(),
            Screen::Services(services) => services.clone().into_any_element(),
        };

        div()
            .size_full()
            .bg(cx.theme().background)
            .child(content)
            .children(dialog_layer)
    }
}

fn main() {
    // /* TODO: later should be provided at startup, with a default value, etc */
    // let yaml = std::fs::read_to_string("docker-compose.yaml").unwrap();
    // let compose: Compose = serde_yaml::from_str(&yaml).unwrap();

    // let services: Vec<String> = compose.services.0.keys().cloned().collect();

    // println!("Services:");
    // for name in &services {
    //     println!("  - {name}");
    // }

    let application = Application::new().with_assets(gpui_component_assets::Assets);

    application.run(move |cx| {
        gpui_component::init(cx);

        // let services = services.clone();
        let services = vec![
            "web".to_string(),
            "db".to_string(),
            "cache".to_string(),
            "worker".to_string(),
        ];

        cx.spawn(async move |cx| {
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: point(px(100.), px(100.)),
                        size: size(px(1024.), px(768.)),
                    })),
                    ..Default::default()
                },
                move |window, cx| {
                    let navigator = cx.new(|cx| NavigatorView::new(services, cx));
                    cx.new(|cx| Root::new(navigator, window, cx))
                },
            )?;

            Ok::<_, Error>(())
        })
        .detach();
    });
}
