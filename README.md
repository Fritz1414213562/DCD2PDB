#DCD2PDB
Extract a snapshot in a .dcd trajectory file as a .pdb structure file.

## Installation
### Prerequisite
This software is written in Rust and the installation depends on cargo.
You must set up the Rust environment (https://www.rust-lang.org/tools/install).
### Procedure
```:sh
git clone https://github.com/Fritz1414213562/DCD2PDB.git
cargo build --release
```

## Usage
/path-to-target/release/dcd2pdb --traj <trajectory-file (.dcd)> --topo <topology-file (.pdb)> --output <output-name (.pdb)> --frame <frame-number (Integer)>
