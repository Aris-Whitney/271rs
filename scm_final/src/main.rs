use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

const SCM_NAME: &str = ".scm";

#[derive(Serialize, Deserialize)]
struct ScmFile {
    latest: HashMap<String, Vec<String>>,
    commit: Vec<CommitEntry>,
}

#[derive(Serialize, Deserialize)]
struct CommitEntry {
    init: HashMap<String, Vec<String>>,
    diff: HashMap<String, Vec<String>>,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: scm [commit|revert|viewer]");
        return;
    }

    match args[1].as_str() {
        "commit" => commit(),
        "revert" => revert(),
        "viewer" => viewer(),
        _ => println!("Unknown command"),
    }
}

// ==============================
// FILE SYSTEM WALKER
// ==============================

fn get_all_files() -> Vec<String> {
    fn walk(dir: &Path, files: &mut Vec<String>) {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let name = path.file_name().unwrap().to_str().unwrap();

            if name.starts_with('.') {
                continue;
            }

            if path.is_dir() {
                walk(&path, files);
            } else {
                files.push(path.to_string_lossy().to_string());
            }
        }
    }

    let mut files = Vec::new();
    walk(Path::new("."), &mut files);
    files
}

// ==============================
// DIFF ENGINE (PURE RUST)
// ==============================

fn diff(old: &[String], new: &[String]) -> Vec<String> {
    let mut out = vec!["---".to_string(), "+++".to_string()];

    for i in 0..old.len().max(new.len()) {
        let o = old.get(i);
        let n = new.get(i);

        if o != n {
            if let Some(v) = o {
                out.push(format!("-{}", v));
            }
            if let Some(v) = n {
                out.push(format!("+{}", v));
            }
        }
    }

    out
}

// ==============================
// COMMIT
// ==============================

fn commit() {
    let fs_list = get_all_files();

    // INIT CASE
    if !Path::new(SCM_NAME).exists() || fs::read_to_string(SCM_NAME).unwrap().trim().is_empty() {
        let mut latest = HashMap::new();

        for f in &fs_list {
            let content = fs::read_to_string(f).unwrap();
            let lines = content.lines().map(|s| s.to_string()).collect();
            latest.insert(f.clone(), lines);
        }

        let scm = ScmFile {
            latest: latest.clone(),
            commit: vec![CommitEntry {
                init: latest,
                diff: HashMap::new(),
            }],
        };

        fs::write(SCM_NAME, serde_json::to_string(&scm).unwrap()).unwrap();
        return;
    }

    // NORMAL COMMIT
    let mut scm: ScmFile =
        serde_json::from_str(&fs::read_to_string(SCM_NAME).unwrap()).unwrap();

    let mut init = HashMap::new();
    let mut diff_map = HashMap::new();

    for f in &fs_list {
        let content = fs::read_to_string(f).unwrap();
        let new_lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

        if let Some(old_lines) = scm.latest.get(f) {
            let d = diff(old_lines, &new_lines);
            diff_map.insert(f.clone(), d);
        } else {
            init.insert(f.clone(), new_lines);
        }
    }

    let mut new_latest = HashMap::new();
    for f in &fs_list {
        let content = fs::read_to_string(f).unwrap();
        new_latest.insert(
            f.clone(),
            content.lines().map(|s| s.to_string()).collect(),
        );
    }

    scm.latest = new_latest;
    scm.commit.push(CommitEntry { init, diff: diff_map });

    fs::write(SCM_NAME, serde_json::to_string(&scm).unwrap()).unwrap();
}

// ==============================
// REVERT (FIXES PROFESSOR'S BUG)
// ==============================

fn revert() {
    if !Path::new(SCM_NAME).exists() {
        return;
    }

    let mut scm: ScmFile =
        serde_json::from_str(&fs::read_to_string(SCM_NAME).unwrap()).unwrap();

    if scm.commit.len() < 2 {
        return;
    }

    // SCRAPE: restore latest snapshot to disk
    for (file, lines) in &scm.latest {
        fs::write(file, lines.join("\n")).unwrap();
    }

    // POP LAST COMMIT
    let last = scm.commit.pop().unwrap();

    // APPLY REVERSE DIFF
    for (file, patch) in last.diff {
        let current = fs::read_to_string(&file).unwrap();
        let mut data: Vec<String> = current.lines().map(|s| s.to_string()).collect();

        for p in patch {
            if p.starts_with("-") {
                // reverse of "-": add the old line back
                data.push(p[1..].to_string());
            } else if p.starts_with("+") {
                // reverse of "+": remove that line
                data.retain(|x| x != &p[1..]);
            }
        }

        fs::write(file, data.join("\n")).unwrap();
    }

    // REWRITE LATEST AFTER REVERT (FIX BUG)
    let mut new_latest = HashMap::new();
    for f in get_all_files() {
        let content = fs::read_to_string(&f).unwrap();
        new_latest.insert(
            f.clone(),
            content.lines().map(|s| s.to_string()).collect(),
        );
    }
    scm.latest = new_latest;

    fs::write(SCM_NAME, serde_json::to_string(&scm).unwrap()).unwrap();
}

// ==============================
// VIEWER
// ==============================

fn viewer() {
    if !Path::new(SCM_NAME).exists() {
        println!("No SCM file found.");
        return;
    }

    let contents = fs::read_to_string(SCM_NAME).unwrap();
    println!("{}", contents);
}
--