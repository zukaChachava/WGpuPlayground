use wgpu::{Backends, Dx12Compiler};
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
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: winit::window::Window
}

impl State{
    async fn new(window: Window) -> Self{
        let size = window.inner_size();
        
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let surface = unsafe{ instance.create_surface(&window) }.unwrap();
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions{
            compatible_surface : Some(&surface),
            force_fallback_adapter: false,
            power_preference: wgpu::PowerPreference::default()
        }).await.unwrap();
        
        todo!()
    }
    
    pub fn window(&self) -> &Window {
        todo!()
    }
    
    fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>){
        todo!()
    }
    
    fn input(&mut self, event: &WindowEvent) -> bool{
        todo!()
    }
    
    fn update(&mut self){
        todo!()
    }
    
    fn render(&mut self) -> Result<(), wgpu::SurfaceError>{
        todo!()
    }
}

pub fn run(){
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

    event_loop.run(move |event, _ , control_flow| match event{
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event   {
            WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                input:
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Escape),
                    ..
                },
                ..
            }
            => *control_flow = ControlFlow::Exit,
            _ => {}
        },
        _ => {}
    });
}
