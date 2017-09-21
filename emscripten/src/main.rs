#![feature(link_args)]

extern crate iota_bindings_rlib as bindings;
pub use bindings::*;

#[link_args = "-s EXPORTED_FUNCTIONS=[\
'_iota_ctrits_drop','_iota_ctrits_convert',\
'_iota_curl_bctrit_new','_iota_curl_bctrit_absorb','_iota_curl_bctrit_squeeze','_iota_curl_bctrit_reset','_iota_curl_bctrit_delete'\
,'_iota_curl_trit_new','_iota_curl_trit_absorb','_iota_curl_trit_squeeze','_iota_curl_trit_reset','_iota_curl_trit_delete'\
,'_iota_kerl_trit_new','_iota_kerl_trit_absorb','_iota_kerl_trit_squeeze','_iota_kerl_trit_reset','_iota_kerl_trit_delete'\
,'_iota_merkle_create','_iota_merkle_drop','_iota_merkle_size','_iota_merkle_depth','_iota_merkle_count','_iota_merkle_slice','_iota_merkle_branch','_iota_merkle_branch_len','_iota_merkle_branch_drop','_iota_merkle_siblings','_iota_merkle_root'\
,'_iota_sign_checksum','_iota_sign_checksum_validate'\
'_iota_sign_iss_subseed','_iota_sing_iss_key','_iota_sign_iss_digest_key','_iota_sign_iss_address','_iota_sign_iss_signature','_iota_sign_iss_subseed_to_signature','_iota_sign_iss_digest_bundle_signature'\
,'_mam_create','_mam_parse','_mam_id','_mam_key'\
]"]
extern "C" {}


fn main() {}
