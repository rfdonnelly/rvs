#include "rvs.h"

#include <cassert>

int main() {
    uint32_t err = 0;

    auto context = rvs_context_new();

    err = rvs_parse(context, "a=5;");
    assert(err == 0);

    uint32_t handle = 0;
    err = rvs_find(context, "a", &handle);
    assert(err == 0);
    assert(handle == 1);

    uint32_t result = 0;
    err = rvs_next(context, handle, &result);
    assert(err == 0);
    assert(result == 5);

    rvs_context_free(context);
}

