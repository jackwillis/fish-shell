use super::prelude::*;

pub fn bind(
    _parser: &Parser,
    _streams: &mut IoStreams<'_>,
    _args: &mut [WString],
) -> Option<c_int> {
    FLOG!(error, "builtin bind is not implemented");
    // todo!("builtin")
    Some(0)
}
