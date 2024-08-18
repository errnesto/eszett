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
    let filepath = match metadata.get_context(&TransformPluginMetadataContextKind::Filename) {
        Some(s) => s,
        None => String::from(""),
    };
    let config = metadata
        .get_transform_plugin_config();
        // .get("root")
        // .and_then(|value| value.as_str())
        // .map(PathBuf::from)
        // .unwrap_or_else(|| {
        //     // Default to the current directory if not specified
        //     std::env::current_dir().expect("Failed to get current directory")
        // });

    println!("------------------------");
    println!("{:?}", config);
    println!("------------------------");

    // let relative_path = match filepath.strip_prefix(&project_root) {
    //     Some(s) => s,
    //     None => &filepath,
    // };

    program.visit_mut_with(&mut transformer(filepath));

    return program;
}
