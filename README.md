# rename-files: Rename files according to regexp
Bart Massey 2021

This command-line tool will rename files using a regular
expression.

For example, in a directory containing

    file0.txt
    file1.txt

running via

    cargo run '[01]' 0\0 *.txt

will rename to

    file00.txt
    file01.txt

This tool currently only builds on Linux. Intended future
work is to make it build for Windows and MacOS.

This work is provided under the "MIT License". Please see
the file `LICENSE.txt` in this distro for license terms.
