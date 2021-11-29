use imgui::*;
use imgui_wgpu::{Renderer, RendererConfig};
pub use imgui_wgpu::TextureConfig;
use imgui_winit_support::WinitPlatform;
use pollster::block_on;
use wgpu::{Device, Queue, Surface};

use std::time::Instant;
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

mod config;
pub use config::Config;

mod uploader;
pub use uploader::AssetUploader;



#[derive(Default)]
pub struct Executor {
    event_loop: Option<EventLoop<()>>,
    window: Option<Window>,
    surface: Option<Surface>,
    device: Option<Device>,
    hidpi_factor: Option<f64>,
    imgui: Option<Context>,
    plataform: Option<WinitPlatform>,
    queue: Option<Queue>,
    renderer: Option<Renderer>, 
}

impl Executor {
    pub fn build<
        YourState: 'static,
        ImguiConfig: 'static + Fn(AssetUploader, &mut YourState),
    >(
        &mut self,
        config: &Config<YourState>,
        state: &mut YourState,
        config_imgui: ImguiConfig,
    ) {
        // Set up window and GPU
        let event_loop = EventLoop::new();

        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);

        let (window, size, surface) = {
            let window = Window::new(&event_loop).unwrap();
            window.set_inner_size(LogicalSize {
                width: config.initial_window_width,
                height: config.initial_window_height,
            });
            window.set_title(&config.window_title);
            let size = window.inner_size();

            let surface = unsafe { instance.create_surface(&window) };

            (window, size, surface)
        };

        let hidpi_factor = window.scale_factor();

        let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .unwrap();

        let (device, queue) =
            block_on(adapter.request_device(&wgpu::DeviceDescriptor::default(), None)).unwrap();

        // Set up swap chain
        let surface_desc = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width as u32,
            height: size.height as u32,
            present_mode: wgpu::PresentMode::Fifo,
        };

        surface.configure(&device, &surface_desc);

        // Set up dear imgui
        let mut imgui = imgui::Context::create();
        let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui);
        platform.attach_window(
            imgui.io_mut(),
            &window,
            imgui_winit_support::HiDpiMode::Default,
        );
        imgui.set_ini_filename(None);

        let font_size = config.font_size.unwrap_or((13.0 * hidpi_factor) as f32);
        imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

        imgui.fonts().add_font(&[FontSource::DefaultFontData {
            config: Some(imgui::FontConfig {
                oversample_h: 1,
                pixel_snap_h: true,
                size_pixels: font_size,
                ..Default::default()
            }),
        }]);

        //
        // Set up dear imgui wgpu renderer
        //
        let renderer_config = RendererConfig {
            texture_format: surface_desc.format,
            ..Default::default()
        };

        let mut renderer = Renderer::new(&mut imgui, &device, &queue, renderer_config);

        let asset_uploader = AssetUploader::new(&mut imgui, &mut renderer, &device, &queue);

        //Run the configure function;
        config_imgui(asset_uploader, state);

        renderer.reload_font_texture(&mut imgui, &device, &queue);
        


        self.event_loop = Some(event_loop);
        self.window = Some(window);
        self.surface = Some(surface);
        self.device = Some(device);
        self.hidpi_factor = Some(hidpi_factor);
        self.imgui = Some(imgui);
        self.plataform = Some(platform);
        self.queue = Some(queue);
        self.renderer = Some(renderer);
    }

    pub fn run<YourState: 'static, UiFunction: 'static + Fn(&imgui::Ui, &mut YourState)>(
        self,
        config: Config<YourState>,
        mut state: YourState,
        render_ui: UiFunction,
    ) {

        let event_loop = self.event_loop.unwrap();
        let window = self.window.unwrap();
        let surface = self.surface.unwrap();
        let device = self.device.unwrap();
        let hidpi_factor = self.hidpi_factor.unwrap();
        let mut imgui = self.imgui.unwrap();
        let mut platform = self.plataform.unwrap();
        let queue = self.queue.unwrap();
        let mut renderer = self.renderer.unwrap();

        let mut last_frame = Instant::now();
        let mut last_cursor = None;


        // Event loop        
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent {
                    event: WindowEvent::Resized(_),
                    ..
                } => {
                    let size = window.inner_size();

                    let surface_desc = wgpu::SurfaceConfiguration {
                        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                        format: wgpu::TextureFormat::Bgra8UnormSrgb,
                        width: size.width as u32,
                        height: size.height as u32,
                        present_mode: wgpu::PresentMode::Fifo,
                    };
                    
                    if size.width != 0 && size.height != 0 {
                        surface.configure(&device, &surface_desc);

                        (config.on_resize)(&size, &mut state, hidpi_factor);
                    }
                    
                }
                Event::WindowEvent {
                    event:
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    state: ElementState::Pressed,
                                    ..
                                },
                            ..
                        },
                    ..
                }
                | Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                }

                Event::MainEventsCleared => window.request_redraw(),
                Event::RedrawEventsCleared => {
                    let now = Instant::now();
                    imgui.io_mut().update_delta_time(now - last_frame);
                    last_frame = now;

                    let frame = match surface.get_current_texture() {
                        Ok(frame) => frame,
                        Err(e) => {
                            eprintln!("dropped frame: {:?}", e);
                            return;
                        }
                    };
                    platform
                        .prepare_frame(imgui.io_mut(), &window)
                        .expect("Failed to prepare frame");
                    let ui = imgui.frame();

                    render_ui(&ui, &mut state);

                    let mut encoder: wgpu::CommandEncoder = device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                    if last_cursor != Some(ui.mouse_cursor()) {
                        last_cursor = Some(ui.mouse_cursor());
                        platform.prepare_render(&ui, &window);
                    }

                    let view = frame
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(config.background_color),
                                store: true,
                            },
                        }],
                        depth_stencil_attachment: None,
                    });

                    renderer
                        .render(ui.render(), &queue, &device, &mut rpass)
                        .expect("Rendering failed");

                    drop(rpass);

                    queue.submit(Some(encoder.finish()));
                    frame.present();
                }
                Event::WindowEvent { ref event, .. } => {
                    (config.on_event)(event, &mut state);
                }
                _ => (),
            }

            platform.handle_event(imgui.io_mut(), &window, &event);
        });
    }
}
