#![feature(link_args)]

extern crate iota_bindings_rlib as bindings;
pub use bindings::*;

#[link_args = "-s EXPORTED_FUNCTIONS=[\
'_curl_pair_new','_curl_pair_absorb','_curl_pair_squeeze','_curl_pair_reset','_curl_pair_delete'\
,'_curl_simple_new','_curl_simple_absorb','_curl_simple_squeeze','_curl_simple_reset','_curl_simple_delete'\
,'_subseed','_key','_digest_key','_address','_signature','_digest_bundle_signature'\
,'_mam_create','_mam_parse'\
,'_merkle_key','_merkle_siblings','_merkle_root'\
]"]
extern "C" {}


fn main() {}
