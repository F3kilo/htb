use std::ops::Range;

use super::{Data, Id, Info};

pub struct Storage {}

impl Storage {
    pub fn new(settings: &Settings) -> Self {
        Self {}
    }

    pub fn add(&self, info: Info, data: &Data) -> Option<Vec<DataCopy>> {
        todo!()
    }

    pub fn remove(&self, id: Id) {
        todo!()
    }
}

pub enum LoadDst {
    IndexBuffer,
    VertexBuffer,
}

pub struct DataCopy {
    pub dst: LoadDst,
    pub from: Range<usize>,
    pub to: Range<usize>,
}

pub struct PlacedData {
    pub to_load: Vec<DataCopy>,
    pub data: Vec<u8>,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Settings {}
