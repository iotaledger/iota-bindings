#ifndef __IOTA_TRYTES_H_
#define __IOTA_TRYTES_H_

#include "ctrits.h"
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

extern void iota_trytes_num_int2trits(int64_t value, iota_ctrits_t *out);
extern int64_t iota_trytes_num_trits2int(iota_ctrits_t *);

#ifdef __cplusplus
}
#endif
#endif /* __IOTA_TRYTES_H_ */
