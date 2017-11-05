#include <cstdint>
#include <cassert>

extern "C" {
    uint32_t sequence_parse(const char* s);
    uint32_t sequence_find(const char* name, uint32_t* handle_ptr);
    uint32_t sequence_next(uint32_t handle, uint32_t* result_ptr);
}

int main() {
    uint32_t err = 0;

    err = sequence_parse("a=5;");
    assert(err == 0);

    uint32_t handle = 0;
    err = sequence_find("a", &handle);
    assert(err == 0);
    assert(handle == 1);

    uint32_t result = 0;
    err = sequence_next(handle, &result);
    assert(err == 0);
    assert(result == 5);
}

