extern crate mpi;

mod vector_extra;

use mpi::traits::*;
use mpi::Count;
use vector_extra::Evec;
use mpi::topology::SystemCommunicator;
use mpi::datatype::PartitionMut;
use std::env;
use std::process::exit;

/// Perform vector in a distributed setting, using MPI capabilities.
///
/// Will only use the number of elements equal to the length of the smallest array, excess
/// elements will be dropped.
fn distributed_vector_add(world: SystemCommunicator, vec1: Evec<i32>, vec2: Evec<i32>) -> Evec<i32>{
    let rank = world.rank(); // Identifier for node running THIS instance
    let size = world.size();

    let smallest_len = if vec1.smaller(&vec2) {
        vec1.size() as i32
    } else {
        vec2.size() as i32
    };

    // Calculate interval for this node
    let start = (smallest_len * rank / size) as i32;
    let len = (smallest_len * (rank + 1) / size) as i32 - (smallest_len * rank / size) as i32;

    let msg: Vec<_> = (0..len)
        .map(|x| vec1.vec[(x+start) as usize] + vec2.vec[(x+start) as usize])
        .collect();

    // Check if the array is dividable with the number of processes
    let counts: Vec<Count> = match smallest_len % size == 0{
        true => {
            vec![len; size as usize]
        }
        false => {
            let mut tmp_v = Vec::new();
            for i in 0..size {
                let tmp_l = (smallest_len * (i + 1) / size) as i32 - (smallest_len * i / size) as i32;
                tmp_v.push(tmp_l);
            }
            tmp_v
        }
    };

    let displs: Vec<Count> = counts
        .iter()
        .scan(0, |acc, &x| {
            let tmp = *acc;
            *acc += x;
            Some(tmp)
        })
        .collect();

    let mut buf = vec![0; smallest_len as usize];

    {
        let mut partition = PartitionMut::new(&mut buf[..], counts, &displs[..]);
        world.all_gather_varcount_into(&msg[..], &mut partition);
    }

    Evec { vec: buf}
}

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let root = if universe.world().rank() == 0 { true } else { false };

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        if root {
            println!("Please provide an array length:\n\
            Usage: mpirun ARGS vector_add <array_length>\n---------------------\n");
        }
        exit(1);
    }

    let array_length = args[1].parse().unwrap();

    // Create two vectors and fill them with incrementing values from 0..<user_input>
    let mut vec1 = Evec::new();
    let mut vec2 = Evec::new();

    for x in 0..array_length {
        vec1.vec.push(x);
        vec2.vec.push(x);
    }

    if root {
        println!("Running distributed vector add on {} nodes.",
                 world.size());
    }

    let result = distributed_vector_add(world, vec1, vec2);

    if root {
        let length = if array_length < 30 {array_length} else {30};
        println!("The first 30 elements in the resulting array are:\n{:?}", &result.vec[0..length as usize]);
    }
}
