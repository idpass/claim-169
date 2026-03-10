# API Reference

Complete API documentation for the claim169 Python SDK.

## Module Functions

### version()

Get the library version.

```python
def version() -> str
```

**Returns:** Version string in semver format (e.g., "0.3.0")

**Example:**
```python
import claim169
print(claim169.version())  # "0.3.0"
```

### generate_nonce()

Generate a random 12-byte nonce for AES-GCM encryption.

```python
def generate_nonce() -> bytes
```

**Returns:** 12-byte nonce suitable for AES-GCM IV

**Example:**
```python
nonce = claim169.generate_nonce()  # 12 bytes
```

### inspect()

Extract credential metadata without full decoding or verification.

Extracts metadata (issuer, key ID, algorithm, expiration) from a QR code without
verifying the signature. For encrypted credentials, only COSE-level headers are
available; CWT-level fields (`issuer`, `subject`, `expires_at`) will be `None`.

Useful for multi-issuer or key-rotation scenarios where you need to determine
which verification key to use before decoding.

```python
def inspect(qr_text: str) -> InspectResult
```

**Parameters:**

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `qr_text` | `str` | required | Base45-encoded QR content |

**Returns:** `InspectResult`

**Raises:** `Base45DecodeError`, `DecompressError`, `CoseParseError`

**Example:**
```python
meta = claim169.inspect(qr_text)
print(meta.issuer, meta.algorithm, meta.key_id)
```

---

## Decode Functions

### decode()

Unified decode function with flexible verification options.

```python
def decode(
    qr_text: str,
    skip_biometrics: bool = False,
    max_decompressed_bytes: int = 65536,
    validate_timestamps: bool = True,
    clock_skew_tolerance_seconds: int = 0,
    verify_with_ed25519: bytes | None = None,
    verify_with_ecdsa_p256: bytes | None = None,
    verify_with_ed25519_pem: str | None = None,
    verify_with_ecdsa_p256_pem: str | None = None,
    allow_unverified: bool = False,
) -> DecodeResult
```

By default, requires signature verification via one of:

- `verify_with_ed25519`: 32-byte raw public key
- `verify_with_ecdsa_p256`: 33 or 65-byte SEC1 public key
- `verify_with_ed25519_pem`: PEM-encoded public key (SPKI format)
- `verify_with_ecdsa_p256_pem`: PEM-encoded public key (SPKI format)

To decode without verification (testing only), set `allow_unverified=True`.

**Parameters:**

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `qr_text` | `str` | required | Base45-encoded QR content |
| `skip_biometrics` | `bool` | `False` | Skip parsing biometric data |
| `max_decompressed_bytes` | `int` | `65536` | Maximum decompressed size limit |
| `validate_timestamps` | `bool` | `True` | Validate exp/nbf timestamps |
| `clock_skew_tolerance_seconds` | `int` | `0` | Tolerance for clock differences |
| `verify_with_ed25519` | `bytes \| None` | `None` | 32-byte Ed25519 public key |
| `verify_with_ecdsa_p256` | `bytes \| None` | `None` | SEC1 encoded P-256 public key |
| `verify_with_ed25519_pem` | `str \| None` | `None` | PEM-encoded Ed25519 public key |
| `verify_with_ecdsa_p256_pem` | `str \| None` | `None` | PEM-encoded P-256 public key |
| `allow_unverified` | `bool` | `False` | Skip signature verification (INSECURE) |

**Returns:** `DecodeResult`

---

### decode_unverified()

Decode without signature verification. **INSECURE - testing only.**

```python
def decode_unverified(
    qr_text: str,
    skip_biometrics: bool = False,
    max_decompressed_bytes: int = 65536,
    validate_timestamps: bool = True,
    clock_skew_tolerance_seconds: int = 0
) -> DecodeResult
```

**Parameters:**

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `qr_text` | `str` | required | Base45-encoded QR content |
| `skip_biometrics` | `bool` | `False` | Skip parsing biometric data |
| `max_decompressed_bytes` | `int` | `65536` | Maximum decompressed size limit |
| `validate_timestamps` | `bool` | `True` | Validate exp/nbf timestamps |
| `clock_skew_tolerance_seconds` | `int` | `0` | Tolerance for clock differences |

**Returns:** `DecodeResult`

**Raises:** `Base45DecodeError`, `DecompressError`, `CoseParseError`, `CwtParseError`, `Claim169NotFoundError`, `Claim169Exception`

---

### decode_with_ed25519()

Decode with Ed25519 signature verification.

