use std::path::{Path, PathBuf};
use std::collections::BTreeMap;

type PrefixMap = BTreeMap<PathBuf, Vec<PathBuf>>;


pub fn collapse(paths: Vec<PathBuf>, pattern: &str, wildcard: bool) -> Vec<String> {
    let map = into_map(paths, pattern);
    let lines = collapse_map(map, wildcard);
    lines
}


fn collapse_map(map: PrefixMap, wildcard: bool) -> Vec<String> {
    map.iter()
        .map(|(key, val)| entry_to_string(key, val, wildcard))
        .collect()
}

fn entry_to_string(prefix: &PathBuf, paths: &Vec<PathBuf>, wildcard: bool) -> String {
    fn path_to_string(path: &PathBuf) -> String {
        path.to_str()
            .expect("failed to convert path")
            .to_owned()
    }

    let wildcard = if wildcard {"*"} else {""};

    match paths.len() {
        0 => panic!("empty map entry"),
        1 => path_to_string(&paths[0]),
        _ => {
            let mut with_wildcard = prefix.clone();
            with_wildcard.push(wildcard);
            path_to_string(&with_wildcard)
        },
    }
}

fn into_map(paths: Vec<PathBuf>, pattern: &str) -> PrefixMap {
    let mut map: PrefixMap = BTreeMap::new();
    for p in paths {
        let key = make_prefix(p.as_path(), pattern);

        if let Some(entry) = map.get_mut(&key) {
            // this prefix entry already exists, add to it
            if entry.len() < 2 {
                entry.push(p);
            }
            continue  // to appease the borrow checker
        }
        // else, create new entry
        map.insert(key, vec![p]);
    }
    map
}

fn make_prefix(path: &Path, pattern: &str) -> PathBuf {
    let components: Vec<&str> = path.iter()
        .map(|c| c.to_str().expect("failed to get path component"))
        .collect();

    let index = components.iter()
        .rposition(|s| s.contains(pattern))
        .expect(&format!("`locate` result did not contain pattern: {}", pattern));

    let mut prefix_path = PathBuf::new();
    for i in 0..index+1 {
        prefix_path.push(path.iter()
                         .nth(i)
                         .expect("weird path indexing error!"));
    }
    prefix_path
}
