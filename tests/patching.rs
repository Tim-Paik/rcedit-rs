use std::{
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
};

use rcedit::ResourceUpdater;

fn copy_test_binary() -> PathBuf {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    hasher.write(b"neutauri_runtime");
    std::time::SystemTime::now().hash(&mut hasher);
    println!("Hash is {:x}!", hasher.finish());
    let temp_path = std::env::temp_dir().join(format!("{:x}.exe", hasher.finish()));

    std::fs::write(
        &temp_path,
        std::fs::read("tests/data/fake_resources_binary.exe").unwrap(),
    )
    .unwrap();
    temp_path
}

fn patch(binary_path: &Path) {
    let mut updater = ResourceUpdater::new();
    updater.load(&binary_path).unwrap();
    updater
        .set_icon(Path::new("tests/data/new_icon.ico"))
        .unwrap();
    updater
        .set_rcdata(102, Path::new("tests/data/new_rcdata.txt"))
        .unwrap();
    updater.set_string(103, "Lorem ipsum").unwrap();
    updater.commit().unwrap();
}

#[test]
fn patching_test() {
    let binary_path = copy_test_binary();
    patch(&binary_path);
    let patched_file = std::fs::File::open(&binary_path).unwrap();
    assert_eq!(patched_file.metadata().unwrap().len(), 12288);
    std::fs::remove_file(&binary_path).unwrap();
}
