pub mod resource;
pub mod scene;
use winit::window::Window;
use self::{resource::ResourceLoader, scene::Scene};

pub struct Render {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
}

impl Render {
    pub fn with_settings(settings: &Settings, window: &Window) -> Option<Self> {
        let instance = wgpu::Instance::new(settings.backends);
        let surface = unsafe { instance.create_surface(window) };
        let (device, queue) = pollster::block_on(Self::request_device(&instance, &surface))?;

        Some(Self {
            device,
            queue,
            surface,
        })
    }

    pub fn resource_loader(&self) -> ResourceLoader {
        todo!()
    }

    pub fn render<T>(&self, scene: &Scene, info: &RenderInfo) {
        todo!()
    }

    async fn request_device(
        instance: &wgpu::Instance,
        surface: &wgpu::Surface,
    ) -> Option<(wgpu::Device, wgpu::Queue)> {
        let options = wgpu::RequestAdapterOptions {
            compatible_surface: Some(surface),
            power_preference: wgpu::PowerPreference::HighPerformance,
            ..Default::default()
        };

        let adapter = instance.request_adapter(&options).await?;
        log::info!("Adapter selected: {}", adapter.get_info().name);

        let limits = wgpu::Limits::downlevel_defaults().using_resolution(adapter.limits());
        let descriptor = wgpu::DeviceDescriptor {
            limits,
            ..Default::default()
        };
        adapter.request_device(&descriptor, None).await.ok()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Settings {
    backends: wgpu::Backends,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            backends: wgpu::Backends::all(),
        }
    }
}

pub struct RenderInfo {
    world: glam::Mat4,
    view: glam::Mat4,
    proj: glam::Mat4,
}