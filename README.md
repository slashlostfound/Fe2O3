# Fe2O3
"Iron(III) oxide or ferric oxide is the inorganic compound with the formula Fe2O3. Iron(III) oxide is often called rust, and to some extent this label is useful, because rust shares several properties and has a similar composition. To a chemist, rust is considered an ill-defined material, described as hydrated ferric oxide."

This is an ELF prepender. It takes a binary in and spits it out into every ELF file in the CWD. Cool!

### Build
```
% cargo build 
```

### Usage
```
% lfe ./virus
```
This will take the binary file `virus` and prepend it onto every ELF binary in the current working directory.

Unlike the original source, this fork is built and tested on OpenBSD, but it *should* (and that is a very unsure should) work on Linux.
