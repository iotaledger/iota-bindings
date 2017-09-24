#ifndef __IOTA_MERKLE_H_
#define __IOTA_MERKLE_H_

#include <stddef.h>
#include <stdint.h>

#include "ctrits.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
} iota_merkle_tree_t;
typedef struct {
} iota_merkle_branch_t;

extern iota_merkle_tree_t *iota_merkle_create(const iota_ctrits_t *seed,
                                              const size_t index,
                                              const size_t count,
                                              const uint8_t security);

extern void iota_merkle_drop(iota_merkle_tree_t *tree);
extern size_t iota_merkle_size(const iota_merkle_tree_t *tree);
extern size_t iota_merkle_depth(const iota_merkle_tree_t *tree);
extern size_t iota_merkle_count(const iota_merkle_tree_t *tree);
extern iota_ctrits_t *iota_merkle_slice(const iota_merkle_tree_t *tree);

extern iota_merkle_branch_t *iota_merkle_branch(const iota_merkle_tree_t *node,
                                                const size_t index);

extern void iota_merkle_branch_drop(iota_merkle_branch_t *branch);
extern size_t iota_merkle_branch_len(const iota_merkle_branch_t *branch);
extern iota_ctrits_t *iota_merkle_siblings(const iota_merkle_branch_t *branch);

extern iota_ctrits_t *iota_merkle_root(const iota_ctrits_t *address,
                                       const iota_ctrits_t *siblings,
                                       const size_t index);

#ifdef __cplusplus
}
#endif
#endif /* __IOTA_MERKLE_H_ */
