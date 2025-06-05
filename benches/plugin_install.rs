use criterion::{criterion_group, criterion_main, Criterion};
use lazy_tmux::path::tmux_dirs;
use lazy_tmux::{install_plugins, install_plugins_one_by_one};
use std::fs;
use std::sync::Once;

static INIT: Once = Once::new();

fn setup_plugins() {
    let dir = tmux_dirs::data_dir().unwrap();
    INIT.call_once(|| {
        // clean dirs or reset plugin folder state 
        fs::remove_dir_all(dir).ok();
    });
}

fn bench_parallel(c: &mut Criterion) {
    setup_plugins();
    c.bench_function("install_plugins (parallel)", |b| {
        b.iter(|| {
            install_plugins().unwrap();
        });
    });
}

fn bench_sequential(c: &mut Criterion) {
    setup_plugins();
    c.bench_function("install_plugins_one_by_one (sequential)", |b| {
        b.iter(|| {
            install_plugins_one_by_one().unwrap();
        });
    });
}

criterion_group!(benches, bench_parallel, bench_sequential);
criterion_main!(benches);

