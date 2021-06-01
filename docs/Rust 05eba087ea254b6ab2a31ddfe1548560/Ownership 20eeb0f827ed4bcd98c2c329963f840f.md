# Ownership

변수는 **stack**이나 **heap**에 저장된다. 고정된 크기의 데이터는 **stack**에 저장되고 컴파일 시점에 데이터 크기를 알 수 없거나 런타임에 동적으로 크기가 변하는 데이터는 **heap**에 저장함.

함수에 parameter를 넘길 때도 이 parameter들은 함수 scope의 **stack**에 저장된다. 
(Reference를 넘겨 받는 경우는 예외)

Rust가 다루는 각각의 값은 *owner*라는 변수를 가지고 있다. 특정 시점에 값의 *owner*는 하나이고 *owner*가 scope를 벗어나면 해당 값은 drop 된다.

```rust
fn main() {
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
}
```

# Reference

Rust에서는 `=` 의 의미가 기본적으로 값의 *copy*가 아닌 *move* 이다. 따라서 `copy` trait가 해당 type에 구현되어 있지 않으면 이것은 값의 이동을 유발한다.

```rust
struct Foo;

let x = Foo;
let y = x;

// `x` has moved into `y`, and so cannot be used
// println!("{:?}", x); // error: use of moved value
```

```rust
#[derive(Clone, Copy)]
struct Foo;

let x = Foo;
let y = x;
let z = x;
```

## - Shared references (&)

대입하는 변수의 *ownership*을 주지 않고 `&`을 활용해 *borrowing* 한다. (Copy trait 없는 변수 대상)

```rust
struct My;

fn consume(dd: &My){
    let b = dd;
}

fn main() {

    let a = My;
    consume(&a);
    let c = a;
    
}
```

## - Mutable Reference (&mut)

대입하는 변수의 값을 함수안에서 바꾸고 싶으면 `&mut` 을 활용해 Mutable Reference
you can have **only one mutable reference** to a particular piece of data in **a particular scope.**

```rust
fn main() {

    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        r1.push_str(", new String")
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s; //  hello, new string

}
```

**Error case:**

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
}
```

아래 case에서 에러가 발생하는 이유는 r1, r2가 사용되지 않았는데 r3에 의해 값이 변경될 수 없기 때문에 예측되지 않은 결과 발생 가능

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
}
```

따라서 아래처럼 작성하면 컴파일 됨.

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem, 이 이후에도 r1, r2 콜 하면 안됨.
    println!("{}", r3);
}
```

# Copy, Clone

`Clone` is a supertrait of `Copy`, so everything which is `Copy` must also implement `Clone`

## 1) Copy

Trait `std::marker::Copy`  for Stack-Only Data

Types whose values can be duplicated simply by copying bits.

### Copy 적용된 type list

- Numeric type (Integer, floating-point)
- Boolean
- `char`, `str`
- Copy가 적용된 element만 포함하는 array, tuple
- Closure
- Function pointer
- Result, Option 등등

엄밀히 말하면 `str`은 copy가 적용되었다기 보단 참조형태라 ownership이 안넘어감.

아래는 적용된 전체 list

[https://doc.rust-lang.org/std/marker/trait.Copy.html#foreign-impls](https://doc.rust-lang.org/std/marker/trait.Copy.html#foreign-impls)

## 2) Clone

Trait `std::clone::Clone`

A common trait for the ability to explicitly duplicate an object.

```rust
#[derive(Debug, Clone, Copy)]
struct Unit;

#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

fn main() {
    let unit = Unit;
    // Copy `Unit`, there are no resources to move
    let copied_unit = unit;

    // Both `Unit`s can be used independently
    println!("original: {:?}", unit);
    println!("copy: {:?}", copied_unit);

    // Instantiate `Pair`
    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);

    // Move `pair` into `moved_pair`, moves resources
    let moved_pair = pair;
    println!("moved: {:?}", moved_pair);

    // Clone `moved_pair` into `cloned_pair` (resources are included)
    let cloned_pair = moved_pair.clone();
    // Drop the original pair using std::mem::drop
    drop(moved_pair);

    // Error! `moved_pair` has been dropped
    //println!("copy: {:?}", moved_pair);

    // The result from .clone() can still be used!
    println!("clone: {:?}", cloned_pair);
}
```