use arma3_database::{DatabaseManager, ClassRepository, MissionRepository, GraphQueryEngine};
use criterion::{criterion_group, criterion_main, Criterion};
use std::path::PathBuf;

const BENCHMARK_DB_PATH: &str = r"D:\pca\git\dep\rs\arma3_tool\cache\pca_next\benchmark.db";

fn get_benchmark_db() -> DatabaseManager {
    let path = PathBuf::from(BENCHMARK_DB_PATH);
    if !path.exists() {
        panic!("Benchmark database not found at: {}", BENCHMARK_DB_PATH);
    }
    DatabaseManager::new(&path).expect("Failed to open benchmark database")
}

fn bench_class_queries(c: &mut Criterion) {
    let db = get_benchmark_db();
    let class_repo = ClassRepository::new(&db);
    
    let mut group = c.benchmark_group("class_queries");
    
    group.bench_function("get_all_classes", |b| {
        b.iter(|| {
            std::hint::black_box(class_repo.get_all().expect("Failed to get all classes"));
        });
    });
    
    group.finish();
}

fn bench_graph_operations(c: &mut Criterion) {
    let db = get_benchmark_db();
    let graph_engine = GraphQueryEngine::new(&db);
    
    let mut group = c.benchmark_group("graph_operations");
    
    // Benchmark building class hierarchy graph
    group.bench_function("build_root_hierarchy", |b| {
        b.iter(|| {
            std::hint::black_box(graph_engine.build_class_hierarchy_graph(
                Some("Object"),
                10, // Max depth
                None,
            ).expect("Failed to build hierarchy"));
        });
    });
    
    // Benchmark building multiple root hierarchies
    group.bench_function("build_multiple_root_hierarchies", |b| {
        b.iter(|| {
            std::hint::black_box(graph_engine.build_class_hierarchy_graph(
                None, // All roots
                5,    // Lower max depth for this benchmark
                None,
            ).expect("Failed to build hierarchy"));
        });
    });
    
    // Benchmark with exclusion patterns
    let exclude_patterns = vec![
        "CAManBase_".to_string(),
        "Helicopter".to_string(),
    ];
    
    group.bench_function("build_hierarchy_with_exclusions", |b| {
        b.iter(|| {
            std::hint::black_box(graph_engine.build_class_hierarchy_graph(
                Some("Object"),
                10,
                Some(&exclude_patterns),
            ).expect("Failed to build hierarchy"));
        });
    });
    
    group.finish();
}

fn bench_mission_queries(c: &mut Criterion) {
    let db = get_benchmark_db();
    let mission_repo = MissionRepository::new(&db);
    
    let mut group = c.benchmark_group("mission_queries");
    
    // Get all missions
    group.bench_function("get_all_missions", |b| {
        b.iter(|| {
            std::hint::black_box(mission_repo.get_all().expect("Failed to get all missions"));
        });
    });
    
    // Test getting mission components and dependencies
    group.bench_function("get_mission_components", |b| {
        b.iter(|| {
            std::hint::black_box(mission_repo.get_components("mission_1").expect("Failed to get components"));
        });
    });
    
    group.bench_function("get_mission_dependencies", |b| {
        b.iter(|| {
            std::hint::black_box(mission_repo.get_dependencies("mission_1").expect("Failed to get dependencies"));
        });
    });
    
    group.bench_function("get_all_dependencies", |b| {
        b.iter(|| {
            std::hint::black_box(mission_repo.get_all_dependencies().expect("Failed to get all dependencies"));
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_class_queries,
    bench_graph_operations,
    bench_mission_queries
);
criterion_main!(benches);