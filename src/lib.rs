use std::sync::Arc;
use wgpu::{Backends, Dx12Compiler, PowerPreference};
#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

use winit::
{
    event::*,
    event_loop::
    {
        ControlFlow,
        EventLoop
    },
    window::WindowBuilder
};
use winit::window::CursorIcon::Default;
use winit::window::Window;

struct State{
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue, // Buffer of GPU instructions
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: winit::window::Window
}

impl State{
    async fn new(window: Window) -> Self{
        let size = window.inner_size();
        
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        
        // Actual area to draw something on that
        let surface = unsafe{ instance.create_surface(&window) }.unwrap();
        
        // Adapter between app and actual GPU driver
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions{
            compatible_surface : Some(&surface),
            force_fallback_adapter: false,
            power_preference: PowerPreference::HighPerformance
        }).await.unwrap();
        
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor{
                features: wgpu::Features::empty(),
                limits: if cfg!(target_arch = "wasm32"){
                    wgpu::Limits::downlevel_webgl2_defaults()
                }
                else { 
                    wgpu::Limits::default()
                },
                label: None,
            },
            None
        ).await.unwrap();
        
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        
        let config = wgpu::SurfaceConfiguration{
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: Vec::new()
        };
        
        surface.configure(&device, &config);
        
        Self {
            surface,
            device,
            queue,
            config,
            size,
            window
        }
    }
    
    pub fn window(&self) -> &Window {
        &self.window
    }
    
    fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>){
        if size.height > 0 && size.width > 0 {
            self.size = size;
            self.config.width = size.width;
            self.config.height = size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }
    
    fn input(&mut self, event: &WindowEvent) -> bool{
        false
    }
    
    fn update(&mut self){
        // ToDo: Still Empty
    }
    
    fn render(&mut self) -> Result<(), wgpu::SurfaceError>{
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        
        // Modern GPUS expect their commands to be written inside buffer. That's why we are creating
        // encoder, which represents GPU instruction and than passing it in queue
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder")
        });
        
        {
            let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor{
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment{
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations{
                        load: wgpu::LoadOp::Clear(wgpu::Color{
                            r: 0.5,
                            g: 0.4,
                            b: 0.9,
                            a: 1.0,
                        }),
                        store: true
                    }
                })],
                depth_stencil_attachment: None,
            });
        }
        
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        
        Ok(())
    }
}

pub async fn run(){
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }
    
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        // Winit prevents sizing with CSS, so we have to set
        // the size manually when on web.
        use winit::dpi::PhysicalSize;
        window.set_inner_size(PhysicalSize::new(450, 400));

        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("wasm-example")?;
                let canvas = web_sys::Element::from(window.canvas());
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }
    
    let mut state = State::new(window).await;

    event_loop.run(move |event, _ , control_flow| match event{
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == state.window.id() => if !state.input(event) {
            println!("Win Event - 3");
            match event {
                WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                    input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                    ..
                } => *control_flow = ControlFlow::Exit,
                
                _ => {}
            }
        },
        
        Event::RedrawRequested(window_id) if window_id == state.window.id() => {
            println!("Redraw - 2");
            state.update();
            match state.render() {
                Ok(_) => {},
                Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => eprintln!("{:?}", e)
            }
        },

        Event::MainEventsCleared => {
            println!("Main Event Cleared - 1");
            state.window().request_redraw();
        }
        _ => {}
    });
}
