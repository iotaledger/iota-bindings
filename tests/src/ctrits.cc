#include <cstdio>
#include <gtest/gtest.h>
#include <string>

#include <iota/ctrits.h>

TEST(CTritsTest, TryteToTrits) {
  const char *trytes = "9A";
  iota_ctrits_t ctrytes = {IOTA_ENCODING_TRYTE, 2, (void *)trytes, 3};

  iota_ctrits_t *ctrits = iota_ctrits_convert(&ctrytes, IOTA_ENCODING_TRIT);
  uint8_t *trits = (uint8_t *)ctrits->data;

  EXPECT_EQ(ctrits->length, 6);
  EXPECT_EQ(ctrits->byte_length, 6);
  EXPECT_EQ(ctrits->encoding, IOTA_ENCODING_TRIT);

  EXPECT_EQ(trits[0], 0);
  EXPECT_EQ(trits[1], 0);
  EXPECT_EQ(trits[2], 0);
  EXPECT_EQ(trits[3], 1);
  EXPECT_EQ(trits[4], 0);
  EXPECT_EQ(trits[5], 0);

  iota_ctrits_drop(ctrits);
}

TEST(CTritsTest, TritsToTrytes) {
  int8_t trits[] = {1, 1, 1, -1, 0, 0};
  iota_ctrits_t ctrits = {IOTA_ENCODING_TRIT, 6, (void *)trits, 6};

  iota_ctrits_t *ctrytes = iota_ctrits_convert(&ctrits, IOTA_ENCODING_TRYTE);
  auto str = (char *)ctrytes->data;
  ASSERT_EQ(str, std::string("MZ"));
  ASSERT_EQ(ctrytes->byte_length - 1, ctrytes->length);
}
