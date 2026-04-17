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
    executor.local_stats.clear();
    let seed_dir = dir.read_dir().expect("read_dir call failed");
    for entry in seed_dir {
        if let Ok(entry) = entry {
            if !running.load(Ordering::SeqCst) {
                break;
            }
            let path = &entry.path();
            if path.is_file() {
                info!("file {:?}", path.to_str());
                let file_len =
                    fs::metadata(path).expect("Could not fetch metadata.").len() as usize;
                if file_len < config::MAX_INPUT_LEN {
                    let buf = read_from_file(path);
                    executor.run_sync(&buf);
                } else {
                    warn!("Seed discarded, too long: {:?}", path);
                }
            }
        }
    }
    info!("sync {} file from seeds.", executor.local_stats.num_inputs);
    executor.update_log();
}