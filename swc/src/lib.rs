use std::path::Path;
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

pub fn transformer(filepath: impl Into<String>) -> impl VisitMut {
    transform::Eszett::new(filepath)
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
    let project_path = Path::new(&working_directory);
    let file_path = Path::new(&filename);
    let relative_path = match file_path.strip_prefix(&project_path) {
        Ok(s) => s,
        Err(_) => file_path,
    };

    program.visit_mut_with(&mut transformer(relative_path.display().to_string()));

    return program;
}
