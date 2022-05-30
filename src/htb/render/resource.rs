use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Mesh(Arc<ResourceInner>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Texture(Arc<ResourceInner>);

pub(crate) struct ResourceInner {
    id: ResourceId,
    loaded: AtomicBool,
    loader: ResourceLoader,
    src: Box<dyn DataSource>
}

pub trait Resource: From<ResourceInner> {
    fn new(loader: ResourceLoader, data_src: Box<dyn DataSource>) -> Self {

    }

    fn inner(&self) -> &ResourceInner;

    fn is_loaded(&self) -> bool {
        self.inner().loaded.load(Ordering::SeqCst)
    }
}

pub type ResourceId = u64;

pub trait DataSource: Send {
    fn load(&self) -> Result<ResourceData, ResourceResult>;
    fn duplicate(&self) -> Box<dyn DataSource>;
}


#[derive(Debug, thiserror::Error)]
pub enum ResourceResult {
    #[error("resource loading failed: {0}")]
    LoadFailed(String),
}

pub trait ResourceData {}

pub struct ResourceLoader {}

impl ResourceLoader {
    fn load(&self, resource: Arc<ResourceInner>) {
        if resource.is_loaded() {
            log::info!("try to load resource which is loaded already {resource}");
            return;
        }

        
    }
}