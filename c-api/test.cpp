#include <cstdint>
#include <cassert>

extern "C" {
    uint32_t parse(const char* s);
    uint32_t lookup(const char* name, uint32_t* handle_ptr);
    uint32_t next(uint32_t handle, uint32_t* result_ptr);
}

int main() {
    uint32_t err = 0;

    err = parse("a=5;");
    assert(err == 0);

    uint32_t handle = 0;
    err = lookup("a", &handle);
    assert(err == 0);
    assert(handle == 1);

    uint32_t result = 0;
    err = next(handle, &result);
    assert(err == 0);
    assert(result == 5);
}

