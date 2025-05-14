use clap::{Parser};
use dcd2pdb::dcd2pdb;
use std::time::Instant;

#[derive(Parser)]
#[clap(
	name = "dcd2pdb",
	author = "Fritz N. <fritz1414213562@gmail.com>",
	version = "1.0",
	about = "Extract a single frame in .dcd trajectory file as pdb file"
)]
struct AppArg {
	#[clap(short = 't', long = "traj")]
	traj: String,
	#[clap(short = 'p', long = "topo")]
	topo: String,
	#[clap(short = 'o', long = "output")]
	output: String,
	#[clap(short = 'n', long = "frame")]
	nframe: usize,
}

fn main() {
	let measure_start  = Instant::now();
	let arg:    AppArg = AppArg::parse();
	let traj:   String = arg.traj;
	let topo:   String = arg.topo;
	let output: String = arg.output;
	let nframe:  usize = arg.nframe;

	match dcd2pdb::run(traj, topo, nframe, output) {
		Ok(()) => (),
		Err(err) => {
			eprintln!("{}", err);
			std::process::exit(1);
		}
	}

	let elapsed = measure_start.elapsed();
	println!("Elapsed Time: {}.{:03} sec", elapsed.as_secs(), elapsed.subsec_nanos() / 1_000_000);
}
