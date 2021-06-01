# Control Flow

# If

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 4, 3");
    }
    
// Using if in a let Statement. return값의 type이 같아야 함.
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}
```

# Repetition with Loops

## 1) loop

`break` 하기 전까지 무한 반복

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
```

## 2) while

*condition*이 `true`이면 계속 진행, `false`면 `break`

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }
}
```

## 3) for

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in &a {
        println!("the value is: {}", element);
    }
}
```

# Match

switch ... case

## 1) `_` placeholder

처리하지 못하는 나머지 케이스를 `_`로 처리해줘야 함.

```rust
fn main() {
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}
```

## 2) Matching with Option<T>

`Option`은 2개의 variant로 구성되어 있음으로 `_` 처리 필요없음

```rust
fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
```

## 3) if let VS match

`if let`의 경우 `match`처럼 모든 케이스를 커버하지 않아도 됨.
일반적으로 하나의 케이스 매칭을 다룰 때 `if let` 사용

```rust
fn main() {
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }

    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        // The condition evaluated false. This branch is the default:
        println!("I don't like letters. Let's go with an emoticon :)!");
    }
}
```

```rust
let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
```