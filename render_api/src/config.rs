/// use `Default::default` if you don't need anything specific.
pub struct Config<State: 'static> {
    /// name of the window
    pub window_title: String,
    /// can be used to resize the window
    pub initial_window_width: f32,
    /// can be used to resize the window
    pub initial_window_height: f32,
    /// if you want to adjust your imgui window to match the size of the outer window
    /// this makes it possible to have a "fullscreen" imgui window spanning the whole current window.
    pub on_resize: &'static dyn Fn(&winit::dpi::PhysicalSize<u32>, &mut State, f64),
    /// called after the premade events have been handled which includes close request
    /// if you think you need to handle this, this api abstraction is probably to high level
    /// and you may want to copy the code from hello_world.rs and adapt directly
    pub on_event: &'static dyn Fn(&winit::event::WindowEvent<'_>, &mut State),
    /// font size
    pub font_size: Option<f32>,
    /// color that fills the window
    pub background_color: wgpu::Color,
}

impl<State> Default for Config<State> {
    fn default() -> Self {
        Self {
            window_title: "imgui".to_string(),
            initial_window_width: 1200.0,
            initial_window_height: 720.0,
            on_resize: &|_, _, _| {},
            on_event: &|_, _| {},
            font_size: None,
            background_color: wgpu::Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
        }
    }
}