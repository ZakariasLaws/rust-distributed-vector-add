extern crate mpi;

mod vector_extra;

use mpi::traits::*;
use vector_extra::Evec;
use mpi::topology::SystemCommunicator;

fn distributed_vector_add(world: SystemCommunicator, vec1: Evec<i32>, vec2: Evec<i32>) -> Evec<i32>{
    let rank = world.rank(); // Identifier for node running THIS instance
    let size = world.size();

    let mut result;

    {
        let smallest_len = if vec1.smaller(&vec2) {
            vec1.size() as i32
        } else {
            vec2.size() as i32
        };

        // Calculate interval for this node
        let start = (smallest_len * rank / size) as i32;
        let len = (smallest_len * (rank + 1) / size) as i32 - (smallest_len * rank / size) as i32;
        let end = start + len;

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
    let world = universe.world();
    let root = if universe.world().rank() == 0 { true } else { false };

    let mut vec1 = Evec::new();

    let mut vec2 = Evec::new();

    for x in 0..100 {
        vec1.vec.push(x);
        vec2.vec.push(x);
    }

    if root {
        println!("Running distributed vector add on {} nodes", world.size());
    }

    let result = distributed_vector_add(world, vec1, vec2);

    if root {
        println!("The first 10 values in the resulting array are:\n{:?}", &result.vec[0..10]);
    }
}
