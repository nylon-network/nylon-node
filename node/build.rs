/// The cargo build starts here
/// It pulls 2 utility scripts in
use substrate_build_script_utils::{generate_cargo_keys, rerun_if_git_head_changed};

fn main() {

	// generate git keys
	generate_cargo_keys();

	// rerun the build when the HEAD has changed
	rerun_if_git_head_changed();
}
