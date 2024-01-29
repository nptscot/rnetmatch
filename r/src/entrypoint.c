// We need to forward routine registration from C to Rust
// to avoid the linker removing the static library.

void R_init_rnetmatch_extendr(void *dll);

void R_init_rnetmatch(void *dll) {
    R_init_rnetmatch_extendr(dll);
}
