use gpui::*;

struct Window {
    title: &str,
}

impl Render for Window {
    fn render(&mut self, window: &mut gpui::Window, cx: &mut Context<Self>) -> impl IntoElement {
        // code here
    }
}

pub fn start_window() {
    Application::new().run(|_cx: &mut App| {
        // Code here
    });
}
