extern crate mpi;

mod vector_extra;

use mpi::traits::*;
use mpi::environment::Universe;
use vector_extra::Evec;

fn test_mpi(universe: Universe){
    let world = universe.world();
    let size = world.size();
    let rank = world.rank();

    if size != 6 {
        panic!("Size is {} must be 6", size);
    }

    match rank {
        0 => {
            for x in 1..6 {
                let msg=[x as f64];
                world.process_at_rank(x).send(&msg[..]);
            }

        }
        1..=6 => {
            let (msg, status) = world.any_process().receive_vec::<f64>();
            println!("Process {} got message {:?}.\nStatus is: {:?}", rank, msg, status);

        }
        _ => unreachable!()
    }
}

fn distributed_vector_add(universe: Universe, vec1: Evec<i32>, vec2: Evec<i32>) -> Evec<i32>{
    let world = universe.world();
    let rank = world.rank(); // Identifier for node running THIS instance
    let size = world.size();

    println!("Running distributed vector add on {} nodes", size);

    let mut result;

    {
        let smallest_len = if vec1.smaller(&vec2) {
            vec1.size() as i32
        } else {
            vec2.size() as i32
        };

        // Calculate interval for this node
        let start = smallest_len * rank / size;
        let end = (smallest_len * (rank + 1) / size) - (smallest_len * rank / size);

        result = Evec::new();
        for _ in 0..smallest_len{
            result.vec.push(0);
        }

        // Perform vector addition on interval partition
        for i in 0..smallest_len {
            if i >= start && i < end {
                result.vec[i as usize] = vec1.vec[i as usize] + vec2.vec[i as usize];
            }
        }

        world.process_at_rank(rank).broadcast_into(
            &mut result.vec[start as usize..end as usize]);
    }

    result
}

fn main() {
    let universe = mpi::initialize().unwrap();
    let root = if universe.world().rank() == 0 { true } else { false };

    let mut vec1 = Evec::new();

    let mut vec2 = Evec::new();

    for x in 0..100 {
        vec1.vec.push(x);
        vec2.vec.push(x);
    }

    let result = distributed_vector_add(universe, vec1, vec2);

    if root {
        println!("The first 10 values in the resulting array are:\n{:?}", &result.vec[0..10]);
    }
}
