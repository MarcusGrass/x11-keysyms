use codegen_rs::structures::visibility::Visibility;
use codegen_rs::structures::RustType;
use codegen_rs::{ConstantBuilder, FileBuilder, ModuleBuilder};
use std::collections::HashSet;

fn main() {
    let source = std::fs::read_to_string("x11-keysyms-runner/keysymdef.h").unwrap();
    let mut current_define = None;
    let mut features = HashSet::new();
    let mut lib = FileBuilder::new("lib");
    for line in source.lines() {
        if line.starts_with("#ifdef") {
            let mut split = line.split(' ');
            split.next();
            let feature = split.next().unwrap();
            current_define = Some(
                feature
                    .to_lowercase()
                    .replace("xk", "")
                    .trim_matches('_')
                    .replace('_', "-"),
            );
        } else if line.starts_with("#endif") {
            current_define = None;
        }
        if line.starts_with("#define") {
            let mut split = line.split(' ');
            split.next();
            let name = split.next();
            for n in split {
                if n.is_empty() {
                    continue;
                }
                let mut constant_builder =
                    ConstantBuilder::const_builder(name.unwrap(), RustType::in_scope("u32"), n)
                        .set_visibility(Visibility::Public);
                if let Some(feature) = current_define.as_ref() {
                    constant_builder = constant_builder
                        .add_simple_annotation(format!("cfg(feature = \"{feature}\")"));
                    features.insert(feature.clone());
                }
                lib = lib.add_const(constant_builder);
                break;
            }
        }
    }
    ModuleBuilder::new(lib.add_simple_annotation("allow(non_upper_case_globals)"))
        .write_to_disk("x11-keysyms/src")
        .unwrap();
    let mut v = Vec::from_iter(features);
    v.sort();
    for val in &v {
        eprintln!("{val} = []");
    }
    eprintln!("all = [");
    for val in &v {
        eprintln!("    \"{val}\",")
    }
    eprintln!("]");
    //eprintln!("all = [{}]", v.join(", "));
}
