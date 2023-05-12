extern crate embed_resource;
fn main() {
    embed_resource::compile("example-manifest.rc", embed_resource::NONE);
}
