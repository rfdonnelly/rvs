#include "rvs.h"

#include <cassert>
#include <iostream>

int main() {
    uint32_t err = 0;

    auto error = rvs_error_new();
    auto context = rvs_context_new("", 0, error);
    if (rvs_error_test(error)) {
        std::cout << "error: " << rvs_error_message(error) << std::endl;
    }
    assert(rvs_error_test(error) == 0);

    rvs_parse(context, "a=5;", error);
    if (rvs_error_test(error)) {
        std::cout << "error: " << rvs_error_message(error) << std::endl;
    }
    assert(rvs_error_test(error) == 0);
    auto model = rvs_transform(context, error);
    if (rvs_error_test(error)) {
        std::cout << "error: " << rvs_error_message(error) << std::endl;
    }
    assert(rvs_error_test(error) == 0);

    auto handle = rvs_get(model, "a");
    assert(handle == 1);

    auto result = rvs_next(model, handle);
    assert(result == 5);

    rvs_error_free(error);
    rvs_model_free(model);
}
