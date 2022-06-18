use std::{
    fmt::{self},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Mesh(Arc<ResourceInner>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Texture(Arc<ResourceInner>);

pub(crate) struct ResourceInner {
    id: ResourceId,
    loaded: AtomicBool,
    sender: ResourceSender,
    src: Box<dyn DataSource>,
}

impl ResourceInner {
    pub fn new(sender: ResourceSender, src: Box<dyn DataSource>) -> Self {
        Self {
            id: new_id(),
            loaded: Default::default(),
            sender,
            src,
        }
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded.load(Ordering::SeqCst)
    }
}

impl fmt::Debug for ResourceInner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ResourceInner")
            .field("id", &self.id)
            .field("loaded", &self.loaded)
            .field("src", &self.src)
            .finish()
    }
}

impl fmt::Display for ResourceInner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let state = match self.is_loaded() {
            true => "loaded",
            false => "not loaded",
        };
        write!(f, "{state} resorce #{} from: {}", self.id, self.src)
    }
}

impl PartialEq for ResourceInner {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for ResourceInner {}

impl std::hash::Hash for ResourceInner {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

pub(crate) trait Resource: From<ResourceInner> {
    fn inner(&self) -> &ResourceInner;

    fn new(sender: ResourceSender, src: Box<dyn DataSource>) -> Self {
        let inner = ResourceInner::new(sender, src);

        inner.into()
    }

    fn is_loaded(&self) -> bool {
        self.inner().is_loaded()
    }
}

pub type ResourceId = u64;

pub trait DataSource: Send + fmt::Debug + fmt::Display {
    fn load(&self) -> Result<ResourceData, ResourceResult>;
}

#[derive(Debug, thiserror::Error)]
pub enum ResourceResult {
    #[error("resource loading failed: {0}")]
    LoadFailed(String),
}

pub struct ResourceData {}

#[derive(Clone)]
pub struct ResourceSender {}

impl ResourceSender {
    fn send(&self, resource: Arc<ResourceInner>) {
        todo!()
    }
}

pub fn new_id() -> u64 {
    todo!()
}
