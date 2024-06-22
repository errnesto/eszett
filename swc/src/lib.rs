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
pub fn process_transform(mut program: Program, data: TransformPluginProgramMetadata) -> Program {
    let filepath = match data.get_context(&TransformPluginMetadataContextKind::Filename) {
        Some(s) => s,
        None => String::from(""),
    };
    program.visit_mut_with(&mut transformer(filepath));

    return program;
}
