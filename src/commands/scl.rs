use super::commands::Command;

// todo: Factor out a trait that hides the arg types for callers to use.

pub struct SclOptions {
    base: String,
    target: String,
}

pub fn new() -> &dyn Command {

}