```python
def decode_with_ed25519(
    qr_text: str,
    public_key: bytes,
    skip_biometrics: bool = False,
    max_decompressed_bytes: int = 65536,
    validate_timestamps: bool = True,
    clock_skew_tolerance_seconds: int = 0
) -> DecodeResult
```

**Parameters:**

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `qr_text` | `str` | required | Base45-encoded QR content |
| `public_key` | `bytes` | required | 32-byte Ed25519 public key |
| `skip_biometrics` | `bool` | `False` | Skip parsing biometric data |
| `max_decompressed_bytes` | `int` | `65536` | Maximum decompressed size limit |
| `validate_timestamps` | `bool` | `True` | Validate exp/nbf timestamps |
| `clock_skew_tolerance_seconds` | `int` | `0` | Tolerance for clock differences |

**Returns:** `DecodeResult`

**Raises:** `SignatureError`, `Base45DecodeError`, `DecompressError`, `CoseParseError`, `CwtParseError`, `Claim169NotFoundError`

---

### decode_with_ecdsa_p256()

Decode with ECDSA P-256 signature verification.

```python
def decode_with_ecdsa_p256(
    qr_text: str,
    public_key: bytes,
    skip_biometrics: bool = False,
    max_decompressed_bytes: int = 65536,
    validate_timestamps: bool = True,
    clock_skew_tolerance_seconds: int = 0
) -> DecodeResult
```

**Parameters:**

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `qr_text` | `str` | required | Base45-encoded QR content |
| `public_key` | `bytes` | required | SEC1 encoded P-256 public key (33 or 65 bytes) |
| `skip_biometrics` | `bool` | `False` | Skip parsing biometric data |
| `max_decompressed_bytes` | `int` | `65536` | Maximum decompressed size limit |
| `validate_timestamps` | `bool` | `True` | Validate exp/nbf timestamps |
| `clock_skew_tolerance_seconds` | `int` | `0` | Tolerance for clock differences |

**Returns:** `DecodeResult`

**Raises:** `SignatureError`, `Base45DecodeError`, `DecompressError`, `CoseParseError`, `CwtParseError`, `Claim169NotFoundError`

---

### decode_with_ed25519_pem()

Decode with Ed25519 signature verification using a PEM-encoded public key.

```python
def decode_with_ed25519_pem(
    qr_text: str,
    pem: str,
    skip_biometrics: bool = False,
    max_decompressed_bytes: int = 65536,
    validate_timestamps: bool = True,
    clock_skew_tolerance_seconds: int = 0
) -> DecodeResult
```

**Parameters:**

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `qr_text` | `str` | required | Base45-encoded QR content |
| `pem` | `str` | required | PEM-encoded Ed25519 public key (SPKI format) |
| `skip_biometrics` | `bool` | `False` | Skip parsing biometric data |
| `max_decompressed_bytes` | `int` | `65536` | Maximum decompressed size limit |
| `validate_timestamps` | `bool` | `True` | Validate exp/nbf timestamps |
| `clock_skew_tolerance_seconds` | `int` | `0` | Tolerance for clock differences |

**Returns:** `DecodeResult`

**Raises:** `SignatureError`, `Base45DecodeError`, `DecompressError`, `CoseParseError`, `CwtParseError`, `Claim169NotFoundError`

---

### decode_with_ecdsa_p256_pem()

Decode with ECDSA P-256 signature verification using a PEM-encoded public key.

```python
def decode_with_ecdsa_p256_pem(
    qr_text: str,
    pem: str,
    skip_biometrics: bool = False,
    max_decompressed_bytes: int = 65536,
    validate_timestamps: bool = True,
    clock_skew_tolerance_seconds: int = 0
) -> DecodeResult
```

**Parameters:**

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `qr_text` | `str` | required | Base45-encoded QR content |
| `pem` | `str` | required | PEM-encoded ECDSA P-256 public key (SPKI format) |
| `skip_biometrics` | `bool` | `False` | Skip parsing biometric data |
| `max_decompressed_bytes` | `int` | `65536` | Maximum decompressed size limit |
| `validate_timestamps` | `bool` | `True` | Validate exp/nbf timestamps |
| `clock_skew_tolerance_seconds` | `int` | `0` | Tolerance for clock differences |

**Returns:** `DecodeResult`

**Raises:** `SignatureError`, `Base45DecodeError`, `DecompressError`, `CoseParseError`, `CwtParseError`, `Claim169NotFoundError`

---

### decode_with_verifier()

Decode with a custom verifier callback for HSM/KMS integration.

