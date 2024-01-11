# C Project Manager
> A simple cli utility to create projects in C


# Makefile

- comes pre-configured to have debug mode with `make debug`
- also has `make release` which uses `-O3` optimization (you can change the level manually)
- also has `make lib` for quick `.so` generation
- and has `make init` if you cloned it from github and `build/` directories are missing
- `make clean` cleans some space

# Examples

### The examples are inside the `examples` folder

## Using the C language

examples found in the folders `c` and `c_empty` <br>
generated with `cargo run -- c` and `cargo run -- c --empty` respectively <br>


## Using C Plus Plus

examples can be found in the folders `cpp` and `cpp_empty` <br>
these were generated with `cargo run -- cpp -l cpp` and `cargo run -- cpp_empty -l cpp --empty` <br>

### `cargo run --` can be replace with the exe name


# The folder structure

**this is inside a folder which is given as the first argument to the cli with lang set as c (just like the c folder example)**
```
build/
    libs/
        headers/
        objs/
    objs/
headers/
src/
    main.c
.gitignore
cpi.toml
makefile
readme.md
```