#include "rvs.h"

#include <cassert>
#include <iostream>

int main() {
    uint32_t err = 0;

    auto context = rvs_context_new();
    auto error = rvs_error_new();

    rvs_parse(context, "a=5;", error);
    if (rvs_error_code(error)) {
        std::cout << "error: " << rvs_error_message(error) << std::endl;
    }
    assert(rvs_error_code(error) == 0);
    rvs_transform(context, error);
    if (rvs_error_code(error)) {
        std::cout << "error: " << rvs_error_message(error) << std::endl;
    }
    assert(rvs_error_code(error) == 0);

    auto handle = rvs_find(context, "a");
    assert(handle == 1);

    auto result = rvs_next(context, handle);
    assert(result == 5);

    rvs_error_free(error);
    rvs_context_free(context);
}