```python
def decode_with_verifier(
    qr_text: str,
    verifier: Callable[[str, bytes | None, bytes, bytes], None],
    skip_biometrics: bool = False,
    max_decompressed_bytes: int = 65536,
    validate_timestamps: bool = True,
    clock_skew_tolerance_seconds: int = 0
) -> DecodeResult
```

**Parameters:**

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `qr_text` | `str` | required | Base45-encoded QR content |
| `verifier` | `Callable` | required | Callback `(algorithm, key_id, data, signature) -> None` |
| `skip_biometrics` | `bool` | `False` | Skip parsing biometric data |
| `max_decompressed_bytes` | `int` | `65536` | Maximum decompressed size limit |
| `validate_timestamps` | `bool` | `True` | Validate exp/nbf timestamps |
| `clock_skew_tolerance_seconds` | `int` | `0` | Tolerance for clock differences |

The verifier callback receives:
- `algorithm`: Algorithm name ("EdDSA" or "ES256")
- `key_id`: Optional key identifier from COSE header
- `data`: Data that was signed
- `signature`: Signature to verify

The callback should raise an exception if verification fails.

**Returns:** `DecodeResult`

**Raises:** `SignatureError`, `Claim169Exception`

---

### decode_encrypted_aes()

Decode an AES-GCM encrypted credential (auto-detects key size).

Either `verifier` or `allow_unverified=True` must be provided; passing neither raises `ValueError`.

```python
def decode_encrypted_aes(
    qr_text: str,
    key: bytes,
    verifier: Callable | None = None,
    allow_unverified: bool = False
) -> DecodeResult
```

**Parameters:**

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `qr_text` | `str` | required | Base45-encoded QR content |
| `key` | `bytes` | required | AES key (16 or 32 bytes) |
| `verifier` | `Callable` | `None` | Optional verifier callback |
| `allow_unverified` | `bool` | `False` | Skip signature verification (INSECURE) |

**Returns:** `DecodeResult`

**Raises:** `DecryptionError`, `SignatureError`, `ValueError`, `Claim169Exception`

---

### decode_encrypted_aes256()

Decode an AES-256-GCM encrypted credential (validates 32-byte key).

Either `verifier` or `allow_unverified=True` must be provided; passing neither raises `ValueError`.

```python
def decode_encrypted_aes256(
    qr_text: str,
    key: bytes,
    verifier: Callable | None = None,
    allow_unverified: bool = False
) -> DecodeResult
```

**Parameters:**

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `qr_text` | `str` | required | Base45-encoded QR content |
| `key` | `bytes` | required | 32-byte AES-256 key |
| `verifier` | `Callable` | `None` | Optional verifier callback |
| `allow_unverified` | `bool` | `False` | Skip signature verification (INSECURE) |

**Returns:** `DecodeResult`

**Raises:** `DecryptionError`, `ValueError` (if key not 32 bytes, or neither verifier nor allow_unverified provided)

---

### decode_encrypted_aes128()

Decode an AES-128-GCM encrypted credential (validates 16-byte key).

Either `verifier` or `allow_unverified=True` must be provided; passing neither raises `ValueError`.

```python
def decode_encrypted_aes128(
    qr_text: str,
    key: bytes,
    verifier: Callable | None = None,
    allow_unverified: bool = False
) -> DecodeResult
```

**Parameters:**

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `qr_text` | `str` | required | Base45-encoded QR content |
| `key` | `bytes` | required | 16-byte AES-128 key |
| `verifier` | `Callable` | `None` | Optional verifier callback |
| `allow_unverified` | `bool` | `False` | Skip signature verification (INSECURE) |

**Returns:** `DecodeResult`

**Raises:** `DecryptionError`, `ValueError` (if key not 16 bytes, or neither verifier nor allow_unverified provided)

---

### decode_with_decryptor()

Decode with a custom decryptor callback for HSM/KMS integration.

Either `verifier` or `allow_unverified=True` must be provided; passing neither raises `ValueError`.

```python
def decode_with_decryptor(
    qr_text: str,
    decryptor: Callable[[str, bytes | None, bytes, bytes, bytes], bytes],
    verifier: Callable | None = None,
    allow_unverified: bool = False
) -> DecodeResult
```

**Parameters:**

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `qr_text` | `str` | required | Base45-encoded QR content |
| `decryptor` | `Callable` | required | Callback `(algorithm, key_id, nonce, aad, ciphertext) -> bytes` |
| `verifier` | `Callable` | `None` | Optional verifier callback |
| `allow_unverified` | `bool` | `False` | Skip signature verification (INSECURE) |

