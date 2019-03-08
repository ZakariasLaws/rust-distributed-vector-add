# rust-distributed-vector-add

Simple, straight forward distributed vector-add written in Rust, made as a practice assignment to learn Rust. It generates two arrays of the length specified by user, with incrementing values.

Uses the [MPI-Crate](https://crates.io/crates/mpi) for communication.

## Run on DAS-5 with slurm

Create a slurm script similar to this one:
```bash
$ cat mpi_run
#!/bin/bash
#SBATCH --time=00:15:00
#SBATCH -N 3
#SBATCH --ntasks-per-node=1

. /etc/bashrc
. /etc/profile.d/modules.sh
module load openmpi/gcc/64

APP=$1
ARGS="$2"
OMPI_OPTS="--mca btl ^usnic"

$MPI_RUN $OMPI_OPTS $APP $ARGS
```

Compile with `cargo build` and execute with slurm with `sbatch mpi_run vector_add <array_length>`
