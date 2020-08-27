mod command;
mod name;

pub use command::*;
pub use name::*;

// helper: "SOMEthing" -> either "SOME" or "SOMEthing" based on verbose
fn pretty_name(name: &str, verbose: bool) -> &str {
    if verbose {
        name
    } else {
        let end = name
            .find(|c: char| !c.is_ascii_uppercase())
            .unwrap_or(name.len());
        &name[..end]
    }
}