The decryptor callback receives:
- `algorithm`: Algorithm name ("A256GCM" or "A128GCM")
- `key_id`: Optional key identifier from COSE header
- `nonce`: 12-byte nonce
- `aad`: Additional authenticated data
- `ciphertext`: Encrypted data with auth tag

The callback should return decrypted plaintext bytes.

**Returns:** `DecodeResult`

**Raises:** `DecryptionError`, `Claim169Exception`

---

## Encode Functions

### encode()

Unified encode function with flexible signing and encryption options.

```python
def encode(
    claim169_data: Claim169Input,
    cwt_meta: CwtMetaInput,
    sign_with_ed25519: bytes | None = None,
    sign_with_ecdsa_p256: bytes | None = None,
    encrypt_with_aes256: bytes | None = None,
    encrypt_with_aes128: bytes | None = None,
    allow_unsigned: bool = False,
    skip_biometrics: bool = False,
) -> str
```

By default, requires a signing key via one of:

- `sign_with_ed25519`: 32-byte private key
- `sign_with_ecdsa_p256`: 32-byte private key

Optionally encrypt with:

- `encrypt_with_aes256`: 32-byte AES key
- `encrypt_with_aes128`: 16-byte AES key

To encode without signing (testing only), set `allow_unsigned=True`.

**Parameters:**

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `claim169_data` | `Claim169Input` | required | Identity data |
| `cwt_meta` | `CwtMetaInput` | required | Token metadata |
| `sign_with_ed25519` | `bytes \| None` | `None` | 32-byte Ed25519 private key |
| `sign_with_ecdsa_p256` | `bytes \| None` | `None` | 32-byte ECDSA P-256 private key |
| `encrypt_with_aes256` | `bytes \| None` | `None` | 32-byte AES-256 key |
| `encrypt_with_aes128` | `bytes \| None` | `None` | 16-byte AES-128 key |
| `allow_unsigned` | `bool` | `False` | Encode without signing (INSECURE) |
| `skip_biometrics` | `bool` | `False` | Exclude biometric data |

**Returns:** Base45-encoded string

---

### encode_with_ed25519()

Encode with Ed25519 signature.

```python
def encode_with_ed25519(
    claim169: Claim169Input,
    cwt_meta: CwtMetaInput,
    private_key: bytes,
    skip_biometrics: bool = False
) -> str
```

**Parameters:**

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `claim169` | `Claim169Input` | required | Identity data |
| `cwt_meta` | `CwtMetaInput` | required | Token metadata |
| `private_key` | `bytes` | required | 32-byte Ed25519 private key |
| `skip_biometrics` | `bool` | `False` | Exclude biometric data |

**Returns:** Base45-encoded string

**Raises:** `ValueError`, `Claim169Exception`

---

### encode_with_ecdsa_p256()

Encode with ECDSA P-256 signature.

```python
def encode_with_ecdsa_p256(
    claim169: Claim169Input,
    cwt_meta: CwtMetaInput,
    private_key: bytes,
    skip_biometrics: bool = False
) -> str
```

**Parameters:**

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `claim169` | `Claim169Input` | required | Identity data |
| `cwt_meta` | `CwtMetaInput` | required | Token metadata |
| `private_key` | `bytes` | required | 32-byte ECDSA P-256 private key |
| `skip_biometrics` | `bool` | `False` | Exclude biometric data |

**Returns:** Base45-encoded string

**Raises:** `ValueError`, `Claim169Exception`

---

### encode_signed_encrypted()

Encode with Ed25519 signature and AES-256-GCM encryption.

```python
def encode_signed_encrypted(
    claim169: Claim169Input,
    cwt_meta: CwtMetaInput,
    sign_key: bytes,
    encrypt_key: bytes,
    skip_biometrics: bool = False
) -> str
```

**Parameters:**

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `claim169` | `Claim169Input` | required | Identity data |
| `cwt_meta` | `CwtMetaInput` | required | Token metadata |
| `sign_key` | `bytes` | required | 32-byte Ed25519 private key |
| `encrypt_key` | `bytes` | required | 32-byte AES-256 key |
| `skip_biometrics` | `bool` | `False` | Exclude biometric data |

**Returns:** Base45-encoded string

**Raises:** `ValueError`, `Claim169Exception`

---

### encode_signed_encrypted_aes128()

Encode with Ed25519 signature and AES-128-GCM encryption.

```python
def encode_signed_encrypted_aes128(
    claim169: Claim169Input,
    cwt_meta: CwtMetaInput,
    sign_key: bytes,
    encrypt_key: bytes,
    skip_biometrics: bool = False
) -> str
```

