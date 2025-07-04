#![forbid(unsafe_code)]

mod auditable_from_metadata;
mod binary_file;
mod cargo_arguments;
mod cargo_auditable;
mod collect_audit_data;
mod object_file;
mod platform_detection;
mod rustc_arguments;
mod rustc_wrapper;
mod sbom_precursor;
mod target_info;

/// Dispatches the call to either `cargo auditable` when invoked through cargo,
/// or to `rustc_wrapper` when Cargo internals invoke it
fn main() {
    let first_arg = std::env::args_os().nth(1);
    if let Some(arg) = first_arg {
        if arg == "auditable" {
            cargo_auditable::main(2)
        }
        // When this binary is called as a rustc wrapper, the first argument is the path to rustc:
        // https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-reads
        // It's important to read it because it can be overridden via env vars or config files.
        // In order to distinguish that from someone running the binary directly by mistake,
        // we check if the env var we set earlier is still present.
        // The "rustc" special-case is purely to accommodate the weird things `sccache` does:
        // https://github.com/rust-secure-code/cargo-auditable/issues/87
        // We should push back and make it sccache's problem if this ever causes issues.
        else if arg == "rustc" || std::env::var_os("CARGO_AUDITABLE_ORIG_ARGS").is_some() {
            rustc_wrapper::main(&arg)
        } else {
            cargo_auditable::main(1)
        }
    } else {
        cargo_auditable::main(1)
    }
}
