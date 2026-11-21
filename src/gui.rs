use gpui::*;

struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x00000))
            .size(px(300.0))
            .justify_center()
            .text_lg()
            .text_color(rgb(0xFFFFFF))
            .child(format!("TEST {}", &self.text))
    }
}

pub fn run_window() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(5.0), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| HelloWorld {
                    text: "Context Menu Switcher".into(),
                })
            },
        )
        .unwrap();
    });
}
