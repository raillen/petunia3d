//! Pure loop-cut preview state. The UI owns confirmation/cancel and history.

#![forbid(unsafe_code)]

use glam::Vec3;
use petunia_mesh::{
    loop_cut::{LoopCutError, LoopRing},
    Mesh,
};

#[derive(Clone)]
pub struct LoopCutPreview {
    pub cuts: usize,
    pub slide: f32,
    ring: LoopRing,
    source: Mesh,
}

impl LoopCutPreview {
    pub fn new(mesh: &Mesh, edge: (u32, u32)) -> Result<Self, LoopCutError> {
        let ring = LoopRing::discover(mesh, edge)?;
        Ok(Self {
            cuts: 1,
            slide: 0.0,
            ring,
            source: mesh.clone(),
        })
    }

    pub fn segments(&self) -> Result<Vec<[Vec3; 2]>, LoopCutError> {
        self.ring.preview(&self.source, self.cuts, self.slide)
    }

    /// Always rebuilds from the original snapshot; pointer updates never accumulate cuts.
    pub fn mesh(&self) -> Result<Mesh, LoopCutError> {
        self.ring.apply(&self.source, self.cuts, self.slide)
    }

    pub fn is_closed(&self) -> bool {
        self.ring.is_closed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeated_slide_previews_rebuild_same_topology() {
        let source = Mesh::cube(2.0);
        let mut preview = LoopCutPreview::new(&source, source.edges_unique()[0]).unwrap();
        preview.cuts = 3;
        for slide in [-0.9, 0.5, 0.0] {
            preview.slide = slide;
            let mesh = preview.mesh().unwrap();
            assert_eq!(mesh.verts.len(), 20);
            assert_eq!(mesh.faces.len(), 18);
            assert_eq!(preview.segments().unwrap().len(), 12);
        }
        assert_eq!(source.verts.len(), 8);
        assert_eq!(source.faces.len(), 6);
    }
}
