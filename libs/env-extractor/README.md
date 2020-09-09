# env-extractor

## Usage

Basic example to extract an environment variable as String :

```rust
use env_extractor::{env_var, required};

fn load_path() -> required::Result<String> {
    env_var("PATH").as_required()
}
```

Another example to use custom type :

```rust
fn load_my_path() -> required::Result<MyPath> {
    // Note that this is exactly the same as load_path()
    env_var("PATH").as_required()
}
```

How to convert is represented using built-in `FromStr` :

```rust
struct MyPath {
    inner: String,
}
impl FromStr for MyPath {
    type Err = <String as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(MyPath {
            inner: s.to_string(),
        })
    }
}
```

Of course, `required::Result` can tell us the key when it's not present (but `std::env` cannot).

```rust
match load_my_path() {
    Ok(path) => println!("path: {}", path.inner),
    Err(NotPresent(key)) => println!("not present: {}", key),
    Err(e) => println!("unexpected error: {:?}", e),
}
```

Use `as_option()` instead, when handling optional value :

```rust
let sample: Option<MyPath> = env_var("foooooo").as_optional()?;
```
