use tryamp::dcd::dcd_parser::DCDParser;
use tryamp::pdb::pdb_parser::PDBParser;
use std::fs::File;
use std::io::{Write, BufWriter};

pub fn run(traj: String, topo: String, nframe: usize, output: String) -> Result<(), String> {
	let trj_parser = DCDParser::new();
	let dcd = trj_parser.read(File::open(&traj).expect("Trajectory file not found"))?;
	let coordinates = &dcd.trajectory()[nframe];
	let mut top_parser = PDBParser::new();
	let system = top_parser.read_model(File::open(&topo).expect("Topology file not found"))?;
	let mut meta_info: Vec<String> = Vec::<String>::new();
	let mut atom_num: usize = 0;
	for model in system.models() {
		for chain in model.chains() {
			for residue in chain.residues() {
				for atom in residue.atoms() {
					let before: String = format!("ATOM  {0:>5}{1:>4} {2:>4} {3:>1}{4:>4}    ",
						atom.atom_number(),
						atom.atom_name_as_str(),
						residue.residue_name_as_str(),
						chain.chain_id_as_str(),
						residue.residue_id(),
					);
					meta_info.push(before);
					atom_num += 1;
				}
			}
			meta_info.push("TER".to_string());
		}
	}
	if coordinates.len() != atom_num {
		return Err(format!("The atom number in dcd is not consistent with that in pdb: {}!={}\n", coordinates.len(), meta_info.len()));
	}
	let mut writer = BufWriter::new(File::create(&output).expect(&format!("Failed to create {}", output)));
	let mut index: usize = 0;
	for line in meta_info.iter() {
		if line.starts_with("ATOM") {
			let coord = match coordinates.atom(index) {
				Some(val) => val,
				None => return Err(format!("The index {} is out of range.", index)),
			};
			let _ = writeln!(writer, "{0}{1:>8.3}{2:>8.3}{3:>8.3}", line, coord[0], coord[1], coord[2]);
			index += 1;
		}
		else if line.starts_with("TER") {
			let _ = writeln!(writer, "{}", line);
		}
	}
	let _ = writer.flush();
	Ok(())
}
