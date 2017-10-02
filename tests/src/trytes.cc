#include <cstdint>
#include <gtest/gtest.h>
#include <string>

#include <iota/ctrits.h>
#include <iota/trytes.h>
#include <iota/tx_v2.h>

TEST(TrytesTest, int2tritsTest) {
  iota_ctrits_t *tx = iota_models_v2_tx_alloc_heap();

  iota_ctrits_t ctrits = {
    encoding : IOTA_ENCODING_TRIT,
    length : 8,
    data : tx->data,
    byte_length : 8
  };

  iota_trytes_num_int2trits(1337, &ctrits);

  int32_t out = iota_trytes_num_trits2int(&ctrits);

  ASSERT_EQ(out, 1337);

  iota_ctrits_drop(tx);
}
