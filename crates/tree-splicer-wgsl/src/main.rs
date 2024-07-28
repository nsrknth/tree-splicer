use anyhow::Result;

fn main() -> Result<()> {
    tree_splicer::cli::main(tree_sitter_wgsl::language(), tree_sitter_wgsl::NODE_TYPES)
}
