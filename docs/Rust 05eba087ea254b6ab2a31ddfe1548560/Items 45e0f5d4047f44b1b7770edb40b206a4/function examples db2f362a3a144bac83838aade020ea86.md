# function examples

```rust
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    // Corner case, early return
    if rhs == 0 {
        return false;
    }

    // This is an expression, the `return` keyword is not necessary here
    lhs % rhs == 0
}
```

## Multiple output types

```rust
use std::collections::HashMap;

enum Either<A, B> {
    Left(A),
    Right(B),
}

fn one_or_the_other (argument: bool) -> Either<HashMap<u32, ()>, String> {
    if argument {
        Either::Left(HashMap::new())
    }
    else {
        Either::Right("A string".to_string())
    }
}

fn main () {

    match one_or_the_other(false) {
        Either::Left(_) => println!("Returned a HashMap"),
        Either::Right(param) => println!("{0} Returned a String {0}", param),
    }
    

```

## Return multi values

```rust
fn addsub(x: i32, y: i32) -> (i32, i32) {
    (x + y, x - y)
}

fn main() {
    let a = 3;
    let b = 4;
    
    let (c, d) = addsub(a, b);
    
    println!("{} {}", c, d);
}
```