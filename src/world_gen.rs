use noise::{Abs, Add, Constant, Displace, Fbm, Multiply, NoiseFn, OpenSimplex, Perlin, PerlinSurflet, ScaleBias, ScalePoint, Simplex, Value, Worley};
use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};



use std::cmp;
use std::time::Instant;
use noise::*;
use rand::prelude::*;
use image::{Rgb, RgbImage};
use rand::seq::SliceRandom;

use crate::pathfinding::a_star::*;

#[derive(Debug, Copy, Clone)]
pub enum Tile {
    Grass,
    Water,
    DeepWater,
    Sand,
    Mountain,
    Lava,
    Zone(usize), // Used in intermediate world generation.
    Road,
    None,

}

fn trans(z: f64) -> Tile {
    // Trans function (because we are using RustLang).
    // Perlin Noise return a -1.0 <= f64 <= 1.0,
    // This function normalize(?) the range between [0.0, 1.0]
    // and map each value to a TileType using the below rules.
    // TODO adjust the conversion rules.
    match z / 2.0 + 0.5 {
        w if w < 0.32 => {Tile::DeepWater}
        w if w < 0.42 => {Tile::Water}
        w if w < 0.52 => {Tile::Zone(0)}
        w if w < 0.7 => {Tile::Grass}
        w if w <= 0.81 => {Tile::Mountain}
        w if w <= 1.0 => {Tile::Lava}
        _ => {Tile::Lava}
    }
}

fn bfs(world: &mut Vec<Vec<Tile>>, id: usize, point: (usize, usize)) -> Vec<(usize, usize)> {
    // Fun fact: Jag has no idea if this is a BFS or a DFS :-)
    // This function takes the World and return a Vec with the points
    // of a contiguos area of TileType::Zone(id).
    // But Jag there is no TileType::Zone(usize) in the common crate, so why?
    // Used to place a diffent TileType for each contiguos Zone(usize) area,
    // like the top of a mountain can be still a mountain or lava.
    // Side effect: convert every TileType::Zone(id) to TileType::None.

    let mut vec = Vec::<(usize, usize)>::new();
    let mut tmp = Vec::<(usize, usize)>::new();
    tmp.push(point);
    while tmp.len() > 0 {
        let (x, y) = tmp.pop().unwrap();
        match world[x][y] {
            Tile::Zone(i) => {
                if i == id  {
                    vec.push((x, y));

                    tmp.push((cmp::min(world.len() - 1, x + 1), y));
                    tmp.push((x, cmp::min(world[0].len() - 1, y + 1)));
                    match x.checked_sub(1) {
                        Some(w) => {
                            tmp.push((w, y));
                        }
                        None => {}
                    };
                    match y.checked_sub(1) {
                        Some(w) => {
                            tmp.push((x, w));
                        }
                        None => {}
                    };
                    // Avoid infinite loop behaves like a black hole
                    // for RAM resources.
                    // Yeah during the refactoring i forgot this line
                    // and had to curse against my will.
                    world[x][y] = Tile::None;
                }
            }
            _ => {}
        };
    }
    vec
}

pub fn gen_world(width: usize, height: usize) -> Vec<Vec<Tile>> {
    let seed = /*rand::random::<u32>()*/ 52;
    let perlin = Perlin::new(seed);
    let scale = 4.2; // TODO adjust automatically based on map size.
    // scale rappresent zoom factor of the map,
    // higher the scale lower the zoom factor.

    let octaves = 3; // Number of octaves added to the Perlin Noise
    // to obtain a better looking map.

    let mut world = Vec::with_capacity(width);
    for x in 0..width {
        let nx = x as f64 / width as f64;
        let mut row = Vec::with_capacity(height);
        for y in 0..height {
            let ny = y as f64 / height as f64;
            let mut elevation = perlin.get([nx * scale, ny * scale]);
            for j in 1..octaves {
                elevation += (1.0 / j as f64) * perlin.get(
                    [nx * scale * 2.0_f64.powi(j), ny * scale * 2.0_f64.powi(j)]);
            }
            elevation /= 1.875; // TODO adjust automatically after the octaves number.
            row.push(trans(elevation)); // TO refactor
        }
        world.push(row);
    }

    // Rest In Peace beatiful line of code ->
    // world.iter().map(|col| col.iter().map(|tile_prob| Tile {rtype: trans(*tile_prob)}).collect()).collect()

    /*
    let mut zones = Vec::<(Tile, Vec<(usize, usize)>)>::new();
    for x in 0..world.len() {
        for y in 0..world[0].len() {
            match world[x][y] {
                Tile::Zone(i) => {
                    zones.push((Tile::Zone(i), bfs(&mut world, i, (x, y))));
                },
                _ => {},
            };
        }
    }
    while zones.len() > 0 {
        let mut rng = thread_rng();
        let zone = zones.pop().unwrap();
        match zone.0 {
            Tile::Zone(0) => {
                let options = [Tile::Sand, Tile::Grass];
                let tile = *options.choose(&mut rng).unwrap();
                for (a, b) in zone.1 {
                    world[a][b] = tile;
                }
            }
            _ => {}
        };

    }
    */

    let mut zones = Vec::<Zone>::new();
    for x in 0..world.len(){
        for y in 0..world[0].len(){
            match world[x][y]{

                Tile::Zone(i) => {
                    zones.push(Zone::bfs(&mut world,i,Point::new(x,y)))
                },
                _ => {}
            }
        }
    }
    for i in zones.iter_mut(){
        let mut rng = thread_rng();
        let tile = *[Tile::Sand, Tile::Grass].choose(&mut rng).unwrap();
        i.fill(&mut world,tile);
    }

    //pathfinding test
    let start = Instant::now();
    let road = shortest_priority(&world, Point::new(20,20),Point::new(1000,1000));
    match road {
        Ok(path) => { for i in path {
            world[i.x][i.y] = Tile::Road
        }}
        Err(s) => {println!("{}",s)}
    }
    let duration = start.elapsed();
    println!("{:?}",duration);
    /*
    let road = shortest_path(&world, Point::new(190,190),Point::new(20,20));
    match road {
        Ok(path) => { for i in path {
            world[i.x][i.y] = Tile::Road
        }}
        Err(s) => {println!("{}",s)}
    }*/
    world

}


