#ifndef __IOTA_MAM_H_
#define __IOTA_MAM_H_

#include <stddef.h>
#include <stdint.h>

#include "ctrits.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
  iota_ctrits_t *message;
  iota_ctrits_t *next_root;
} iota_mam_parse_result_t;

extern iota_ctrits_t *
iota_mam_create(const iota_ctrits_t *seed, const iota_ctrits_t *message,
                const iota_ctrits_t *key, const iota_ctrits_t *root,
                const iota_ctrits_t *siblings, const iota_ctrits_t *next_root,
                size_t start, size_t index, uint8_t security);

extern iota_mam_parse_result_t *iota_mam_parse(const iota_ctrits_t *payload,
                                               const iota_ctrits_t *side_key,
                                               const iota_ctrits_t *root);

extern iota_ctrits_t *iota_mam_id(const iota_ctrits_t *key,
                                  const iota_ctrits_t *root);

#ifdef __cplusplus
}
#endif
#endif /* __IOTA_MAM_H_ */
