use std::{
    fmt::{self},
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        mpsc::{SendError, SyncSender},
        Arc,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Mesh(Arc<ResourceInner>);

impl Resource for Mesh {
    fn inner(&self) -> &Arc<ResourceInner> {
        &self.0
    }
}

impl From<ResourceInner> for Mesh {
    fn from(inner: ResourceInner) -> Self {
        Self(Arc::new(inner))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Texture(Arc<ResourceInner>);

impl Resource for Texture {
    fn inner(&self) -> &Arc<ResourceInner> {
        &self.0
    }
}

impl From<ResourceInner> for Texture {
    fn from(inner: ResourceInner) -> Self {
        Self(Arc::new(inner))
    }
}

pub struct ResourceInner {
    id: Id,
    loaded: AtomicBool,
    sender: Sender,
    src: Box<dyn DataSource>,
}

impl ResourceInner {
    pub fn new(sender: Sender, src: Box<dyn DataSource>) -> Self {
        log::info!("Loading resource from: {src}");
        Self {
            id: new_id(),
            loaded: Default::default(),
            sender,
            src,
        }
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded.load(Ordering::SeqCst)
    }

    pub fn load(self: &Arc<Self>) {
        let sender = self.sender.clone();
        let inner = self.clone();
        rayon::spawn(move || {
            let data = match inner.src.load() {
                Ok(d) => d,
                Err(e) => {
                    log::warn!("resource loading failed: {e}");
                    return;
                }
            };

            let action = Action::SetData(inner.id, data);
            sender.send(action);
        });
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

impl Drop for ResourceInner {
    fn drop(&mut self) {
        self.sender.send(Action::Remove(self.id))
    }
}

pub(crate) trait Resource: From<ResourceInner> {
    fn inner(&self) -> &Arc<ResourceInner>;

    fn new(sender: Sender, src: Box<dyn DataSource>) -> Self {
        let inner = ResourceInner::new(sender, src);
        inner.into()
    }

    fn id(&self) -> Id {
        self.inner().id
    }

    fn load(&self) {
        self.inner().load();
    }

    fn is_loaded(&self) -> bool {
        self.inner().is_loaded()
    }
}

pub type Id = u64;

pub trait DataSource: Send + Sync + fmt::Debug + fmt::Display {
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

pub enum Action {
    SetData(Id, Data),
    Remove(Id),
}

#[derive(Clone)]
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

pub fn new_id() -> u64 {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

#[cfg(test)]
mod tests {
    use std::{fmt, sync::mpsc};

    use super::{Action, Data, DataSource, Format, LoadError, Mesh, Resource, Sender};

    #[derive(Debug)]
    struct MeshDataSrc;

    impl fmt::Display for MeshDataSrc {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "MeshDataSrc")
        }
    }

    impl DataSource for MeshDataSrc {
        fn load(&self) -> Result<Data, LoadError> {
            Ok(Data {
                data: Default::default(),
                format: Format::RawMesh,
            })
        }
    }

    #[test]
    fn test_set_data_action() {
        let (tx, rx) = mpsc::sync_channel(10);
        let sender = Sender::new(tx);

        let mesh = Mesh::new(sender, Box::new(MeshDataSrc));
        mesh.load();

        let action = rx.recv().unwrap();
        assert!(matches!(action, Action::SetData(..)));
    }

    #[test]
    fn test_remove_action() {
        let (tx, rx) = mpsc::sync_channel(10);
        let sender = Sender::new(tx);

        let mesh = Mesh::new(sender, Box::new(MeshDataSrc));
        mesh.load();
        let mesh_id = mesh.id();
        std::mem::drop(mesh);

        let _set_data_action = rx.recv().unwrap();
        let remove_action = rx.recv().unwrap();
        if let Action::Remove(id) = remove_action {
            assert_eq!(id, mesh_id)
        } else {
            panic!("remove action expected");
        }
    }
}
