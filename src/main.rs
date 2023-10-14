use syntect::{
    dumps::dump_to_uncompressed_file,
    parsing::{SyntaxDefinition, SyntaxSet},
};

fn main() {
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let ts = SyntaxDefinition::load_from_str(
        include_str!("./TypeScriptReact.sublime-syntax"),
        false,
        None,
    )
    .unwrap();
    let mut builder = syntax_set.into_builder();
    builder.add(ts);
    let syntax_set = builder.build();
    dump_to_uncompressed_file(&syntax_set, "syntaxes.bin").unwrap();
}
