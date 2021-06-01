# Rust

*Rust* is primarily an expression language. This means that most forms of **value-producing** or **effect-causing evaluation** are directed by **the uniform syntax category of expressions.**

---

[Attributes](Rust%2005eba087ea254b6ab2a31ddfe1548560/Attributes%20901841f859f6452fa61aacf378d77807.md)

[Keywords](Rust%2005eba087ea254b6ab2a31ddfe1548560/Keywords%20839a2f45fa0648c88e43e01b286c2515.md)

[Control Flow](Rust%2005eba087ea254b6ab2a31ddfe1548560/Control%20Flow%20cd3f7bd007334e489bd1db930cc788fa.md)

[Generics, Trait, Lifetime](Rust%2005eba087ea254b6ab2a31ddfe1548560/Generics,%20Trait,%20Lifetime%20881d3a0fbc0841eeb08069f23c04017f.md)

[Method Syntax](Rust%2005eba087ea254b6ab2a31ddfe1548560/Method%20Syntax%20c90c030dbc7d488aab27b06336e25a47.md)

[Ownership](Rust%2005eba087ea254b6ab2a31ddfe1548560/Ownership%2020eeb0f827ed4bcd98c2c329963f840f.md)

[String formatting](Rust%2005eba087ea254b6ab2a31ddfe1548560/String%20formatting%2066eae80b1eaf489db825ac13ebb27ba4.md)

[Items](Rust%2005eba087ea254b6ab2a31ddfe1548560/Items%2045e0f5d4047f44b1b7770edb40b206a4.md)

---

`#[allow(dead_code)]` : 사용하지 않는 함수, enum 등에 대한 컴파일러 경고 없애기 위해 사용

![Rust%2005eba087ea254b6ab2a31ddfe1548560/Untitled.png](Rust%2005eba087ea254b6ab2a31ddfe1548560/Untitled.png)

![Rust%2005eba087ea254b6ab2a31ddfe1548560/Untitled%201.png](Rust%2005eba087ea254b6ab2a31ddfe1548560/Untitled%201.png)

# Setting

```bash
cargo init                 # 현재 디렉토리에서 프로젝트 생성
cargo new <project_name>
cargo new --lib <my_lib_name> 

cargo check                # 컴파일 가능여부 확인
cargo build  [--release]   # Build the local package and all of its dependencies
cargo run                  # build + execution
cargo clean                # Remove the entire target directory
cargo build --all-targets  # --lib --bins --tests --benches --examples

cargo update               # Update dependencies listed in Cargo.lock
cargo build --locked       # 원래는 Cargo.toml 기준으로 dependency에 맞는 최신 버전
                           # 패키지들을 받아옴. 이건 Cargo.lock을 기준으로
                           # 해당 버전에 고정된 패키지들을 받아옴 (production 용)

cargo doc [--no-deps] [-p <target_1>] [-p <target_2>] [...]
```

`cargo fix` : To read and apply the suggestions made by rustc

`cargo clippy` : A collection of lints to catch common mistakes

## 1) dependency 관리

*crate* 는 rust의 compilation, linking 단위이다. 이것은 source code, binary executable, binary library 형태로 존재할 수 있다. Rust project는 *package*로 불리고 하나 이상의 crate가 포함될 수 있다. 이 때 프로젝트에 포함된 `Cargo.toml`에 어떻게 이 *crate*들이 빌드될 지 작성된다.
*libary crate*는 다른 project에서 사용할 수 있도록 만든 컴포넌트의 그룹. 이것은 entry point(`fn main()`)를 포함하지 않음.

`rustup`은 rust toolchain의 installer/updater

`rustc`는 Rust의 compiler

*Cargo*는 Rust의 build system, package manager

*Crates.io*는 Rust community's crate registry

The `Cargo.toml` file for each package is called its *manifest*

```bash
rustup show
rustup component list

```

![Rust%2005eba087ea254b6ab2a31ddfe1548560/Untitled%202.png](Rust%2005eba087ea254b6ab2a31ddfe1548560/Untitled%202.png)

### ※ Cargo.toml