**Parameters:**

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `claim169` | `Claim169Input` | required | Identity data |
| `cwt_meta` | `CwtMetaInput` | required | Token metadata |
| `sign_key` | `bytes` | required | 32-byte Ed25519 private key |
| `encrypt_key` | `bytes` | required | 16-byte AES-128 key |
| `skip_biometrics` | `bool` | `False` | Exclude biometric data |

**Returns:** Base45-encoded string

**Raises:** `ValueError`, `Claim169Exception`

---

### encode_unsigned()

Encode without signature. **INSECURE - testing only.**

```python
def encode_unsigned(
    claim169: Claim169Input,
    cwt_meta: CwtMetaInput,
    skip_biometrics: bool = False
) -> str
```

**Parameters:**

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `claim169` | `Claim169Input` | required | Identity data |
| `cwt_meta` | `CwtMetaInput` | required | Token metadata |
| `skip_biometrics` | `bool` | `False` | Exclude biometric data |

**Returns:** Base45-encoded string

**Raises:** `Claim169Exception`

---

### encode_with_signer()

Encode with a custom signer callback for HSM/KMS integration.

```python
def encode_with_signer(
    claim169: Claim169Input,
    cwt_meta: CwtMetaInput,
    signer: Callable[[str, bytes | None, bytes], bytes],
    algorithm: str,
    key_id: bytes | None = None,
    skip_biometrics: bool = False
) -> str
```

**Parameters:**

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `claim169` | `Claim169Input` | required | Identity data |
| `cwt_meta` | `CwtMetaInput` | required | Token metadata |
| `signer` | `Callable` | required | Callback `(algorithm, key_id, data) -> signature` |
| `algorithm` | `str` | required | "EdDSA" or "ES256" |
| `key_id` | `bytes` | `None` | Optional key identifier |
| `skip_biometrics` | `bool` | `False` | Exclude biometric data |

**Returns:** Base45-encoded string

**Raises:** `ValueError`, `Claim169Exception`

---

### encode_with_signer_and_encryptor()

Encode with custom signer and encryptor callbacks.

```python
def encode_with_signer_and_encryptor(
    claim169: Claim169Input,
    cwt_meta: CwtMetaInput,
    signer: Callable[[str, bytes | None, bytes], bytes],
    sign_algorithm: str,
    encryptor: Callable[[str, bytes | None, bytes, bytes, bytes], bytes],
    encrypt_algorithm: str,
    key_id: bytes | None = None,
    skip_biometrics: bool = False
) -> str
```

**Parameters:**

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `claim169` | `Claim169Input` | required | Identity data |
| `cwt_meta` | `CwtMetaInput` | required | Token metadata |
| `signer` | `Callable` | required | Signer callback |
| `sign_algorithm` | `str` | required | "EdDSA" or "ES256" |
| `encryptor` | `Callable` | required | Encryptor callback |
| `encrypt_algorithm` | `str` | required | "A256GCM" or "A128GCM" |
| `key_id` | `bytes` | `None` | Optional key identifier |
| `skip_biometrics` | `bool` | `False` | Exclude biometric data |

**Returns:** Base45-encoded string

**Raises:** `ValueError`, `Claim169Exception`

---

### encode_with_encryptor()

Encode with software signing and custom encryptor callback.

```python
def encode_with_encryptor(
    claim169: Claim169Input,
    cwt_meta: CwtMetaInput,
    sign_key: bytes,
    encryptor: Callable[[str, bytes | None, bytes, bytes, bytes], bytes],
    encrypt_algorithm: str,
    skip_biometrics: bool = False
) -> str
```

**Parameters:**

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `claim169` | `Claim169Input` | required | Identity data |
| `cwt_meta` | `CwtMetaInput` | required | Token metadata |
| `sign_key` | `bytes` | required | 32-byte Ed25519 private key |
| `encryptor` | `Callable` | required | Encryptor callback |
| `encrypt_algorithm` | `str` | required | "A256GCM" or "A128GCM" |
| `skip_biometrics` | `bool` | `False` | Exclude biometric data |

**Returns:** Base45-encoded string

**Raises:** `ValueError`, `Claim169Exception`

---

## Classes

### Claim169Input

Input class for encoding identity data. All fields can be passed as constructor
keyword arguments or set as attributes after construction.

