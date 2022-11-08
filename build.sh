parentdir="$(dirname $(pwd))"
cargo build --release
cp target/release/jsonschema2md $parentdir/documentation_compilator/jsonschema2md