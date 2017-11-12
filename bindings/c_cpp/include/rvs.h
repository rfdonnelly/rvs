#ifdef __cplusplus
#include <cstdint>
#else
#include <stdint.h>
#endif

#ifdef __cplusplus
extern "C" {
#endif

void* rvs_context_new();
void rvs_context_free(void* context);
void rvs_seed(void* context, uint32_t seed);
uint32_t rvs_parse(void* context, const char* s);
uint32_t rvs_find(void* context, const char* id, uint32_t* handle_ptr);
uint32_t rvs_next(void* context, uint32_t handle, uint32_t* result_ptr);
uint32_t rvs_prev(void* context, uint32_t handle, uint32_t* result_ptr);
bool rvs_done(void* context, uint32_t handle, uint32_t* result_ptr);

#ifdef __cplusplus
}
#endif
