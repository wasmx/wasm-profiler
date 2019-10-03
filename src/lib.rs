use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::path::Path;
use std::time::Duration;

type FunctionIndex = u32;

#[derive(Debug)]
pub struct ProfileEntry {
    func_index: u32,
    duration: Duration,
}

pub type ProfileEntries = Vec<ProfileEntry>;

#[derive(Deserialize, Debug)]
struct CSVLine {
    func_index: u32,
    duration: u64,
}

#[derive(Debug)]
pub struct Profiler {
    profile: HashMap<FunctionIndex, Duration>,
    names: HashMap<FunctionIndex, String>,
}

impl Profiler {
    pub fn import_profile(entries: ProfileEntries) -> Profiler {
        let mut ret = Profiler {
            profile: HashMap::new(),
            names: HashMap::new(),
        };

        for entry in entries {
            *ret.profile
                .entry(entry.func_index)
                .or_insert(Duration::new(0, 0)) += entry.duration;
        }

        ret
    }

    pub fn import_profile_from_file(path: &Path) -> Result<Profiler, Box<dyn Error>> {
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

    pub fn load_module(
        &mut self,
        module: parity_wasm::elements::Module,
    ) -> Result<(), parity_wasm::elements::Error> {
        let module = module.parse_names()?;

        if let Some(names_section) = module.names_section() {
            if let Some(function_names) = names_section.functions() {
                for (index, name) in function_names.names().iter() {
                    self.names.insert(index, name.to_string());
                }
            }
        }

        Ok(())
    }

    pub fn load_module_from_bytes<T: AsRef<[u8]>>(
        &mut self,
        module: T,
    ) -> Result<(), parity_wasm::elements::Error> {
        let module = parity_wasm::elements::Module::from_bytes(&module)?;
        self.load_module(module)
    }

    pub fn load_module_from_file(
        &mut self,
        path: &Path,
    ) -> Result<(), parity_wasm::elements::Error> {
        let module = parity_wasm::elements::deserialize_file(&path)?;
        self.load_module(module)
    }

    pub fn print(&self) {
        println!("{}", self)
    }
}

impl fmt::Display for Profiler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let total_time = self
            .profile
            .iter()
            .fold(Duration::new(0, 0), |acc, e: (_, &Duration)| acc + *e.1);
        writeln!(f, "Total time taken {}us", total_time.as_micros())?;

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

            writeln!(
                f,
                "Function {} took {}us ({:.2}%)",
                name,
                val.as_micros(),
                // TODO: use as_nanos() for better precision?
                val.as_micros() * 100 / total_time.as_micros()
            )?
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn import_from_vec() {
        let entries: ProfileEntries = vec![
            ProfileEntry {
                func_index: 0,
                duration: Duration::from_micros(1234),
            },
            ProfileEntry {
                func_index: 1,
                duration: Duration::from_micros(555),
            },
        ];
        let profile = Profiler::import_profile(entries);
        assert_eq!(profile.profile.len(), 2);
        assert_eq!(format!("{}", profile), "Total time taken 1789us\nFunction <index:0> took 1234us (68%)\nFunction <index:1> took 555us (31%)\n");
    }

    #[test]
    fn import_wasm() {
        let mut profile = Profiler::import_profile(vec![]);
        let module = parity_wasm::elements::Module::default();
        let module = module.to_bytes().expect("wasm serialization to work");
        profile
            .load_module_from_bytes(&module)
            .expect("wasm loading to work");
        assert_eq!(profile.profile.len(), 0);
        assert_eq!(format!("{}", profile), "Total time taken 0us\n");
    }
}
