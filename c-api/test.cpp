#include <cstdint>
#include <cassert>

extern "C" {
    void* sequence_context_new();
    void sequence_context_free(void* context);
    uint32_t sequence_parse(void* context, const char* s);
    uint32_t sequence_find(void* context, const char* name, uint32_t* handle_ptr);
    uint32_t sequence_next(void* context, uint32_t handle, uint32_t* result_ptr);
}

int main() {
    uint32_t err = 0;

    auto context = sequence_context_new();

    err = sequence_parse(context, "a=5;");
    assert(err == 0);

    uint32_t handle = 0;
    err = sequence_find(context, "a", &handle);
    assert(err == 0);
    assert(handle == 1);

    uint32_t result = 0;
    err = sequence_next(context, handle, &result);
    assert(err == 0);
    assert(result == 5);

    sequence_context_free(context);
}