```python
class Claim169Input:
    def __init__(
        self,
        id: str | None = None,
        version: str | None = None,
        language: str | None = None,
        full_name: str | None = None,
        first_name: str | None = None,
        middle_name: str | None = None,
        last_name: str | None = None,
        date_of_birth: str | None = None,
        gender: int | None = None,
        address: str | None = None,
        email: str | None = None,
        phone: str | None = None,
        nationality: str | None = None,
        marital_status: int | None = None,
        guardian: str | None = None,
        photo: bytes | None = None,
        photo_format: int | None = None,
        secondary_full_name: str | None = None,
        secondary_language: str | None = None,
        location_code: str | None = None,
        legal_status: str | None = None,
        country_of_issuance: str | None = None,
        right_thumb: list[Biometric] | None = None,
        right_pointer_finger: list[Biometric] | None = None,
        right_middle_finger: list[Biometric] | None = None,
        right_ring_finger: list[Biometric] | None = None,
        right_little_finger: list[Biometric] | None = None,
        left_thumb: list[Biometric] | None = None,
        left_pointer_finger: list[Biometric] | None = None,
        left_middle_finger: list[Biometric] | None = None,
        left_ring_finger: list[Biometric] | None = None,
        left_little_finger: list[Biometric] | None = None,
        right_iris: list[Biometric] | None = None,
        left_iris: list[Biometric] | None = None,
        face: list[Biometric] | None = None,
        right_palm: list[Biometric] | None = None,
        left_palm: list[Biometric] | None = None,
        voice: list[Biometric] | None = None,
    ) -> None
```

**Demographic Attributes:**

| Name | Type | Description |
|------|------|-------------|
| `id` | `str \| None` | Unique identifier |
| `version` | `str \| None` | Credential version |
| `language` | `str \| None` | Primary language code |
| `full_name` | `str \| None` | Full name |
| `first_name` | `str \| None` | First/given name |
| `middle_name` | `str \| None` | Middle name |
| `last_name` | `str \| None` | Last/family name |
| `date_of_birth` | `str \| None` | Date of birth (YYYY-MM-DD) |
| `gender` | `int \| None` | 1=Male, 2=Female, 3=Other |
| `address` | `str \| None` | Full address |
| `email` | `str \| None` | Email address |
| `phone` | `str \| None` | Phone number |
| `nationality` | `str \| None` | Nationality code |
| `marital_status` | `int \| None` | 1=Unmarried, 2=Married, 3=Divorced |
| `guardian` | `str \| None` | Guardian name |
| `photo` | `bytes \| None` | Photo data |
| `photo_format` | `int \| None` | 1=JPEG, 2=JPEG2000, 3=AVIF, 4=WebP |
| `secondary_full_name` | `str \| None` | Name in secondary language |
| `secondary_language` | `str \| None` | Secondary language code |
| `location_code` | `str \| None` | Location code |
| `legal_status` | `str \| None` | Legal status |
| `country_of_issuance` | `str \| None` | Issuing country code |

**Biometric Attributes:**

| Name | Type | Description |
|------|------|-------------|
| `right_thumb` | `list[Biometric] \| None` | Right thumb biometrics |
| `right_pointer_finger` | `list[Biometric] \| None` | Right pointer finger biometrics |
| `right_middle_finger` | `list[Biometric] \| None` | Right middle finger biometrics |
| `right_ring_finger` | `list[Biometric] \| None` | Right ring finger biometrics |
| `right_little_finger` | `list[Biometric] \| None` | Right little finger biometrics |
| `left_thumb` | `list[Biometric] \| None` | Left thumb biometrics |
| `left_pointer_finger` | `list[Biometric] \| None` | Left pointer finger biometrics |
| `left_middle_finger` | `list[Biometric] \| None` | Left middle finger biometrics |
| `left_ring_finger` | `list[Biometric] \| None` | Left ring finger biometrics |
| `left_little_finger` | `list[Biometric] \| None` | Left little finger biometrics |
| `right_iris` | `list[Biometric] \| None` | Right iris biometrics |
| `left_iris` | `list[Biometric] \| None` | Left iris biometrics |
| `face` | `list[Biometric] \| None` | Face biometrics |
| `right_palm` | `list[Biometric] \| None` | Right palm biometrics |
| `left_palm` | `list[Biometric] \| None` | Left palm biometrics |
| `voice` | `list[Biometric] \| None` | Voice biometrics |

---

### CwtMetaInput

Input class for CWT token metadata.

```python
class CwtMetaInput:
    def __init__(
        self,
        issuer: str | None = None,
        expires_at: int | None = None
    ) -> None
```

**Attributes:**

| Name | Type | Description |
|------|------|-------------|
| `issuer` | `str \| None` | Credential issuer |
| `subject` | `str \| None` | Subject identifier |
| `expires_at` | `int \| None` | Expiration timestamp (Unix epoch) |
| `not_before` | `int \| None` | Not valid before timestamp |
| `issued_at` | `int \| None` | Issuance timestamp |

