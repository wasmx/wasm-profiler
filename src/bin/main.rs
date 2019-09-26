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
        // TODO: make this support multiple inputs
        .arg(
            Arg::with_name("module")
                .help("WebAssembly module on which profiling was run.")
                .index(2),
        )
        .get_matches();

    let profile_file = Path::new(cli_matches.value_of("profile").unwrap());
    let mut profile = Profiler::import_profile_from_file(profile_file)?;

    if let Some(module_name) = cli_matches.value_of("module") {
        profile.load_module_from_file(Path::new(module_name))?;
    }

    profile.print();

    Ok(())
}
