use std::{ffi::OsString, error::Error};

/// A CLI command that can be invoked with an `Iter` of `Item`.
pub(crate) trait Command<Iter, Item>
where
    Iter: IntoIterator<Item = Item>,
    Item: Into<OsString> + Clone, {

    fn run(&self, itr: Iter) -> Result<(), Box<dyn Error>>;
}
