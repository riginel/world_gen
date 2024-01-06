
use std::ops::Range;
use noise::{NoiseFn, Perlin};
use crate::utils::generator_error::GeneratorError;
use crate::utils::tile::PreTileType;

pub struct NoiseDist{
    dist: Vec<(Range<usize>, PreTileType)>
}
impl NoiseDist{
    pub fn new(dist: Vec<(Range<usize>, PreTileType)>) ->Self{
        Self{
            dist
        }
    }
    pub fn get(&self, position:usize)-> PreTileType {
        for (range,tile) in self.dist.iter(){
            if range.contains(&position){
                return tile.clone();
            }
        }
        PreTileType::None
    }
}
impl Default for NoiseDist{
    fn default() -> Self {
        let vec = vec![
            (0..32, PreTileType::DeepWater),
            (32..42, PreTileType::Water),
            (42..52, PreTileType::Zone(0)),
            (52..70, PreTileType::Grass),
            (70..81, PreTileType::Zone(1)),
            (81..100, PreTileType::Zone(2))
        ];
        Self{
            dist:vec
        }
    }
}
impl PreTileType {
    pub fn noise_to_tile(z:f64,noise_distribution: &NoiseDist)->Self{
        //normalize z to an usize between 0 and 100
        let normalized_z: i64 = (((z/2.0)+0.5)*100.0).floor() as i64;
        if normalized_z < 0 || normalized_z >=100{
            return PreTileType::None
        }
        noise_distribution.get(normalized_z as usize)
    }
}
#[cfg(test)]
mod tests{
    use crate::customization::noise_to_tile::NoiseDist;
    use crate::utils::tile::PreTileType;

    #[test]
    fn test(){
        let dist = NoiseDist::default();
        assert_eq!(PreTileType::Grass, PreTileType::noise_to_tile(0.07, &dist));

    }
}
pub struct NoiseBundle {
    noise_fn:Box<dyn NoiseFn<f64,2>>,
    octaves:i32,
    scale:f64
}
impl Default for NoiseBundle {
    fn default() -> Self {
        Self{
            noise_fn: Box::new(Perlin::new(420)),
            octaves: 3,
            scale:4.3
        }
    }

}
impl NoiseBundle where {
    pub fn new(noise_fn:Box<dyn NoiseFn<f64,2>>, octaves:i32,scale:f64)->Self{
        Self{
            noise_fn,
            octaves,
            scale
        }
    }
    pub fn generate_tiles_and_elevation(&self, size:usize,dist: NoiseDist,elevation_factor: f64)->(Vec<Vec<PreTileType>>, Vec<Vec<usize>>){
        let scale = self.scale;
        let octaves = self.octaves;

        let mut tiles: Vec<Vec<PreTileType>> = Vec::with_capacity(size);
        let mut elevation_vec: Vec<Vec<usize>> = Vec::with_capacity(size);

        for x in 0..size{
            let nx = x as f64/size as f64;
            let mut row = Vec::with_capacity(size);
            let mut elevation_row = Vec::with_capacity(size);
            for y in 0..size{
                let ny = y as f64 /size as f64;
                let mut elevation = self.noise_fn.get([nx*scale,ny*scale]);
                for j in 1..octaves{
                    elevation +=(1.0/j as f64) * self.noise_fn.get(
                        [nx * scale * 2.0_f64.powi(j),ny * scale *2.0_f64.powi(j) ]);
                }
                elevation /= 1.875; //TODO adjust automatically after the octaves number
                row.push(PreTileType::noise_to_tile(elevation, &dist));
                elevation_row.push ((((elevation/2.0)+0.5)*100.0* elevation_factor).floor() as usize);

            }
            tiles.push(row);
            elevation_vec.push(elevation_row);
        }

        (tiles,elevation_vec)
    }
}

