extern crate wyzoid;

use std::path::PathBuf;
use std::time::Instant;

const NUM_DATA: usize = 1024 * 1024;

fn main() {
    let input = wyzoid::utils::rand_vec::<f32>(NUM_DATA, 0.0, 1.0);
    let shader = PathBuf::from("taylor.cs.spirv");

    let job = wyzoid::high::job::JobBuilder::new()
        .add_buffer(&input, 0, 0)
        .add_shader(&shader)
        .add_dispatch((NUM_DATA as u32 / 64, 1, 1))
        .build();
    let (res, timing) = job.execute();

    println!("gpu: {}", timing);

    let s = Instant::now();
    let mut test: Vec<f32> = Vec::with_capacity(input.len());
    for v in &input {
        test.push(f32::sin(v.clone()));
    }

    println!("cpu exe: {}ms", wyzoid::utils::get_fract_s(s.elapsed()));

    for i in 0..NUM_DATA {
        if !wyzoid::utils::f32_cmp(res[0][i], test[i], 0.001) {
            println!("ERR {}, {}, {}", input[i], res[0][i], f32::sin(input[i]));
            return;
        }
    }
}
