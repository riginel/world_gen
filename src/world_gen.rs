use noise::{Abs, Add, Constant, Displace, Fbm, Multiply, NoiseFn, OpenSimplex, Perlin, PerlinSurflet, ScaleBias, ScalePoint, Simplex, Value, Worley};
use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};



use std::cmp;
use std::time::Instant;
use noise::*;
use rand::prelude::*;
use image::{Rgb, RgbImage};
use rand::seq::SliceRandom;

use crate::pathfinding::a_star::*;
use crate::tile::*;
use crate::zones::zones::Zones;







pub fn gen_world(width: usize, height: usize) -> Vec<Vec<Tile>> {
    let seed = rand::random::<u32>() ;
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
    /*
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
    */
    /*
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
    */
    let zones = Zones::get_zones(&mut world);
    world

}







fn displace<Source,T>(func: Source,x:f64,y:f64 )-> Displace<Source,Constant,Constant,Constant,Constant> where Source : NoiseFn<T, 2> {
    Displace::new( func, Constant::new(x),Constant::new(y), Constant::new(0.0), Constant::new(0.0))
}
fn scale<Source,T>(func: Source,x:f64,y:f64)-> ScalePoint<Source> where Source:NoiseFn<T, 2>{
    ScalePoint::new(func).set_x_scale(x).set_y_scale(y)
}