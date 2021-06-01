# Generics, Trait, Lifetime

# Generics

Generics are abstract stand-ins for concrete types or other properties.

# Trait

다른 언어에서 *interface*랑 비슷한 역할을 하는 것. 즉, type 사이 공유가능한 행위 정의

A *trait* tells the Rust compiler about functionality a particular type has and can share with other types. We can use traits **to define shared behavior in an abstract way.** We can use trait bounds to specify that a generic can be any type that has certain behavior.

# Bounds

`Where` clauses provide another way to specify bounds on type and lifetime parameters as well as a way to specify bounds on types that aren't type parameters.

```rust
impl <A, D> MyTrait<A, D> for YourType where
    A: TraitB + TraitC,
    D: TraitE + TraitF {}
```

- Generic parameter
`fn f<A: Copy>() {}`  ↔ `fn f<A> where A: Copy () {}`
- Supertraits
`trait Circle : Shape {}` ↔ `trait Circle where Self : Shape {}`

```rust
trait Shape { fn area(&self) -> f64; }
trait Circle : Shape { fn radius(&self) -> f64; }
```

- Associated types
`trait A { type B: Copy; }` ↔ `trait A where Self::B: Copy { type B; }`

# Lifetime

![Generics,%20Trait,%20Lifetime%20881d3a0fbc0841eeb08069f23c04017f/Untitled.png](Generics,%20Trait,%20Lifetime%20881d3a0fbc0841eeb08069f23c04017f/Untitled.png)

a variable's **lifetime** begins when it is created and ends when it is destroyed

```rust
fn main() {
    let i = 3; // Lifetime for `i` starts. ────────────────┐
    //                                                     │
    { //                                                   │
        let borrow1 = &i; // `borrow1` lifetime starts. ──┐│
        //                                                ││
        println!("borrow1: {}", borrow1); //              ││
    } // `borrow1 ends. ──────────────────────────────────┘│
    //                                                     │
    //                                                     │
    { //                                                   │
        let borrow2 = &i; // `borrow2` lifetime starts. ──┐│
        //                                                ││
        println!("borrow2: {}", borrow2); //              ││
    } // `borrow2` ends. ─────────────────────────────────┘│
    //                                                     │
}   // Lifetime ends. ─────────────────────────────────────┘
```

```rust
foo<'a, 'b>
// `foo` has lifetime parameters `'a` and `'b`
// the lifetime of foo cannot exceed that of either 'a or 'b.
```

## 1) Static

As a reference lifetime `'static` indicates that the data pointed to by the reference lives for the entire lifetime of the running program. It can still be coerced to a shorter lifetime.

```rust
static NUM: i32 = 18;

// Returns a reference to `NUM` where its `'static`
// lifetime is coerced to that of the input argument.
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn main() {
    {
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        // When `static_string` goes out of scope, the reference
        // can no longer be used, but the data remains in the binary.
    }

    {
        let lifetime_num = 9;
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);   // 18
    }

    println!("NUM: {} stays accessible!", NUM);          // 18
}
```