- `[cargo-features](https://doc.rust-lang.org/cargo/reference/unstable.html)` — Unstable, nightly-only features.
- `[[package]](https://doc.rust-lang.org/cargo/reference/manifest.html#the-package-section)` — Defines a package.
    - `[name](https://doc.rust-lang.org/cargo/reference/manifest.html#the-name-field)` — The name of the package.
    - `[version](https://doc.rust-lang.org/cargo/reference/manifest.html#the-version-field)` — The version of the package.
    - `[authors](https://doc.rust-lang.org/cargo/reference/manifest.html#the-authors-field)` — The authors of the package.
    - `[edition](https://doc.rust-lang.org/cargo/reference/manifest.html#the-edition-field)` — The Rust edition.
    - `[description](https://doc.rust-lang.org/cargo/reference/manifest.html#the-description-field)` — A description of the package.
    - `[documentation](https://doc.rust-lang.org/cargo/reference/manifest.html#the-documentation-field)` — URL of the package documentation.
    - `[readme](https://doc.rust-lang.org/cargo/reference/manifest.html#the-readme-field)` — Path to the package's README file.
    - `[homepage](https://doc.rust-lang.org/cargo/reference/manifest.html#the-homepage-field)` — URL of the package homepage.
    - `[repository](https://doc.rust-lang.org/cargo/reference/manifest.html#the-repository-field)` — URL of the package source repository.
    - `[license](https://doc.rust-lang.org/cargo/reference/manifest.html#the-license-and-license-file-fields)` — The package license.
    - `[license-file](https://doc.rust-lang.org/cargo/reference/manifest.html#the-license-and-license-file-fields)` — Path to the text of the license.
    - `[keywords](https://doc.rust-lang.org/cargo/reference/manifest.html#the-keywords-field)` — Keywords for the package.
    - `[categories](https://doc.rust-lang.org/cargo/reference/manifest.html#the-categories-field)` — Categories of the package.
    - `[workspace](https://doc.rust-lang.org/cargo/reference/manifest.html#the-workspace-field)` — Path to the workspace for the package.
    - `[build](https://doc.rust-lang.org/cargo/reference/manifest.html#the-build-field)` — Path to the package build script.
    - `[links](https://doc.rust-lang.org/cargo/reference/manifest.html#the-links-field)` — Name of the native library the package links with.
    - `[exclude](https://doc.rust-lang.org/cargo/reference/manifest.html#the-exclude-and-include-fields)` — Files to exclude when publishing.
    - `[include](https://doc.rust-lang.org/cargo/reference/manifest.html#the-exclude-and-include-fields)` — Files to include when publishing.
    - `[publish](https://doc.rust-lang.org/cargo/reference/manifest.html#the-publish-field)` — Can be used to prevent publishing the package.
    - `[metadata](https://doc.rust-lang.org/cargo/reference/manifest.html#the-metadata-table)` — Extra settings for external tools.
    - `[default-run](https://doc.rust-lang.org/cargo/reference/manifest.html#the-default-run-field)` — The default binary to run by `[cargo run](https://doc.rust-lang.org/cargo/commands/cargo-run.html)`.
    - `[autobins](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#target-auto-discovery)` — Disables binary auto discovery.
    - `[autoexamples](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#target-auto-discovery)` — Disables example auto discovery.
    - `[autotests](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#target-auto-discovery)` — Disables test auto discovery.
    - `[autobenches](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#target-auto-discovery)` — Disables bench auto discovery.
    - `[resolver](https://doc.rust-lang.org/cargo/reference/resolver.html#resolver-versions)` — Sets the dependency resolver to use.
- Target tables: (see [configuration](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#configuring-a-target) for settings)
    - `[[lib]](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#library)` — Library target settings.
    - `[[[bin]]](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#binaries)` — Binary target settings.
    - `[[[example]]](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#examples)` — Example target settings.
    - `[[[test]]](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#tests)` — Test target settings.
    - `[[[bench]]](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#benchmarks)` — Benchmark target settings.
- Dependency tables:
    - `[[dependencies]](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)` — Package library dependencies.
    - `[[dev-dependencies]](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#development-dependencies)` — Dependencies for examples, tests, and benchmarks.
    - `[[build-dependencies]](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#build-dependencies)` — Dependencies for build scripts.
    - `[[target]](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#platform-specific-dependencies)` — Platform-specific dependencies.
- `[[badges]](https://doc.rust-lang.org/cargo/reference/manifest.html#the-badges-section)` — Badges to display on a registry.
- `[[features]](https://doc.rust-lang.org/cargo/reference/features.html)` — Conditional compilation features.
- `[[patch]](https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html#the-patch-section)` — Override dependencies.
- `[[replace]](https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html#the-replace-section)` — Override dependencies (deprecated).
- `[[profile]](https://doc.rust-lang.org/cargo/reference/profiles.html)` — Compiler settings and optimizations.
- `[[workspace]](https://doc.rust-lang.org/cargo/reference/workspaces.html)` — The workspace definition.

## 2) Manage a package & modules

각 모듈안의 함수들은 각각 `pub`를 붙여줘야 외부로 노출됨

`struct`는 각 field별로 따로 pub 지정해줘야 함.

`enum`은 `pub`하면 이것의 variants도 모두 공개됨.

`pub use` 하면 re-export로 외부에서도 해당 경로로 접근 가능

```rust
// src/lib.rs

use crate::front_of_house::hosting;
use self::back_of_house as MyWorkspace;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}

	  pub enum Appetizer {
	          Soup,
	          Salad,
	      }

    pub struct Breakfast {
            pub toast: String,
            seasonal_fruit: String,
        }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // meal.seasonal_fruit = String::from("blueberries");
}
```

[tkxkd0159/rust](https://github.com/tkxkd0159/rust/tree/main/workspace_test)

# Style guide

![Rust%2005eba087ea254b6ab2a31ddfe1548560/Untitled%203.png](Rust%2005eba087ea254b6ab2a31ddfe1548560/Untitled%203.png)

In `CamelCase`, acronyms count as one word: use `Uuid` rather than `UUID`. In `snake_case`, acronyms are lower-cased: `is_xid_start`.

In `snake_case` or `SCREAMING_SNAKE_CASE`, a "word" should never consist of a single letter unless it is the last "word". So, we have `btree_map` rather than `b_tree_map`, but `PI_2` rather than `PI2`.

![Rust%2005eba087ea254b6ab2a31ddfe1548560/Untitled%204.png](Rust%2005eba087ea254b6ab2a31ddfe1548560/Untitled%204.png)

![Rust%2005eba087ea254b6ab2a31ddfe1548560/Untitled%205.png](Rust%2005eba087ea254b6ab2a31ddfe1548560/Untitled%205.png)