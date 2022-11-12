# Env To Array

Get from env variable string, decode it from format and put integer array as result

```rust
const BS58_ID: [u8; 32] = env_to_array::bs58_env_to_array!("_ENV_TO_ARRAY_BS58");
const BS64_ID: [i8; 12] = env_to_array::bs64_env_to_array!("_ENV_TO_ARRAY_BS64");
const HEX_ID: [u64; 128] = env_to_array::hex_env_to_array!("_ENV_TO_ARRAY_HEX");
```
