use voxelize::{
    chunk::Chunk,
    pipeline::{ChunkStage, ResourceRequirements, ResourceResults},
    world::voxels::{access::VoxelAccess, space::Space},
};

pub struct WaterStage;

impl ChunkStage for WaterStage {
    fn name(&self) -> String {
        "Water".to_owned()
    }

    fn needs_resources(&self) -> ResourceRequirements {
        ResourceRequirements {
            needs_noise: false,
            needs_config: true,
            needs_registry: true,
        }
    }

    fn process(&self, mut chunk: Chunk, resources: ResourceResults, _: Option<Space>) -> Chunk {
        let config = resources.config.unwrap();
        let registry = resources.registry.unwrap();

        let water_level = config.water_level as i32;
        let water = registry.get_block_by_name("Water");

        for vx in chunk.min.0..chunk.max.0 {
            for vz in chunk.min.2..chunk.max.2 {
                let height = chunk.get_max_height(vx, vz) as i32;
                for vy in (height + 1)..water_level {
                    chunk.set_voxel(vx, vy, vz, water.id);
                }
            }
        }

        chunk
    }
}
