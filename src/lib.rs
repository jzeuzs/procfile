//! # procfile
//!
//! A rust library for parsing Procfile(s).
//!
//! ## Examples
//!
//! ```rs
//! let my_procfile = "web: cargo run";
//! let parsed = procfile::parse(my_procfile).expect("Failed parsing procfile");
//! let web_process = parsed.get("web").expect("Failed getting web process");
//!
//! assert_eq!("cargo", web_process.command);
//! assert_eq!(vec!["run"], web_process.options);
//! ```

use cfg_if::cfg_if;
use dashmap::DashMap;
use lazy_static::lazy_static;
#[cfg(feature = "rayon")]
use rayon::prelude::*;
use regex::Regex;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Parses a Procfile string.
///
/// # Examples
///
/// ```rs
/// use procfile;
///
/// let my_procfile = "web: cargo run";
/// let parsed = procfile::parse(my_procfile).expect("Failed parsing procfile");
/// let web_process = parsed.get("web").expect("Failed getting web process");
///
/// assert_eq!("cargo", web_process.command);
/// assert_eq!(vec!["run"], web_process.options);
/// ```
///
/// # Errors
///
/// - When building the regex fails
/// - When either the command, options, and the process name don't exist but the regex matched
pub fn parse<'a>(content: &'a str) -> Result<DashMap<&'a str, Process>> {
    lazy_static! {
        static ref REGEX: Regex =
            Regex::new(r"^([A-Za-z0-9_]+):\s*(.+)$").expect("Failed building regex");
    }

    let map: DashMap<&'a str, Process> = DashMap::new();

    #[cfg(feature = "rayon")]
    content.split('\n').par_bridge().for_each(|line| match REGEX.captures(line) {
        Some(captures) => {
            let details = captures
                .get(2)
                .expect("Failed getting command and options")
                .as_str()
                .trim()
                .split(' ')
                .collect::<Vec<_>>();

            let name = captures.get(1).expect("Failed getting process name").as_str();

            map.insert(name, Process {
                command: details[0],
                options: details[1..].to_vec(),
            });
        },
        None => (),
    });

    #[cfg(not(feature = "rayon"))]
    content.split('\n').for_each(|line| match REGEX.captures(line) {
        Some(captures) => {
            let details = captures
                .get(2)
                .expect("Failed getting command and options")
                .as_str()
                .trim()
                .split(' ')
                .collect::<Vec<_>>();

            let name = captures.get(1).expect("Failed getting process name").as_str();

            map.insert(name, Process {
                command: details[0],
                options: details[1..].to_vec(),
            });
        },
        None => (),
    });

    Ok(map)
}

cfg_if! {
    if #[cfg(feature = "serde")] {
        use serde::{Serialize, Deserialize};

        /// Represents a single process.
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
        pub struct Process<'a> {
            /// The command to use. (e.g. `cargo`)
            pub command: &'a str,
            /// The command options. (e.g. `["build", "--release"]`)
            pub options: Vec<&'a str>,
        }
    } else {
        /// Represents a single process.
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
        pub struct Process<'a> {
            /// The command to use. (e.g. `cargo`)
            pub command: &'a str,
            /// The command options. (e.g. `["build", "--release"]`)
            pub options: Vec<&'a str>,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_process() {
        let procfile = "web: node a.js --option-1 --option-2";
        let parsed = parse(procfile).unwrap();

        assert!(parsed.contains_key("web"));

        let process = parsed.get("web").unwrap();

        assert_eq!("node", process.command);
        assert_eq!(vec!["a.js", "--option-1", "--option-2"], process.options)
    }

    #[test]
    fn test_multiple_process() {
        let procfile = "\
web: py b.py --my-option
worker: gcc c.c    
        ";

        let parsed = parse(procfile).unwrap();

        assert!(parsed.contains_key("web") && parsed.contains_key("worker"));

        let web = parsed.get("web").unwrap();
        let worker = parsed.get("worker").unwrap();

        assert_eq!("py", web.command);
        assert_eq!("gcc", worker.command);
        assert_eq!(vec!["b.py", "--my-option"], web.options);
        assert_eq!(vec!["c.c"], worker.options);
    }

    #[test]
    fn test_no_process() {
        let procfile = "";
        let parsed = parse(procfile).unwrap();

        assert!(parsed.is_empty());
    }

    #[test]
    fn test_invalid_process() {
        let procfile = "hedhehiidhodhidhiodiedhidwhio";
        let parsed = parse(procfile).unwrap();

        assert!(parsed.is_empty());
    }
}
