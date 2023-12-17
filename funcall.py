from cffi import FFI

ffi = FFI()

# Загрузка библиотеки Rust
rust_lib = ffi.dlopen("./target/release/rust_python_integration.dll")  # Замените на путь к вашей библиотеке

# Определение сигнатуры функции
# rust_lib.main_loop.argtypes = [ffi.size_t, ffi.size_t, ffi.size_t, ffi.size_t, ffi.double]
# rust_lib.main_loop.restype = None
ffi.cdef("""
    void main_loop(size_t, size_t, size_t, size_t, double);
""")

# Обертка для вызова функции из Python
def call_main_loop(n, c1, c2, c3, e):
    rust_lib.main_loop(n, c1, c2, c3, e)

# Пример вызова функции
# call_main_loop(10, 1, 2, 3, 0.01)