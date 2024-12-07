fn main() {
    let mut cfg = cc::Build::new();
    cfg.file("sqlite/spellfix.c")
        .define("SQLITE_CORE", None)
        .compile("spellfix");
}
