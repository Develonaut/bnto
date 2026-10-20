// Benchmarks -- file-collect walking a big generated folder tree.

use criterion::{Criterion, criterion_group, criterion_main};

use bnto_core::NoopContext;
use bnto_core::processor::{FileData, NodeInput, NodeProcessor};
use bnto_core::progress::ProgressReporter;
use bnto_file::FileCollect;
use std::path::Path;

/// 100 folders × 200 files: big enough that per-file costs dominate.
const FOLDERS: usize = 100;
const FILES_PER_FOLDER: usize = 200;

fn build_tree(root: &Path) {
    for d in 0..FOLDERS {
        let dir = root.join(format!("pkg-{d}")).join("lib");
        std::fs::create_dir_all(&dir).unwrap();
        for f in 0..FILES_PER_FOLDER {
            let ext = if f % 4 == 0 { "md" } else { "js" };
            std::fs::write(dir.join(format!("file-{f}.{ext}")), b"").unwrap();
        }
    }
}

fn collect(root: &Path, pattern: &str) -> usize {
    let mut params = serde_json::Map::new();
    params.insert("pattern".into(), pattern.into());
    let input = NodeInput {
        data: FileData::Path(root.to_path_buf()),
        filename: "tree".into(),
        mime_type: None,
        params,
    };
    let progress = ProgressReporter::new_noop();
    let output = FileCollect::new()
        .process(input, &progress, &NoopContext)
        .unwrap();
    output.files.len()
}

fn bench_collect(c: &mut Criterion) {
    let tmp = tempfile::tempdir().unwrap();
    build_tree(tmp.path());
    let total = FOLDERS * FILES_PER_FOLDER;

    let mut group = c.benchmark_group("file_collect_20k");
    group.sample_size(10);
    group.bench_function("all_files", |b| {
        b.iter(|| assert_eq!(collect(tmp.path(), "*"), total))
    });
    group.bench_function("extension_glob", |b| {
        b.iter(|| assert_eq!(collect(tmp.path(), "*.md"), total / 4))
    });
    group.finish();
}

criterion_group!(benches, bench_collect);
criterion_main!(benches);
