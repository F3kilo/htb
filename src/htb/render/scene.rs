use super::res::{Mesh, Texture};
use std::collections::HashMap;

pub struct Scene {
    colored_mesh: HashMap<Mesh, Vec<ColorInstance>>,
    textured_mesh: HashMap<TexturedMesh, Vec<TextureInstance>>,
}

impl Scene {
    pub fn add_colored_mesh(&mut self, mesh: Mesh, instances: &[ColorInstance]) {
        let entry = self.colored_mesh.entry(mesh).or_default();
        entry.extend_from_slice(instances);
    }

    pub fn add_textured_mesh(
        &mut self,
        textured_mesh: TexturedMesh,
        instances: &[TextureInstance],
    ) {
        let entry = self.textured_mesh.entry(textured_mesh).or_default();
        entry.extend_from_slice(instances);
    }

    pub fn clear(&mut self) {
        self.colored_mesh.clear();
        self.textured_mesh.clear();
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct TexturedMesh {
    mesh: Mesh,
    texture: Texture,
}

#[derive(Clone, Copy)]
pub struct ColorInstance {
    transform: glam::Mat4,
    color: glam::Vec4,
}

#[derive(Clone, Copy)]
pub struct TextureInstance {
    transform: glam::Mat4,
    uv_transform: glam::Vec4,
    color: glam::Vec4,
}