---

### DecodeResult

Result of decoding a credential.

```python
class DecodeResult:
    claim169: Claim169
    cwt_meta: CwtMeta
    verification_status: str
    x509_headers: X509Headers
    detected_compression: str

    def is_verified(self) -> bool
```

**Attributes:**

| Name | Type | Description |
|------|------|-------------|
| `claim169` | `Claim169` | Decoded identity data |
| `cwt_meta` | `CwtMeta` | CWT metadata |
| `verification_status` | `str` | "verified", "skipped", etc. |
| `x509_headers` | `X509Headers` | X.509 certificate headers from COSE structure |
| `detected_compression` | `str` | Detected compression format ("zlib", "brotli", or "none") |

**Methods:**

- `is_verified() -> bool`: Returns `True` if signature was verified

---

### InspectResult

Metadata extracted from a credential without full verification or decoding.

```python
class InspectResult:
    issuer: str | None
    subject: str | None
    key_id: bytes | None
    algorithm: str | None
    x509_headers: X509Headers
    expires_at: int | None
    cose_type: str
```

**Attributes:**

| Name | Type | Description |
|------|------|-------------|
| `issuer` | `str \| None` | Token issuer from CWT claims (None for encrypted credentials) |
| `subject` | `str \| None` | Token subject from CWT claims (None for encrypted credentials) |
| `key_id` | `bytes \| None` | Key ID from the COSE header |
| `algorithm` | `str \| None` | COSE algorithm name (e.g., "EdDSA", "ES256") |
| `x509_headers` | `X509Headers` | X.509 certificate headers from COSE structure |
| `expires_at` | `int \| None` | Expiration timestamp (None for encrypted credentials) |
| `cose_type` | `str` | COSE structure type: "Sign1" or "Encrypt0" |

---

### Claim169

Decoded identity claim.

```python
class Claim169:
    # All fields are read-only
    id: str | None
    version: str | None
    language: str | None
    full_name: str | None
    first_name: str | None
    middle_name: str | None
    last_name: str | None
    date_of_birth: str | None
    gender: int | None
    address: str | None
    email: str | None
    phone: str | None
    nationality: str | None
    marital_status: int | None
    guardian: str | None
    photo: bytes | None
    photo_format: int | None
    best_quality_fingers: bytes | None
    secondary_full_name: str | None
    secondary_language: str | None
    location_code: str | None
    legal_status: str | None
    country_of_issuance: str | None

    # Biometrics
    right_thumb: list[Biometric] | None
    right_pointer_finger: list[Biometric] | None
    right_middle_finger: list[Biometric] | None
    right_ring_finger: list[Biometric] | None
    right_little_finger: list[Biometric] | None
    left_thumb: list[Biometric] | None
    left_pointer_finger: list[Biometric] | None
    left_middle_finger: list[Biometric] | None
    left_ring_finger: list[Biometric] | None
    left_little_finger: list[Biometric] | None
    right_iris: list[Biometric] | None
    left_iris: list[Biometric] | None
    face: list[Biometric] | None
    right_palm: list[Biometric] | None
    left_palm: list[Biometric] | None
    voice: list[Biometric] | None

    def has_biometrics(self) -> bool
    def to_dict(self) -> dict
```

**Methods:**

- `has_biometrics() -> bool`: Returns `True` if any biometric data present
- `to_dict() -> dict`: Convert to Python dictionary

---

### CwtMeta

CWT metadata from decoded credential.

```python
class CwtMeta:
    issuer: str | None
    subject: str | None
    expires_at: int | None
    not_before: int | None
    issued_at: int | None

    def is_valid_now(self) -> bool
    def is_expired(self) -> bool
```

**Methods:**

- `is_valid_now() -> bool`: Returns `True` if token is currently valid
- `is_expired() -> bool`: Returns `True` if token has expired

---

### Biometric

Biometric data container.

```python
class Biometric:
    data: bytes
    format: int | None
    sub_format: int | None
    issuer: str | None
```

**Attributes:**

| Name | Type | Description |
|------|------|-------------|
| `data` | `bytes` | Raw biometric data |
| `format` | `int \| None` | Format code |
| `sub_format` | `int \| None` | Sub-format code |
| `issuer` | `str \| None` | Biometric issuer |

---

### CertificateHash

X.509 certificate hash (COSE_CertHash).

```python
class CertificateHash:
    algorithm: str
    hash_value: bytes
```

**Attributes:**

