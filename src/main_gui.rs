use std::string;

use gpui::*;

const WINDOW_TITLE: &str = "Context Menu Switcher";

struct Window {
    title: &str,
    size: u32,             //implement windows size
    background_color: u32, //Implement background color
}

impl Render for Window {
    fn render(&mut self, window: &mut gpui::Window, cx: &mut Context<Self>) -> impl IntoElement {
        // TODO
    }
}

pub fn start_window() {
    Application::new().run(|_cx: &mut App| {
        // Proper way is to declare a struct, implement rendering for sayed struct and call it inside a function.
    });
}
