use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Mesh(Arc<ResourceInner>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Texture(Arc<ResourceInner>);

struct ResourceInner {
    id: ResourceId,
    loaded: AtomicBool,
    src: Box<dyn DataSource>
}

pub trait Resource: From<ResourceInner> {
    fn inner(&self) -> &ResourceInner;

    fn is_loaded(&self) -> bool {
        self.inner().loaded.load(Ordering::SeqCst)
    }

    fn reload(&self) {
        let inner = self.inner();
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
    pub fn load_resource<T: Resource>(&self, data_src: Box<dyn DataSource>) -> T {
        
    }

    pub fn reload(&self, resource: &impl Resource) {
        todo!("get resource id and send command to reload it")
    }
}