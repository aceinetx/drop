# drop
> [!WARNING]
> This project is Work In Progress, do not expect it to be usable right now

drop is a programming language inspired by Jai and Zig, features of drop include:<br>
1) No own standard library/runtime, you pay for what you use
2) Compilation down to C
3) Methods in structures
4) Tagged unions
5) Slices

drop is a C alternative, but is not meant to replace C, rather work along with it.

here's a hello world in drop:
```jai
extern fn puts (s: *const u8) i32;

fn main () i32 {
    return puts("Hello, World!");
}
```
