1) Создайте код Rust:
   Создайте новый проект Rust и добавьте функцию, которую вы хотите вызвать из Python. Например, создайте файл с именем lib.rs:

#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

Атрибут #[no_mangle] в Rust говорит компилятору не изменять имя функции при генерации объектного кода.

2) Создайте Cargo.toml:
   Создайте файл Cargo.toml в том же каталоге:

[package]
name = "rust_python_integration"
version = "0.1.0"

[lib]
name = "rust_python_integration"
crate-type = ["cdylib"]

[dependencies]

3) Соберите библиотеку Rust:
   В терминале выполните

cargo build --release

После выполнения этой команды в вашем каталоге появится файл target/release/librust_python_integration.so (или .dll в Windows).



pip install cffi
pip install tk

py main.py