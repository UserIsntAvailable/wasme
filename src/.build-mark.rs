// This is needed to make the build script work.
//
// `src` and the build.rs can not be together, because the build script needs src to be `builded`
// to be able to run `wasm-pack`; threfore there is a deadlock, because `src` can not finish
// being builded if build.rs is not done, but build.rs can not finish building `src`, because it
// needs to be builded for `wasm-pack`.
fn __() {}
