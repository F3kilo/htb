use std::{
    fmt::{self},
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        mpsc::{SendError, SyncSender},
        Arc,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Mesh(Arc<Inner>);

impl Resource for Mesh {
    fn inner(&self) -> &Arc<Inner> {
        &self.0
    }

    fn typ() -> Type {
        Type::Mesh
    }
}

impl From<Inner> for Mesh {
    fn from(inner: Inner) -> Self {
        Self(Arc::new(inner))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Texture(Arc<Inner>);

impl Resource for Texture {
    fn inner(&self) -> &Arc<Inner> {
        &self.0
    }

    fn typ() -> Type {
        Type::Texture
    }
}

impl From<Inner> for Texture {
    fn from(inner: Inner) -> Self {
        Self(Arc::new(inner))
    }
}

#[derive(Debug)]
pub enum Type {
    Mesh,
    Texture,
}

pub struct Inner {
    id: UniqueId,
    loaded: Arc<AtomicBool>,
}

impl Inner {
    pub fn new(id: UniqueId, loaded: Arc<AtomicBool>) -> Self {
        Self { id, loaded }
    }

    pub fn id(&self) -> &UniqueId {
        &self.id
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded.load(Ordering::SeqCst)
    }
}

impl fmt::Debug for Inner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ResourceInner")
            .field("id", &self.id)
            .field("loaded", &self.loaded)
            .finish()
    }
}

impl fmt::Display for Inner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let state = match self.is_loaded() {
            true => "loaded",
            false => "not loaded",
        };
        write!(f, "{state} resource #{}", self.id)
    }
}

impl PartialEq for Inner {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Inner {}

impl std::hash::Hash for Inner {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

pub(crate) trait Resource: From<Inner> {
    fn inner(&self) -> &Arc<Inner>;

    fn typ() -> Type;

    fn new(src: Box<dyn Source>, sender: Sender) -> Self {
        let id = UniqueId::new(sender.clone());
        let loaded = Arc::<AtomicBool>::default();
        log::trace!("Sending Action::Add with {id} from: {src}");

        let info = Info::new(id.raw(), Self::typ(), src, loaded.clone());
        sender.send(Action::Add(info));
        let inner = Inner::new(id, loaded);
        inner.into()
    }

    fn is_loaded(&self) -> bool {
        self.inner().is_loaded()
    }
}

pub type Id = u64;

pub struct UniqueId {
    id: Id,
    sender: Sender,
}

impl UniqueId {
    pub fn new(sender: Sender) -> Self {
        Self {
            id: new_id(),
            sender,
        }
    }

    pub fn raw(&self) -> Id {
        self.id
    }
}

impl fmt::Debug for UniqueId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ResourceInner")
            .field("id", &self.id)
            .field("sende", &self.sender)
            .finish()
    }
}

impl fmt::Display for UniqueId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unique id {}", self.id)
    }
}

impl PartialEq for UniqueId {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for UniqueId {}

impl std::hash::Hash for UniqueId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Drop for UniqueId {
    fn drop(&mut self) {
        self.sender.send(Action::Remove(self.id))
    }
}

pub trait Source: Send + Sync + fmt::Debug + fmt::Display {
    fn load(&self) -> Result<Data, LoadError>;
}

#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("resource loading failed: {0}")]
    Failed(String),
}

#[derive(Clone)]
pub struct Data {
    format: Format,
    data: Vec<u8>,
}

#[derive(Debug, Copy, Clone)]
pub enum Format {
    RawMesh,
    Ktx2,
}

#[derive(Debug)]
pub struct Info {
    pub id: Id,
    pub typ: Type,
    pub src: Box<dyn Source>,
    pub loaded: Arc<AtomicBool>,
}

impl Info {
    pub fn new(id: Id, typ: Type, src: Box<dyn Source>, loaded: Arc<AtomicBool>) -> Self {
        Self {
            id,
            typ,
            src,
            loaded,
        }
    }
}

pub enum Action {
    Add(Info),
    Remove(Id),
}

#[derive(Debug, Clone)]
pub struct Sender {
    sender: SyncSender<Action>,
}

impl Sender {
    pub fn new(sender: SyncSender<Action>) -> Self {
        Self { sender }
    }

    pub fn send(&self, action: Action) {
        let send_result = self.sender.send(action);
        if let Err(SendError(res)) = send_result {
            log::warn!("Can't send resource action");
        }
    }
}

pub fn new_id() -> Id {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

#[cfg(test)]
mod tests {
    use std::{fmt, sync::mpsc};

    use super::{Action, Data, Format, LoadError, Mesh, Resource, Sender, Source};

    #[derive(Debug)]
    struct MeshDataSrc;

    impl fmt::Display for MeshDataSrc {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "MeshDataSrc")
        }
    }

    impl Source for MeshDataSrc {
        fn load(&self) -> Result<Data, LoadError> {
            Ok(Data {
                data: Default::default(),
                format: Format::RawMesh,
            })
        }
    }

    #[test]
    fn test_actions_sent() {
        let (tx, rx) = mpsc::sync_channel(10);
        let sender = Sender::new(tx);

        let mesh = Mesh::new(Box::new(MeshDataSrc), sender);
        std::mem::drop(mesh);

        let add_action = rx.recv().unwrap();
        assert!(matches!(add_action, Action::Add(..)));
        let remove_action = rx.recv().unwrap();
        assert!(matches!(remove_action, Action::Remove(..)));
    }
}
