extern crate mpi;

use mpi::traits::*;
use mpi::request::WaitGuard;

fn main() {
    let universe = mpi::initialize().unwrap();
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
