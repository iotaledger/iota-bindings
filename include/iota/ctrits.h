#ifndef __IOTA_CTRITS_H_
#define __IOTA_CTRITS_H_

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

  enum iota_trit_encoding {
    IOTA_ENCODING_BYTE = 1,
    IOTA_ENCODING_TRIT = 2,
    IOTA_ENCODING_TRYTE = 3
  };

  typedef uint32_t iota_trit_encoding_t;

  typedef struct {
    iota_trit_encoding_t encoding;
    size_t length;
    void* data;
    size_t byte_length;
  } iota_ctrits_t;

  extern iota_ctrits_t* iota_ctrits_convert(const iota_ctrits_t* ctrits, const iota_trit_encoding_t to);

  extern void iota_ctrits_drop(iota_ctrits_t*);

#ifdef __cplusplus
}
#endif
#endif /* __IOTA_CTRITS_H_ */
