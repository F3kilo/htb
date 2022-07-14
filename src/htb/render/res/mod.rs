pub mod store;

use std::{
    fmt::{self},
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        mpsc::{self, SendError, SyncSender},
        Arc,
    },
};

use self::store::PlacedData;

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

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Mesh,
    Texture,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Mesh => write!(f, "Mesh"),
            Type::Texture => write!(f, "Texture"),
        }
    }
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

    fn new(src: Box<dyn SourceFactory>, sender: Sender) -> Self {
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

pub trait SourceFactory: Send + Sync + fmt::Debug + fmt::Display {
    fn source(&self) -> Box<dyn Source>;
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
    pub src_factory: Box<dyn SourceFactory>,
    pub loaded: Arc<AtomicBool>,
}

impl Info {
    pub fn new(id: Id, typ: Type, src: Box<dyn SourceFactory>, loaded: Arc<AtomicBool>) -> Self {
        Self {
            id,
            typ,
            src_factory: src,
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

    use super::{Action, Mesh, Resource, Sender, Source, SourceFactory};

    #[derive(Debug)]
    struct MeshDataSrcFactory;

    impl fmt::Display for MeshDataSrcFactory {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "MeshDataSrc")
        }
    }

    impl SourceFactory for MeshDataSrcFactory {
        fn source(&self) -> Box<dyn Source> {
            todo!()
        }
    }

    #[test]
    fn test_actions_sent() {
        let (tx, rx) = mpsc::sync_channel(10);
        let sender = Sender::new(tx);

        let mesh = Mesh::new(Box::new(MeshDataSrcFactory), sender);
        std::mem::drop(mesh);

        let add_action = rx.recv().unwrap();
        assert!(matches!(add_action, Action::Add(..)));
        let remove_action = rx.recv().unwrap();
        assert!(matches!(remove_action, Action::Remove(..)));
    }
}

pub struct Manager {
    sender: Sender,
    thread_pool: rayon::ThreadPool,
    storage: Arc<store::Storage>,
}

impl Manager {
    pub fn new(settings: &Settings, data_sender: SyncSender<PlacedData>) -> Self {
        let (sender, receiver) = mpsc::sync_channel(settings.channel_size);
        let sender = Sender::new(sender);

        let storage = Arc::new(store::Storage::new(&settings.storage));

        let thread_pool = rayon::ThreadPoolBuilder::new()
            .num_threads(settings.io_threads)
            .build()
            .expect("can't initialize rayon thread pool");

        let action_processor = ActionProcessor::new(receiver, storage.clone(), data_sender);

        thread_pool.install(move || action_processor.run());

        Self {
            sender,
            thread_pool,
            storage,
        }
    }

    pub fn sender(&self) -> Sender {
        self.sender.clone()
    }
}

struct ActionProcessor {
    receiver: mpsc::Receiver<Action>,
    storage: Arc<store::Storage>,
    data_sender: SyncSender<PlacedData>,
}

impl ActionProcessor {
    pub fn new(
        receiver: mpsc::Receiver<Action>,
        storage: Arc<store::Storage>,
        data_sender: SyncSender<PlacedData>,
    ) -> Self {
        Self {
            receiver,
            storage,
            data_sender,
        }
    }

    pub fn run(self) {
        for action in self.receiver.iter() {
            match action {
                Action::Add(info) => self.add(info),
                Action::Remove(id) => self.remove(id),
            }
        }
    }

    pub fn add(&self, info: Info) {
        let source = info.src_factory.source();
        let storage = self.storage.clone();
        let sender = self.data_sender.clone();
        rayon::spawn(move || {
            let data = match source.load() {
                Ok(data) => data,
                Err(e) => {
                    log::warn!("Can't load resoure from {source}: {e}");
                    return;
                }
            };

            let resource_type = info.typ;
            let to_load = if let Some(to_load) = storage.add(info, &data) {
                to_load
            } else {
                log::warn!(
                    "Not enough place in storage for {} resource from {source}",
                    resource_type
                );
                return;
            };

            let placed_data = PlacedData {
                data: data.data,
                to_load,
            };
            if sender.send(placed_data).is_err() {
                log::warn!(
                    "Try to send resoure data from {source} to gpu loader, but receiver is dead"
                );
            }
        })
    }

    pub fn remove(&self, id: Id) {
        self.storage.remove(id)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Settings {
    pub channel_size: usize,
    pub io_threads: usize,
    pub storage: store::Settings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            channel_size: 1024,
            io_threads: 32,
            storage: store::Settings {},
        }
    }
}
