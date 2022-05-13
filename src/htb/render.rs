use crate::htb::view::Gfx;
use glam::Vec2;
use winit::window::Window;

pub struct Render {
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl Render {
    pub fn with_settings(settings: &RenderSettings, window: &Window) -> Option<Self> {
        let instance = wgpu::Instance::new(settings.backends);
        let surface = unsafe { instance.create_surface(window) };
        let (device, queue) = pollster::block_on(Self::request_device(&instance, &surface))?;

        Some(Self { device, queue })
    }

    async fn request_device(
        instance: &wgpu::Instance,
        surface: &wgpu::Surface,
    ) -> Option<(wgpu::Device, wgpu::Queue)> {
        let options = wgpu::RequestAdapterOptions {
            compatible_surface: Some(surface),
            ..Default::default()
        };

        let adapter = instance.request_adapter(&options).await?;
        let limits = wgpu::Limits::downlevel_defaults().using_resolution(adapter.limits());
        let descriptor = wgpu::DeviceDescriptor {
            limits,
            ..Default::default()
        };
        adapter.request_device(&descriptor, None).await.ok()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct RenderSettings {
    backends: wgpu::Backends,
}

impl Default for RenderSettings {
    fn default() -> Self {
        Self {
            backends: wgpu::Backends::all(),
        }
    }
}

impl Gfx for Render {
    fn present(&self) {
        // todo!()
    }
}
