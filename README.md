# rename-files: Rename files according to regexp
Bart Massey 2021

This command-line tool will rename files using a regular
expression.

For example, in a directory containing

    file0.txt
    file1.txt

running via

    cargo run '[0-9]+' '0$0' *.txt

will rename to

    file00.txt
    file01.txt

You can test that this example works with `sh test.sh`.

This tool currently only builds on Linux. Intended future
work is to make it build for Windows and MacOS by splitting
out the OS-dependent functionality into a separate
general-purpose crate.

This work is provided under the "MIT License". Please see
the file `LICENSE.txt` in this distro for license terms.

This app took me about four hours to create. Most of those
four hours were spent working out how to use a new-to-me
argument parsing crate (about 1.5 hours), working out how to
use the `regex` crate on `OsString` (about half an hour),
and implementing a safe wrapper for the Linux `renameat2()`
syscall from the `libc` crate (about an hour).
