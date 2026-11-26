use gpui::*;

struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .bg(rgba(0x000001FF))
            .size(px(500.0))
            .flex()
            .justify_center()
            .text_lg()
            .text_color(rgb(0xFFFFFF))
            .child(format!("TEST {}", &self.text))
    }
}

pub fn run_window() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(500.0), px(300.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                //titlebar: Option<TitlebarOptions>,
                //A LOT more window options to choose from, i'll have a look later.
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| HelloWorld {
                    text: "📖".into()
                })
            },
        )
        .unwrap();
    });
}
