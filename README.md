license
-------

license is a command line utility for managing license files in the
current directory. It automatically downloads license texts and saves
them as `LICENSE.txt` in the current directory.

### Future work

license currently only writes text files to the current directory. It
would also be nice to add licenses directly to the manifest (for example
`Cargo.toml` for Rust or `package.json` for Javascript).

There is also a lot of error handling I skipped in this first draft.