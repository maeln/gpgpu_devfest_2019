extern crate wyzoid;

use std::path::PathBuf;
use std::time::Instant;

fn double(x: f32) -> f32 {
    x * 2.0
}

const VALUES: usize = 1024 * 1024;

fn main() {
    let input = wyzoid::utils::rand_vec::<f32>(VALUES, 0.0, 1.0);

    let shader = PathBuf::from("double.cs.spirv");

    let job = wyzoid::high::job::JobBuilder::new()
        .add_buffer(&input, 0, 0)
        .add_shader(&shader)
        .add_dispatch((VALUES as u32 / 32, 1, 1))
        .build();
    let (res, timing) = job.execute();
    println!("gpu exe: {}", timing);

    let s = Instant::now();
    let mut test: Vec<f32> = Vec::with_capacity(input.len());
    for v in &input {
        test.push(double(v.clone()));
    }

    println!("cpu exe: {}ms", wyzoid::utils::get_fract_s(s.elapsed()));
    for i in 0..input.len() {
        if res[0][i] != test[i] {
            println!("ERROR");
        }
    }
}
