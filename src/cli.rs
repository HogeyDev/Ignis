use std::{collections::{HashMap, HashSet}, process};

#[derive(Debug)]
pub struct CliParser {
    pub args: Vec<String>,
    pub flags: HashSet<String>,
    pub options: HashMap<String, String>,
    pub arguments: Vec<String>,
}

impl CliParser {
    pub fn from(args: Vec<String>) -> CliParser {
        let mut flags = HashSet::new();
        let mut options = HashMap::new();
        let mut arguments = Vec::new();

        let mut iter = args.iter().skip(1);
        while let Some(arg) = iter.next() {
            if arg.chars().nth(0).unwrap() == '-' {
                if arg.chars().nth(1).unwrap() == '-' {
                    // this is a flag and becomes true when mentioned
                    let flag = arg[2..].to_string();
                    flags.insert(flag);
                } else {
                    // this is an option and will have a value specified after
                    let option = arg[1..].to_string();
                    if let Some(value) = iter.next() {
                        options.insert(option, value.to_string());
                    } else {
                        eprintln!("Expected argument after `{option}`");
                        process::exit(1);
                    }
                }
            } else {
                // this is an input file, which is the only argument
                arguments.push(arg.to_string());
            }
        }

        CliParser {
            args,
            flags,
            options,
            arguments,
        }
    }

    pub fn flag_value(&self, flag: &str) -> bool {
        return self.flags.contains(&flag.to_string());
    }
    pub fn option_value(&self, flag: &str, default_value: &str) -> String {
        return self.options.get(&flag.to_string()).unwrap_or(&default_value.to_string()).to_string();
    }
}
