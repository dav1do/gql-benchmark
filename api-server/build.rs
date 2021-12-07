use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn write_schema(path: PathBuf, schema: String) {
    if path.exists() {
        std::fs::remove_file(path.clone()).unwrap();
    }
    let mut f = File::create(path).unwrap();
    f.write_all(schema.as_bytes()).unwrap();
    f.flush().unwrap();
}

fn main() {
    let cargo_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let graphql_dir = cargo_dir.parent().unwrap().join("bench-graphql");

    let schema = bench_graphql::new_schema().finish().sdl();
    write_schema(
        graphql_dir.join("schema").join("benchmarks.graphql"),
        schema,
    );
}
