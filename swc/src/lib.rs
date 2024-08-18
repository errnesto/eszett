use std::{hash::{DefaultHasher, Hash, Hasher}, path::Path};
use swc_core::{
    ecma::{
        ast::Program,
        visit::{VisitMut, VisitMutWith},
    },
    plugin::{
        metadata::TransformPluginMetadataContextKind, plugin_transform,
        proxies::TransformPluginProgramMetadata,
    },
};
mod transform;

pub fn transformer(working_directory: &Path, filename: &Path) -> impl VisitMut {
    let relative_path = match filename.strip_prefix(&working_directory) {
        Ok(s) => s,
        Err(_) => filename,
    };
    let mut hasher = DefaultHasher::new();
    relative_path.hash(&mut hasher);
    let filename_hash = hasher.finish().to_string();

    transform::Eszett::new(filename_hash)
}

#[plugin_transform]
pub fn process_transform(
    mut program: Program,
    metadata: TransformPluginProgramMetadata,
) -> Program {
    let working_directory = match metadata.get_context(&TransformPluginMetadataContextKind::Cwd) {
        Some(string) => string,
        None => String::from(""),
    };
    let filename = match metadata.get_context(&TransformPluginMetadataContextKind::Filename) {
        Some(string) => string,
        None => String::from(""),
    };

    program.visit_mut_with(&mut transformer(
        Path::new(&working_directory),
        Path::new(&filename),
    ));

    return program;
}
