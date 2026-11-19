use gpui::*;

pub fn start_window() {
    Application::new().run(|_cx: &mut App| {
        // Proper way is to declare a struct, implement rendering for sayed struct and call it inside a function.
    })
}
