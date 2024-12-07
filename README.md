# spellfix1

```rust
use rusqlite::ffi::sqlite3_auto_extension;
use sqlite_spellfix1::sqlite3_spellfix_init;

fn main() {
    unsafe {
        sqlite3_auto_extension(Some(std::mem::transmute(
            sqlite3_spellfix_init as *const (),
        )));
    }
}
```
