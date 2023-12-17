from cffi import FFI

ffi = FFI()

# Загрузка библиотеки Rust
rust_lib = ffi.dlopen("./target/release/rust_python_integration.dll")  # Замените на путь к вашей библиотеке

# Определение сигнатуры функции
ffi.cdef("""
    void main_loop(size_t, size_t, size_t, size_t, double);
""")

ffi.cdef("""
    void main_loop_lab2(char *, size_t, size_t, size_t);
""")

ffi.cdef("""
    void parse_logs(char *, size_t);
""")

# Обертка для вызова функции из Python
def call_main_loop(n, c1, c2, c3, e):
    rust_lib.main_loop(n, c1, c2, c3, e)

def call_main_loop_lab2(src, tet, r, target):
    rust_lib.main_loop_lab2(src.encode('utf-8'), tet, r, target)

def call_parse_logs(path, max_rank):
    rust_lib.parse_logs(path.encode('utf-8'), max_rank)

# Пример вызова функции
# call_main_loop(10, 1, 2, 3, 0.01)