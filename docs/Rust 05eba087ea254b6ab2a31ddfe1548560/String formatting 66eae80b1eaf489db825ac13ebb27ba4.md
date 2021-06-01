# String formatting

[std::fmt - Rust](https://doc.rust-lang.org/std/fmt/)

- `fmt::Debug`: Uses the `{:?}` marker. Format text for debugging purposes.
- `fmt::Display`: Uses the `{}` marker. Format text in a more elegant, user friendly fashion.
`fmt::Display` implementations assert that the type can be faithfully represented as a UTF-8 string at all times. It is not expected that all types implement the Display trait.

**- Formatting traits**

- *nothing* ⇒ `[Display](https://doc.rust-lang.org/std/fmt/trait.Display.html)`
- `?` ⇒ `[Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html)`
- `x?` ⇒ `[Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html)` with lower-case hexadecimal integers
- `X?` ⇒ `[Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html)` with upper-case hexadecimal integers
- `o` ⇒ `[Octal](https://doc.rust-lang.org/std/fmt/trait.Octal.html)`
- `x` ⇒ `[LowerHex](https://doc.rust-lang.org/std/fmt/trait.LowerHex.html)`
- `X` ⇒ `[UpperHex](https://doc.rust-lang.org/std/fmt/trait.UpperHex.html)`
- `p` ⇒ `[Pointer](https://doc.rust-lang.org/std/fmt/trait.Pointer.html)`
- `b` ⇒ `[Binary](https://doc.rust-lang.org/std/fmt/trait.Binary.html)`
- `e` ⇒ `[LowerExp](https://doc.rust-lang.org/std/fmt/trait.LowerExp.html)`
- `E` ⇒ `[UpperExp](https://doc.rust-lang.org/std/fmt/trait.UpperExp.html)`

```rust
assert_eq!(format!("Hello {:<5}!", "x"),  "Hello x    !");
assert_eq!(format!("Hello {:-<5}!", "x"), "Hello x----!");
assert_eq!(format!("Hello {:^5}!", "x"),  "Hello   x  !");
assert_eq!(format!("Hello {:>5}!", "x"),  "Hello     x!");
```

- `[fill]<` - the argument is left-aligned in `width` columns
- `[fill]^` - the argument is center-aligned in `width` columns
- `[fill]>` - the argument is right-aligned in `width` columns

```rust
assert_eq!(format!("Hello {:+}!", 5), "Hello +5!");
assert_eq!(format!("{:#x}!", 27), "0x1b!");
assert_eq!(format!("Hello {:05}!", 5),  "Hello 00005!");
assert_eq!(format!("Hello {:05}!", -5), "Hello -0005!");
assert_eq!(format!("{:#010x}!", 27), "0x0000001b!");
assert_eq!(format!("Hello {:08.3}!", 5.43),  "Hello 0005.430!");
```

- `+` - 양수도 sign 표시
- `#` - This flag indicates that the “alternate” form of printing should be used. The alternate forms are:
    - `#?` - pretty-print the `[Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html)` formatting
    - `#x` - precedes the argument with a `0x`
    - `#X` - precedes the argument with a `0x`
    - `#b` - precedes the argument with a `0b`
    - `#o` - precedes the argument with a `0o`
- `0N` - N만큼 해당 숫자 width를 가지고 남는 건 0으로 padding
- `.N` : precision (소수점 자리 수)

## Macro std::format

Creates a `String` using interpolation of runtime expressions.

```rust
assert_eq!(format!("{} {:?}", 3, 4), "3 4");
assert_eq!(format!("{} {:?}", 'a', 'b'), "a 'b'");
assert_eq!(format!("{} {:?}", "foo\n", "bar\n"), "foo\n \"bar\\n\"");
assert_eq!(format!("x = {}, y = {y}", 10, y = 30), "x = 10, y = 30");

assert_eq!("Hello   Some(\"hi\")   !", format!("Hello {:^15}!", format!("{:?}", Some("hi"))));
```