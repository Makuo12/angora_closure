use super::*;
use crate::executor::Executor;
use angora_common::{config, defs};
use std::{
    collections::HashMap,
    fs,
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

pub fn sync_depot(executor: &mut Executor, running: Arc<AtomicBool>, dir: &Path) {
    println!("[sync_depot] start, dir={:?}", dir);
    executor.local_stats.clear();
    let seed_dir = dir.read_dir().expect("read_dir call failed");
    for entry in seed_dir {
        if let Ok(entry) = entry {
            if !running.load(Ordering::SeqCst) {
                println!("[sync_depot] running flag is false, stopping early");
                break;
            }
            let path = &entry.path();
            println!("[sync_depot] processing entry={:?}", path);
            if path.is_file() {
                let file_len =
                    fs::metadata(path).expect("Could not fetch metadata.").len() as usize;
                println!("[sync_depot] file_len={}", file_len);
                if file_len < config::MAX_INPUT_LEN {
                    let buf = read_from_file(path);
                    println!("[sync_depot] running sync on {:?}, buf_len={}", path, buf.len());
                    executor.run_sync(&buf);
                } else {
                    println!("[sync_depot] discarding seed, too long: {:?}, len={}", path, file_len);
                    warn!("Seed discarded, too long: {:?}", path);
                }
            } else {
                println!("[sync_depot] skipping non-file entry: {:?}", path);
            }
        } else {
            println!("[sync_depot] failed to read entry: {:?}", entry);
        }
    }
    println!("sync {} file from seeds.", executor.local_stats.num_inputs);
    println!("[sync_depot] done, calling update_log");
    executor.update_log();
}