fn color_for_tile(tile: Tile) -> Rgb<u8> {
    // TODO Adjust colors
    match tile {
        Tile::DeepWater => Rgb([0,0,125]),
        Tile::Grass => Rgb([124,252,0]),
        Tile::Sand => Rgb([246,215,176]),
        Tile::Water => Rgb([35,137,218]),
        Tile::Mountain => Rgb([90, 75, 65]),
        Tile::Lava => Rgb([207, 16, 32]),
        //Tile::Tree => Rgb([66, 105, 47]),
        Tile::Road => Rgb([50,50,50]),
        Tile::Zone(0) => Rgb([0, 0, 0]),
        Tile::Zone(1) => Rgb([0, 125, 0]),
        Tile::Zone(2) => Rgb([125, 0, 0]),
        Tile::None => Rgb([255, 255, 255]),
        _ => Rgb([0,0,0]),
    }
}

pub fn export_to_image(map: &Vec<Vec<Tile>>, filename: &str) {
    // TODO implement UI using Bevy
    let width = map.len();
    let height = map[0].len();

    let mut image = RgbImage::new(width as u32, height as u32);

    for x in 0..width {
        for y in 0..height {
            let color = color_for_tile(map[x][y]);
            image.put_pixel(x as u32, y as u32, color);
        }
    }

    // Save the image to a file
    image.save(filename).expect("Failed to save image");
}
//for pathfinding
impl Walkable for Tile{
    fn cost(&self) -> u32 {
        match self{
            Tile::Grass => {1}
            Tile::Water => {10}
            Tile::DeepWater => {1000}
            Tile::Sand => {1}
            Tile::Mountain => {500}
            Tile::Lava => {100000}
            Tile::Road => {0}
            _ => {1}
        }
    }
}

struct Zone{
    tile:Tile,
    inner: Vec<Point>,
}
impl Zone{
    fn bfs(world: &mut Vec<Vec<Tile>>, id: usize, point: Point) -> Self{
        let mut vec = Vec::<Point>::new();
        let mut stack = Vec::<Point>::new();
        stack.push(point);
        while let Some(p) = stack.pop(){
            let (x,y) = p.as_tuple();
            match world[x][y] {
                Tile::Zone(i) =>{
                    if i == id {
                        vec.push(p);
                        stack.append(&mut p.neighbours(world.len(),world[0].len()));
                        world[x][y] = Tile::None;
                    }
                }
                _=>{}
            }
        }

        Zone {
            tile:Tile::None,
            inner: vec
        }
    }
    fn fill(&mut self, world: &mut Vec<Vec<Tile>>,tile:Tile){
        self.tile = tile;
        for i in self.inner.iter(){
            world[i.x][i.y] = tile;
        }
    }
}



fn displace<Source,T>(func: Source,x:f64,y:f64 )-> Displace<Source,Constant,Constant,Constant,Constant> where Source : NoiseFn<T, 2> {
    Displace::new( func, Constant::new(x),Constant::new(y), Constant::new(0.0), Constant::new(0.0))
}
fn scale<Source,T>(func: Source,x:f64,y:f64)-> ScalePoint<Source> where Source:NoiseFn<T, 2>{
    ScalePoint::new(func).set_x_scale(x).set_y_scale(y)
}