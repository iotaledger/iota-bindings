#ifndef __IOTA_TX_V2_H_
#define __IOTA_TX_V2_H_

#include "ctrits.h"
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

extern iota_ctrits_t *iota_models_v2_tx_alloc_heap(void);

extern iota_ctrits_t *iota_models_v2_tx_signature_or_message(iota_ctrits_t *);
extern iota_ctrits_t *iota_models_v2_tx_address(iota_ctrits_t *);
extern int64_t iota_models_v2_tx_value(iota_ctrits_t *);
extern iota_ctrits_t *iota_models_v2_tx_obsolete_tag(iota_ctrits_t *);
extern uint64_t iota_models_v2_tx_timestamp(iota_ctrits_t *);
extern uint64_t iota_models_v2_tx_current_index(iota_ctrits_t *);
extern uint64_t iota_models_v2_tx_last_index(iota_ctrits_t *);
extern iota_ctrits_t *iota_models_v2_tx_bundle(iota_ctrits_t *);
extern iota_ctrits_t *iota_models_v2_tx_trunk(iota_ctrits_t *);
extern iota_ctrits_t *iota_models_v2_tx_branch(iota_ctrits_t *);
extern iota_ctrits_t *iota_models_v2_tx_tag(iota_ctrits_t *);
extern uint64_t iota_models_v2_tx_attachment_timestamp(iota_ctrits_t *);
extern uint64_t iota_models_v2_tx_attachment_timestamp_lower(iota_ctrits_t *);
extern uint64_t iota_models_v2_tx_attachment_timestamp_upper(iota_ctrits_t *);
extern iota_ctrits_t *iota_models_v2_tx_nonce(iota_ctrits_t *);

extern void iota_models_v2_tx_set_signature_or_message(iota_ctrits_t *,
                                                       iota_ctrits_t *);
extern void iota_models_v2_tx_set_address(iota_ctrits_t *, iota_ctrits_t *);
extern void iota_models_v2_tx_set_value(iota_ctrits_t *, int64_t);
extern void iota_models_v2_tx_set_obsolete_tag(iota_ctrits_t *,
                                               iota_ctrits_t *);
extern void iota_models_v2_tx_set_timestamp(iota_ctrits_t *, uint64_t);
extern void iota_models_v2_tx_set_current_index(iota_ctrits_t *, uint64_t);
extern void iota_models_v2_tx_set_last_index(iota_ctrits_t *, uint64_t);
extern void iota_models_v2_tx_set_bundle(iota_ctrits_t *, iota_ctrits_t *);
extern void iota_models_v2_tx_set_trunk(iota_ctrits_t *, iota_ctrits_t *);
extern void iota_models_v2_tx_set_branch(iota_ctrits_t *, iota_ctrits_t *);
extern void iota_models_v2_tx_set_tag(iota_ctrits_t *, iota_ctrits_t *);
extern void iota_models_v2_tx_set_attachment_timestamp(iota_ctrits_t *,
                                                       uint64_t);
extern void iota_models_v2_tx_set_attachment_timestamp_lower(iota_ctrits_t *,
                                                             uint64_t);
extern void iota_models_v2_tx_set_attachment_timestamp_upper(iota_ctrits_t *,
                                                             uint64_t);
extern void iota_models_v2_tx_set_nonce(iota_ctrits_t *, iota_ctrits_t *);

#ifdef __cplusplus
}
#endif

#endif /* __IOTA_TX_V2_H_ */
