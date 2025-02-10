Several solutions exist to resolve this issue. Here are two examples:

**Solution 1: Using a single mutable reference:**

```rust
fn main() {
    let mut x = 5;
    let y = &mut x;
    *y = 10;
    println!("x = {}", x);
}
```
This solution uses only one mutable reference to `x`. 

**Solution 2: Cloning the value:**

```rust
fn main() {
    let mut x = 5;
    let mut y = x;
    let mut z = x;
    y = 10;
    z = 15;
    println!("x = {}, y = {}, z = {}", x, y, z);
}
```
This approach creates copies of `x`, allowing independent modification.  Note that this requires a `Copy` trait for the type of `x`.  If `x` were a larger or non-`Copy` structure this would not be as efficient.