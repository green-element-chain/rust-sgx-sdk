/*
This internal module contains the terminal detection implementation.

If the `atty` crate is available then we use it to detect whether we're
attached to a particular TTY. If the `atty` crate is not available we
assume we're not attached to anything. This effectively prevents styles
from being printed.
*/

#[cfg(feature = "atty")]
mod imp {
    //use atty;

    pub(in ::fmt) fn is_stdout() -> bool {
        //atty::is(atty::Stream::Stdout)
        true
    }

    pub(in ::fmt) fn is_stderr() -> bool {
        //atty::is(atty::Stream::Stderr)
        true
    }
}

#[cfg(not(feature = "atty"))]
mod imp {
    pub(in ::fmt) fn is_stdout() -> bool {
        true
    }

    pub(in ::fmt) fn is_stderr() -> bool {
        true
    }
}

pub(in ::fmt) use self::imp::*;
