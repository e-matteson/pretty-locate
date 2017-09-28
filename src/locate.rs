use std::io::prelude::*;
use std::fs::File;
// TODO faster non-crypto hash
use std::collections::BTreeMap;
use std::collections::btree_map::Entry::{Occupied, Vacant};

use grep::GrepBuilder;
use regex::bytes::Regex;

type PathString = String;


pub struct Locations(BTreeMap<PathMatch, HasPrefix>);

// TODO only hash path string?
#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct PathMatch {
    path: PathString,
    matches: Vec<Region>,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Region {
    start: usize,
    end: usize,
}

#[derive(Debug)]
pub enum HasPrefix {
    One(PathMatch),
    Many,
}

#[allow(dead_code)]
enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

////////////////////////////////////////////////////////////////////////////////

impl Color {
    fn code(&self) -> u8 {
        match *self {
            Color::Black => 30,
            Color::Red => 31,
            Color::Green => 32,
            Color::Yellow => 33,
            Color::Blue => 34,
            Color::Magenta => 35,
            Color::Cyan => 36,
            Color::White => 37,
        }
    }
}

impl Locations {
    pub fn new(pattern: &str, database_path: &str) -> Locations {
        Locations::to_map(Locations::find_paths(pattern, database_path))
    }

    fn find_paths(pattern: &str, database_path: &str) -> Vec<PathMatch> {
        // TODO no string for database_path
        if pattern.is_empty() {
            // TODO return Result instead
            Vec::new()
        } else {
            let grep = GrepBuilder::new(pattern).build().expect("grepbuilder");
            let database_string = read_file(database_path);
            let database = database_string.as_bytes();
            let regex = grep.regex();

            let mut paths = Vec::new();
            for m in grep.iter(database) {
                let bytes = database[m.start()..m.end()].to_owned();
                paths.push(PathMatch::from_bytes(bytes, regex));
            }
            paths
        }
    }

    fn to_map(paths: Vec<PathMatch>) -> Locations {
        let mut map = BTreeMap::new();
        for p in paths {
            match map.entry(p.prefix()) {
                Vacant(entry) => {
                    entry.insert(HasPrefix::One(p));
                }
                Occupied(entry) => entry.into_mut().into_many(),
            }
        }
        Locations(map)
    }


    fn into_lines(self) -> Vec<String> {
        self.0
            .into_iter()
            .map(|(k, v)| entry_to_string(k, v))
            .collect()
    }

    pub fn into_string(self) -> String {
        self.into_lines().join("\n")
    }
}


impl PathMatch {
    fn from_bytes(bytes: Vec<u8>, regex: &Regex) -> PathMatch {
        PathMatch {
            matches: regex
                .find_iter(&bytes)
                .map(|m| Region::new(m.start(), m.end()))
                .collect(),
            path: String::from_utf8(bytes)
                .expect("utf8")
            // TODO remove trailing newlines earlier? in grep crate?
                .trim_right()
                .to_owned(),
        }
    }

    pub fn prefix(&self) -> PathMatch {
        const PATH_SEP: char = '/';
        let last_match_end =
            self.matches.iter().last().expect("no matches").end;

        let prefix_end = if last_match_end == self.path.len() {
            last_match_end - 1
        } else {
            let slice_past_match =
                self.path.get(last_match_end..).expect("slice str");

            // TODO off-by-1?
            let prefix_end_offset = slice_past_match
                .find(PATH_SEP)
                .unwrap_or_else(|| slice_past_match.len() - 1);
            last_match_end + prefix_end_offset
        };

        // println!("{:?}", self);
        // println!("{:?}", last_match_end);
        PathMatch {
            path: self.path.get(...prefix_end).expect("get prefix").to_owned(),
            matches: self.matches.clone(),
        }
        // panic!("done");
    }

    pub fn format(&self) -> String {
        self.format_helper(false)
    }

    pub fn format_wildcard(&self) -> String {
        self.format_helper(true)
    }

    fn format_helper(&self, has_wildcard: bool) -> String {
        let start_format_str = start_format(Color::Red, true);
        let end_format_str = end_format();

        let mut s = String::new(); // TODO with_capacity?
        let mut start = 0;
        for region in &self.matches {
            s.push_str(self.path.get(start..region.start).unwrap());
            s.push_str(&start_format_str);
            s.push_str(self.path.get(region.start..region.end).unwrap());
            s.push_str(&end_format_str);
            start = region.end; // TODO +1?
        }
        s.push_str(self.path.get(start..).unwrap());
        if has_wildcard {
            // TODO should * be highlighted?
            s.push_str("*");
        }
        s
    }
}

impl Region {
    fn new(start: usize, end: usize) -> Region {
        Region {
            start: start,
            end: end,
        }
    }
}

impl HasPrefix {
    fn into_many(&mut self) {
        *self = HasPrefix::Many;
    }
}


fn start_format(color: Color, is_bold: bool) -> String {
    let mut s = String::with_capacity(12);
    s.push_str("\x1b[");
    if is_bold {
        s.push_str("1;");
    }
    s.push_str(&color.code().to_string());
    s.push_str("m");
    s
}

fn end_format() -> String {
    "\x1b[0m".to_owned()
}

fn entry_to_string(prefix: PathMatch, value: HasPrefix) -> String {
    // TODO use regions for coloring
    // const WILDCARD: &str = "*";
    // TODO fastest way of appending wildcard?
    match value {
        HasPrefix::Many => prefix.format_wildcard(),
        HasPrefix::One(full_path) => full_path.format(),
    }
}


fn read_file(path: &str) -> String {
    let mut f: File = File::open(path).expect("open");
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).expect("read_to_string");
    buffer
}