| Name | Type | Description |
|------|------|-------------|
| `algorithm` | `str` | Hash algorithm identifier (numeric COSE algorithm ID as string, or named algorithm) |
| `hash_value` | `bytes` | Hash value bytes |

---

### X509Headers

X.509 headers extracted from COSE protected/unprotected headers.

```python
class X509Headers:
    x5bag: list[bytes] | None
    x5chain: list[bytes] | None
    x5t: CertificateHash | None
    x5u: str | None

    def has_any(self) -> bool
```

**Attributes:**

| Name | Type | Description |
|------|------|-------------|
| `x5bag` | `list[bytes] \| None` | x5bag (label 32): Unordered bag of X.509 certificates (DER-encoded) |
| `x5chain` | `list[bytes] \| None` | x5chain (label 33): Ordered chain of X.509 certificates (DER-encoded) |
| `x5t` | `CertificateHash \| None` | x5t (label 34): Certificate thumbprint hash |
| `x5u` | `str \| None` | x5u (label 35): URI pointing to an X.509 certificate |

**Methods:**

- `has_any() -> bool`: Returns `True` if any X.509 headers are present

---

## Crypto Hook Wrapper Classes

Wrapper classes for integrating with external crypto providers (HSMs, Cloud KMS,
remote signing services, smart cards, TPMs, etc.).

### SignatureVerifier

Wrapper for a custom signature verifier callback.

```python
class SignatureVerifier:
    def __init__(
        self,
        callback: Callable[[str, bytes | None, bytes, bytes], None]
    ) -> None
```

The callback receives `(algorithm, key_id, data, signature)` and should raise an exception if verification fails.

---

### Decryptor

Wrapper for a custom decryptor callback.

```python
class Decryptor:
    def __init__(
        self,
        callback: Callable[[str, bytes | None, bytes, bytes, bytes], bytes]
    ) -> None
```

The callback receives `(algorithm, key_id, nonce, aad, ciphertext)` and should return decrypted plaintext bytes.

---

### Signer

Wrapper for a custom signer callback.

```python
class Signer:
    def __init__(
        self,
        callback: Callable[[str, bytes | None, bytes], bytes],
        key_id: bytes | None = None,
    ) -> None
```

The callback receives `(algorithm, key_id, data)` and should return signature bytes.

---

### Encryptor

Wrapper for a custom encryptor callback.

```python
class Encryptor:
    def __init__(
        self,
        callback: Callable[[str, bytes | None, bytes, bytes, bytes], bytes]
    ) -> None
```

The callback receives `(algorithm, key_id, nonce, aad, plaintext)` and should return ciphertext bytes (including the auth tag).

---

## Exceptions

All exceptions inherit from `Claim169Exception`.

### Claim169Exception

Base exception class for all claim169 errors.

```python
class Claim169Exception(Exception):
    pass
```

### Base45DecodeError

Raised when Base45 decoding fails.

```python
class Base45DecodeError(Claim169Exception):
    pass
```

### DecompressError

Raised when zlib decompression fails or size limit exceeded.

```python
class DecompressError(Claim169Exception):
    pass
```

### CoseParseError

Raised when COSE structure parsing fails.

```python
class CoseParseError(Claim169Exception):
    pass
```

### CwtParseError

Raised when CWT parsing fails.

```python
class CwtParseError(Claim169Exception):
    pass
```

### Claim169NotFoundError

Raised when Claim 169 is not present in the CWT.

```python
class Claim169NotFoundError(Claim169Exception):
    pass
```

### SignatureError

Raised when signature verification fails.

```python
class SignatureError(Claim169Exception):
    pass
```

### DecryptionError

Raised when decryption fails.

```python
class DecryptionError(Claim169Exception):
    pass
```

### EncryptionError

Raised when encryption fails.

```python
class EncryptionError(Claim169Exception):
    pass
```

---

## Constants

### Gender Values

| Value | Meaning |
|-------|---------|
| 1 | Male |
| 2 | Female |
| 3 | Other |

### Marital Status Values

| Value | Meaning |
|-------|---------|
| 1 | Unmarried |
| 2 | Married |
| 3 | Divorced |

### Photo Format Values

| Value | Meaning |
|-------|---------|
| 1 | JPEG |
| 2 | JPEG2000 |
| 3 | AVIF |
| 4 | WebP |

### Algorithm Names

**Signing:**
- `"EdDSA"`: Ed25519
- `"ES256"`: ECDSA P-256

**Encryption:**
- `"A256GCM"`: AES-256-GCM
- `"A128GCM"`: AES-128-GCM
