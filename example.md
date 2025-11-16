# Por qué Rust es Genial

## Seguridad y Rendimiento
Rust se centra en la **seguridad de la memoria** sin usar un recolector de basura. Su modelo de *ownership* y *borrowing* garantiza que los problemas comunes de otros lenguajes (como *data races* o *null pointers*) se detecten en **tiempo de compilación**.

```rust
fn main() {
    let mut x = 5;
    println!("El valor es: {}", x);
}