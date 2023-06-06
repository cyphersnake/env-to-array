# Env To Array

This crate allows you to create constant arrays from encoded string literals as well as take them directly from the env variable.

## Install

```bash
cargo add env-to-array
```

## Usage

Get from env variable string, decode it from format and put integer array as result

### base58

The `bs58` feature should be enabled (enabled by default)

```rust
/// Get array from env variable encoded by base58
const BS58_ENV_ID: [u8; 17] = env_to_array::bs58_env_to_array!("_ENV_TO_ARRAY_BS58");

/// Get array from constant encoded by base58
const BS58_ID: [u8; 17] = env_to_array::bs58_to_array!("dwVAPpaonY26V6JH17ToUQ");
```

### base64

The `bs64` feature should be enabled (enabled by default)

```rust
/// Get array from env variable encoded by base64
const BS64_ENV_ID: [u8; 32] = env_to_array::bs64_env_to_array!("_ENV_TO_ARRAY_BS64");

/// Get array from constant encoded by base64
const BS64_ID: [u8; 32] = env_to_array::bs64_to_array!("W7MmhbfqLQc4LbN0TUPfiflxSO6uVZ7E0NH+76LueJ0=");
```

### hex

The `hex` feature should be enabled (enabled by default)

```rust
/// Get array from env variable encoded by hex
const HEX_ENV_ID: [u64; 64] = env_to_array::hex_env_to_array!("_ENV_TO_ARRAY_HEX");

/// Get array from constant encoded by hex
const HEX_ID: [u64; 64] = env_to_array::hex_to_array!("5bb32685b7e5bb32685b7ea2d07382db3744d43df89f97148eeae559ec4d0d1feefa2ee789da2d07382db3744d43df89f97148eeae559ec4d0d1feefa2ee789d");
```
