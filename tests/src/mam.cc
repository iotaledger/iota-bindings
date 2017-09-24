#include <cstdio>
#include <gtest/gtest.h>
#include <string>

#include <iota/ctrits.h>
#include <iota/mam.h>
#include <iota/merkle.h>

iota_ctrits_t *string_to_ctrits_trits(const std::string &str) {
  auto trytes = iota_ctrits_ctrits_from_trytes(str.data(), str.length());
  auto trits = iota_ctrits_convert(trytes, IOTA_ENCODING_TRIT);
  iota_ctrits_drop(trytes);
  return trits;
}

TEST(MAMTest, MAMCreateAndParseTest) {
  const std::string SEED = "ABCDEFGHIJKLMNOPQRSTUVWXYZ9ABCDEFGHIJKLMNOPQRSTUVWX"
                           "YZ9ABCDEFGHIJKLMNOPQRSTUVWXYZ9";
  const std::string MESSAGE =
      "ABCDEFGHIJKLMNOPQRSTUVWXYZ9ABCDEFGHIJKLMNOPQRSTUVWXYZ9";
  const std::string SIDE_KEY =
      "EFGHIJKLMNOPQRSTUVWXYZ9ABCDEFGHIJKLMNOPQRSTUVWXYZ9ABCDABCD";

  const uint8_t SECURITY = 1;
  const size_t START = 1;
  const size_t COUNT = 4;
  const auto NEXT_START = START + COUNT;
  const auto NEXT_COUNT = 2;
  const auto INDEX = 1;

  iota_ctrits_t *SEED_ctrits = string_to_ctrits_trits(SEED);
  iota_ctrits_t *MESSAGE_ctrits = string_to_ctrits_trits(MESSAGE);
  iota_ctrits_t *SIDE_KEY_ctrits = string_to_ctrits_trits(SIDE_KEY);

  auto root_merkle = iota_merkle_create(SEED_ctrits, START, COUNT, SECURITY);
  auto next_root_merkle =
      iota_merkle_create(SEED_ctrits, NEXT_START, NEXT_COUNT, SECURITY);

  auto root_branch = iota_merkle_branch(root_merkle, INDEX);
  auto root_siblings = iota_merkle_siblings(root_branch);

  auto next_root_branch = iota_merkle_branch(next_root_merkle, INDEX);
  auto next_root_siblings = iota_merkle_siblings(next_root_branch);

  auto root = iota_merkle_slice(root_merkle);
  auto next_root = iota_merkle_slice(next_root_merkle);

  auto masked_payload =
      iota_mam_create(SEED_ctrits, MESSAGE_ctrits, SIDE_KEY_ctrits, root,
                      root_siblings, next_root, START, INDEX, SECURITY);

  auto parse_result = iota_mam_parse(masked_payload, SIDE_KEY_ctrits, root);

  auto unmasked_payload_trytes =
      iota_ctrits_convert(parse_result->message, IOTA_ENCODING_TRYTE);
  auto unmasked_next_root_trytes =
      iota_ctrits_convert(parse_result->next_root, IOTA_ENCODING_TRYTE);

  iota_ctrits_drop(parse_result->message);
  iota_ctrits_drop(parse_result->next_root);

  {
    auto payload = std::string((char *)unmasked_payload_trytes->data,
                               unmasked_payload_trytes->length);

    ASSERT_EQ(payload, MESSAGE);
  }

  {
    auto unmasked_next_root =
        std::string((char *)unmasked_next_root_trytes->data,
                    unmasked_next_root_trytes->length);

    auto next_root_trytes = iota_ctrits_convert(next_root, IOTA_ENCODING_TRYTE);
    auto next_root_str =
        std::string((char *)next_root_trytes->data, next_root_trytes->length);

    ASSERT_EQ(unmasked_next_root, next_root_str);
    iota_ctrits_drop(next_root_trytes);
  }

  iota_ctrits_drop(unmasked_payload_trytes);
  iota_ctrits_drop(unmasked_next_root_trytes);

  iota_merkle_branch_drop(root_branch);
  iota_merkle_branch_drop(next_root_branch);

  iota_merkle_drop(root_merkle);
  iota_merkle_drop(next_root_merkle);
}
