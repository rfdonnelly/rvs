#ifndef __rvs_h__
#define __rvs_h__

#ifdef __cplusplus
#include <cstdint>
#else
#include <stdint.h>
#endif

#ifdef __cplusplus
extern "C" {
#endif

typedef struct rvs_error rvs_error;
typedef struct rvs_context rvs_context;
typedef struct rvs_model rvs_model;

rvs_context* rvs_context_new(const char* search_path, uint32_t seed, rvs_error* error);
void rvs_context_free(rvs_context* context);
void rvs_parse(rvs_context* context, const char* s, rvs_error* error);
rvs_model* rvs_transform(rvs_context* context, rvs_error* error);
void rvs_model_free(rvs_model* model);

uint32_t rvs_get(rvs_model* model, const char* id);
uint32_t rvs_next(rvs_model* model, uint32_t handle);
uint32_t rvs_prev(rvs_model* model, uint32_t handle);
bool rvs_done(rvs_model* model, uint32_t handle);

rvs_error* rvs_error_new();
void rvs_error_free(rvs_error* error);
bool rvs_error_test(rvs_error* error);
const char* rvs_error_message(rvs_error* error);

#ifdef __cplusplus
}
#endif

#endif
