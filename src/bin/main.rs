#[macro_use]
extern crate clap;
extern crate wasmprofiler;

use clap::{App, Arg};
use std::error::Error;
use std::path::Path;
use wasmprofiler::Profiler;

fn main() -> Result<(), Box<dyn Error>> {
    let cli_matches = App::new("wasm-profiler")
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::with_name("profile")
                .help("Profiling result in CSV format.")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("modules")
                .help("WebAssembly modules on which profiling was run.")
                .index(2)
                .multiple(true),
        )
        .get_matches();

    let profile_file = Path::new(cli_matches.value_of("profile").unwrap());
    let mut profile = Profiler::import_profile_from_file(profile_file)?;

    if let Some(module_names) = cli_matches.values_of("modules") {
        assert!(module_names.len() <= 2 ^ 32);
        for (index, module_name) in module_names.enumerate() {
            profile.load_module_from_file(index as u32, Path::new(module_name))?;
        }
    }

    profile.print();

    Ok(())
}
