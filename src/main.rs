#[macro_use]
extern crate clap;

use clap::{App, Arg};
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use std::time::Duration;

type FunctionIndex = u32;

#[derive(Deserialize, Debug)]
struct CSVLine {
    func_index: u32,
    duration: u64,
}

#[derive(Debug)]
struct Profiler {
    profile: HashMap<FunctionIndex, Duration>,
    names: HashMap<FunctionIndex, String>,
}

impl Profiler {
    fn import_profile_from_file(path: &Path) -> Result<Profiler, Box<Error>> {
        let mut reader = csv::Reader::from_path(&path)?;

        let mut ret = Profiler {
            profile: HashMap::new(),
            names: HashMap::new(),
        };

        for line in reader.deserialize() {
            let line: CSVLine = line?;
            // Translate CSV input lines into a hashmap. Sum up durations for the same functions.
            *ret.profile
                .entry(line.func_index)
                .or_insert(Duration::new(0, 0)) += Duration::from_micros(line.duration);
        }

        Ok(ret)
    }

    fn load_module_from_file(&mut self, path: &Path) -> Result<(), Box<Error>> {
        let module = parity_wasm::elements::deserialize_file(&path)?;
        // FIXME: implement error handling
        let module = module.parse_names().expect("Failed to parse NamesSection");

        if let Some(names_section) = module.names_section() {
            if let Some(function_names) = names_section.functions() {
                for (index, name) in function_names.names().iter() {
                    self.names.insert(index, name.to_string());
                }
            }
        }

        Ok(())
    }

    fn print(&self) {
        let total_time = self
            .profile
            .iter()
            .fold(Duration::new(0, 0), |acc, e: (_, &Duration)| acc + *e.1);
        println!("Total time taken {}us", total_time.as_micros());

        // Sort results by value.
        use std::iter::FromIterator;
        let mut profile = Vec::from_iter(self.profile.clone());
        profile.sort_by(|&(_, a), &(_, b)| b.cmp(&a));

        for (key, val) in profile.iter() {
            let name = if self.names.contains_key(key) {
                self.names[key].to_string()
            } else {
                format!("<index:{}>", key)
            };

            println!(
                "Function {} took {}us ({:.2}%)",
                name,
                val.as_micros(),
                // TODO: use as_nanos() for better precision?
                val.as_micros() * 100 / total_time.as_micros()
            );
        }
    }
}

fn main() {
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
    let mut profile =
        Profiler::import_profile_from_file(profile_file).expect("failed to parse/load profile");

    if let Some(module_name) = cli_matches.value_of("module") {
        profile
            .load_module_from_file(Path::new(module_name))
            .expect("failed to load wasm");
    }

    profile.print();
}
