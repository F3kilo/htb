use std::{ops::Range, sync::RwLock};

use dashmap::DashMap;
use num::Integer;
use subranges::{FreeIntervals, Interval};

use super::{Data, Id, MeshMetadata, Metadata, Type};

pub struct Storage {
    mesh_buffer: RwLock<FreeIntervals>,
    metadata: DashMap<Id, Stored>,
}

impl Storage {
    pub fn new(settings: &Settings) -> Self {
        let length = settings.mesh_buffer_length;
        let mesh_buffer = RwLock::new(FreeIntervals::new((0..length).into()));
        let metadata = DashMap::with_capacity(settings.init_resources_count);
        Self {
            mesh_buffer,
            metadata,
        }
    }

    pub fn add(&self, id: Id, data: &Data) -> Option<DataCopy> {
        match data.meta {
            Metadata::Mesh(meta) => self.add_mesh(id, meta),
            Metadata::Texture(_) => todo!(),
        }
    }

    fn add_mesh(&self, id: Id, meta: MeshMetadata) -> Option<DataCopy> {
        const INDEX_SIZE: u64 = 4;
        let alignment = meta.vertex_size.lcm(&INDEX_SIZE);
        let data_len = meta.vertex_offset + meta.vertex_count * meta.vertex_size - meta.data_offset;
        let interval = self
            .mesh_buffer
            .write()
            .unwrap()
            .take_exact_aligned(data_len, alignment)?;

        let stored = Stored {
            meta: Metadata::Mesh(meta),
            interval,
        };
        self.metadata.insert(id, stored);

        let from = meta.data_offset as usize..data_len as usize;
        Some(DataCopy {
            from,
            dst: LoadDst::MeshBuffer,
            interval,
        })
    }

    pub fn remove(&self, id: Id, typ: Type) -> Option<Metadata> {
        match typ {
            Type::Mesh => self.remove_mesh(id),
            Type::Texture => todo!(),
        }
    }

    fn remove_mesh(&self, id: Id) -> Option<Metadata> {
        let (_, meta) = self.metadata.remove(&id)?;
        self.mesh_buffer.write().unwrap().insert(meta.interval);
        Some(meta.meta)
    }
}

pub enum LoadDst {
    MeshBuffer,
}

pub struct DataCopy {
    pub from: Range<usize>,
    pub dst: LoadDst,
    pub interval: Interval,
}

pub struct PlacedData {
    pub to_load: DataCopy,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Copy)]
pub struct Settings {
    mesh_buffer_length: u64,
    init_resources_count: usize,
}

impl Default for Settings {
    fn default() -> Self {
        const MB: u64 = 1024 * 1024;
        Self {
            mesh_buffer_length: 128 * MB,
            init_resources_count: 1024,
        }
    }
}

pub struct Stored {
    pub meta: Metadata,
    pub interval: Interval,
}
