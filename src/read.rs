use crate::process::Process;
use std::collections::HashMap;
use std::fs::{self, DirEntry};

pub fn read_proc(entry: DirEntry) -> Option<Process> {
    let pid = entry.file_name().to_string_lossy().parse::<u32>().ok()?;
    Some(Process {
        pid,
        name: read_name(pid),
        ppid: read_status(pid),
        exe: read_exe(pid),
    })
}

pub fn get_proc(path: &str) -> std::io::Result<HashMap<u32, Process>> {
    let procs: HashMap<u32, Process> = fs::read_dir(path)?
        .flatten()
        .filter_map(|entry| read_proc(entry))
        .map(|p| (p.pid, p))
        .collect();
    Ok(procs)
}

pub fn read_name(pid: u32) -> String {
    fs::read(format!("/proc/{pid}/comm"))
        .map(|r| String::from_utf8_lossy(&r).trim().to_string()).unwrap_or_else(|_| "<kernel>".into())
}

pub fn read_exe(pid: u32) -> String {
    fs::read_link(format!("/proc/{}/exe", pid))
        .map(|p| p.display().to_string()).unwrap_or_else(|_| "<no access>".into())
}

pub fn read_status(pid: u32) -> u32 {
    let content = fs::read(format!("/proc/{}/status", pid))
        .ok()
        .map(|r| String::from_utf8_lossy(&r).to_string())
        .unwrap_or_default();

    let mut ppid = 0;

    for line in content.lines() {
        if let Some(v) = line.strip_prefix("PPid:") {
            ppid = v.trim().parse().unwrap_or(0);
        }
    }
    return ppid;
}

pub fn build_tree(procs: &HashMap<u32, Process>) -> HashMap<u32, Vec<u32>> {
    let mut tree: HashMap<u32, Vec<u32>> = HashMap::new();
    for p in procs.values() {
        tree.entry(p.ppid).or_default().push(p.pid);
    }
    tree
}
