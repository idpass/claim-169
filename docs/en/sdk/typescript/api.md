# API Reference

Complete API documentation for the claim169 TypeScript SDK, auto-generated from source code.


---

# Class: Decoder

Defined in: [src/index.ts:523](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L523)

Builder-pattern decoder for Claim 169 QR codes.

Provides a fluent API for configuring decoding options and executing the decode.
Supports signature verification with Ed25519 and ECDSA P-256, as well as
AES-GCM decryption for encrypted credentials.

## Example

```typescript
// With verification (recommended for production)
const result = new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .decode();

// Without verification (testing only)
const result = new Decoder(qrText)
  .allowUnverified()
  .skipBiometrics()
  .decode();

// With decryption and verification
const result = new Decoder(qrText)
  .decryptWithAes256(aesKey)
  .verifyWithEd25519(publicKey)
  .decode();
```

## Implements

- `IDecoder`

## Constructors

### Constructor

```ts
new Decoder(qrText): Decoder;
```

Defined in: [src/index.ts:532](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L532)

Create a new Decoder instance.

#### Parameters

##### qrText

`string`

The QR code text content (Base45 encoded)

#### Returns

`Decoder`

## Methods

### allowUnverified()

```ts
allowUnverified(): Decoder;
```

Defined in: [src/index.ts:596](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L596)

Allow decoding without signature verification.
WARNING: Credentials decoded with verification skipped (`verificationStatus === "skipped"`)
cannot be trusted. Use for testing only.

#### Returns

`Decoder`

The decoder instance for chaining

#### Implementation of

`IDecoder`.`allowUnverified`

***

### clockSkewTolerance()

```ts
clockSkewTolerance(seconds): Decoder;
```

Defined in: [src/index.ts:664](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L664)

Set clock skew tolerance in seconds.
Allows credentials to be accepted when clocks are slightly out of sync.
Applies when timestamp validation is enabled.

#### Parameters

##### seconds

`number`

The tolerance in seconds

#### Returns

`Decoder`

The decoder instance for chaining

#### Implementation of

`IDecoder`.`clockSkewTolerance`

***

### decode()

```ts
decode(): DecodeResult;
```

Defined in: [src/index.ts:735](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L735)

Decode the QR code with the configured options.
Requires either a verifier (verifyWithEd25519/verifyWithEcdsaP256) or
explicit allowUnverified() to be called first.

#### Returns

`DecodeResult`

The decoded result

#### Throws

If decoding fails or no verification method specified

#### Implementation of

`IDecoder`.`decode`

***

### decryptWith()

```ts
decryptWith(decryptor): Decoder;
```

Defined in: [src/index.ts:721](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L721)

Decrypt with a custom decryptor callback.
Use for external crypto providers (HSM, cloud KMS, etc.)

#### Parameters

##### decryptor

`DecryptorCallback`

Function that decrypts ciphertext

#### Returns

`Decoder`

The decoder instance for chaining

#### Example

```typescript
const result = new Decoder(qrText)
  .decryptWith((algorithm, keyId, nonce, aad, ciphertext) => {
    // Call your crypto provider here
    return myKms.decrypt(keyId, ciphertext, { nonce, aad });
  })
  .decode();
```

#### Implementation of

`IDecoder`.`decryptWith`

***

### decryptWithAes128()

```ts
decryptWithAes128(key): Decoder;
```

Defined in: [src/index.ts:620](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L620)

Decrypt with AES-128-GCM.

#### Parameters

##### key

`Uint8Array`

16-byte AES-128 key

#### Returns

`Decoder`

The decoder instance for chaining

#### Throws

If the key is invalid

#### Implementation of

`IDecoder`.`decryptWithAes128`

***

### decryptWithAes256()

```ts
decryptWithAes256(key): Decoder;
```

Defined in: [src/index.ts:607](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L607)

Decrypt with AES-256-GCM.

#### Parameters

##### key

`Uint8Array`

32-byte AES-256 key

#### Returns

`Decoder`

The decoder instance for chaining

#### Throws

If the key is invalid

#### Implementation of

`IDecoder`.`decryptWithAes256`

***

### maxDecompressedBytes()

```ts
maxDecompressedBytes(bytes): Decoder;
```

Defined in: [src/index.ts:675](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L675)

Set maximum decompressed size in bytes.
Protects against decompression bomb attacks.

#### Parameters

##### bytes

`number`

The maximum size in bytes (default: 65536)

#### Returns

`Decoder`

The decoder instance for chaining

#### Implementation of

`IDecoder`.`maxDecompressedBytes`

***

### skipBiometrics()

```ts
skipBiometrics(): Decoder;
```

Defined in: [src/index.ts:632](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L632)

Skip biometric data during decoding.
Useful when only demographic data is needed for faster parsing.

#### Returns

`Decoder`

The decoder instance for chaining

#### Implementation of

`IDecoder`.`skipBiometrics`

***

### verifyWith()

```ts
verifyWith(verifier): Decoder;
```

Defined in: [src/index.ts:697](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L697)

Verify signature with a custom verifier callback.
Use for external crypto providers (HSM, cloud KMS, remote signing, etc.)

#### Parameters

##### verifier

`VerifierCallback`

Function that verifies signatures

#### Returns

`Decoder`

The decoder instance for chaining

#### Example

```typescript
const result = new Decoder(qrText)
  .verifyWith((algorithm, keyId, data, signature) => {
    // Call your crypto provider here
    myKms.verify(keyId, data, signature);
  })
  .decode();
```

#### Implementation of

`IDecoder`.`verifyWith`

***

### verifyWithEcdsaP256()

```ts
verifyWithEcdsaP256(publicKey): Decoder;
```

Defined in: [src/index.ts:555](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L555)

Verify signature with ECDSA P-256 public key.

#### Parameters

##### publicKey

`Uint8Array`

SEC1-encoded P-256 public key (33 or 65 bytes)

#### Returns

`Decoder`

The decoder instance for chaining

#### Throws

If the public key is invalid

#### Implementation of

`IDecoder`.`verifyWithEcdsaP256`

***

### verifyWithEcdsaP256Pem()

```ts
verifyWithEcdsaP256Pem(pem): Decoder;
```

Defined in: [src/index.ts:583](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L583)

Verify signature with ECDSA P-256 public key in PEM format.
Supports SPKI format with "BEGIN PUBLIC KEY" headers.

#### Parameters

##### pem

`string`

PEM-encoded P-256 public key

#### Returns

`Decoder`

The decoder instance for chaining

#### Throws

If the PEM is invalid

#### Implementation of

`IDecoder`.`verifyWithEcdsaP256Pem`

***

### verifyWithEd25519()

```ts
verifyWithEd25519(publicKey): Decoder;
```

Defined in: [src/index.ts:542](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L542)

Verify signature with Ed25519 public key.

#### Parameters

##### publicKey

`Uint8Array`

32-byte Ed25519 public key

#### Returns

`Decoder`

The decoder instance for chaining

#### Throws

If the public key is invalid

#### Implementation of

`IDecoder`.`verifyWithEd25519`

***

### verifyWithEd25519Pem()

```ts
verifyWithEd25519Pem(pem): Decoder;
```

Defined in: [src/index.ts:569](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L569)

Verify signature with Ed25519 public key in PEM format.
Supports SPKI format with "BEGIN PUBLIC KEY" headers.

#### Parameters

##### pem

`string`

PEM-encoded Ed25519 public key

#### Returns

`Decoder`

The decoder instance for chaining

#### Throws

If the PEM is invalid

#### Implementation of

`IDecoder`.`verifyWithEd25519Pem`

***

### withoutTimestampValidation()

```ts
withoutTimestampValidation(): Decoder;
```

Defined in: [src/index.ts:652](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L652)

Disable timestamp validation.

#### Returns

`Decoder`

The decoder instance for chaining

#### Implementation of

`IDecoder`.`withoutTimestampValidation`

***

### withTimestampValidation()

```ts
withTimestampValidation(): Decoder;
```

Defined in: [src/index.ts:643](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L643)

Re-enable timestamp validation (enabled by default).
When enabled, expired or not-yet-valid credentials will throw an error.
Implemented in the host (JavaScript) to avoid WASM runtime time limitations.

#### Returns

`Decoder`

The decoder instance for chaining

#### Implementation of

`IDecoder`.`withTimestampValidation`

---

# Class: Encoder

Defined in: [src/index.ts:850](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L850)

Builder-pattern encoder for Claim 169 QR codes.

Provides a fluent API for configuring encoding options and generating QR data.

## Example

```typescript
// Signed credential (recommended)
const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(privateKey)
  .encode();

// Signed and encrypted
const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(privateKey)
  .encryptWithAes256(aesKey)
  .encode();

// Unsigned (testing only)
const qrData = new Encoder(claim169, cwtMeta)
  .allowUnsigned()
  .encode();
```

## Implements

- `IEncoder`

## Constructors

### Constructor

```ts
new Encoder(claim169, cwtMeta): Encoder;
```

Defined in: [src/index.ts:858](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L858)

Create a new Encoder instance.

#### Parameters

##### claim169

`Claim169Input`

The identity claim data to encode

##### cwtMeta

`CwtMetaInput`

CWT metadata including issuer, expiration, etc.

#### Returns

`Encoder`

## Methods

### allowUnsigned()

```ts
allowUnsigned(): Encoder;
```

Defined in: [src/index.ts:917](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L917)

Allow encoding without a signature.
WARNING: Unsigned credentials cannot be verified. Use for testing only.

#### Returns

`Encoder`

The encoder instance for chaining

#### Implementation of

`IEncoder`.`allowUnsigned`

***

### compression()

```ts
compression(mode): Encoder;
```

Defined in: [src/index.ts:999](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L999)

Set compression mode for encoding.

#### Parameters

##### mode

`CompressionMode`

Compression mode: "zlib", "none", "adaptive", "brotli:N" (0-11), or "adaptive-brotli:N"

#### Returns

`Encoder`

The encoder instance for chaining

#### Throws

If the mode is invalid or unsupported by the WASM build

#### Implementation of

`IEncoder`.`compression`

***

### encode()

```ts
encode(): EncodeResult;
```

Defined in: [src/index.ts:1011](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L1011)

Encode the credential to a QR-ready result object.

#### Returns

`EncodeResult`

Encode result with QR data, compression info, and warnings

#### Throws

If encoding fails

#### Implementation of

`IEncoder`.`encode`

***

### encryptWith()

```ts
encryptWith(encryptor, algorithm): Encoder;
```

Defined in: [src/index.ts:983](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L983)

Encrypt with a custom encryptor callback.
Use for external crypto providers (HSM, cloud KMS, etc.)

#### Parameters

##### encryptor

`EncryptorCallback`

Function that encrypts data

##### algorithm

Encryption algorithm: "A256GCM" or "A128GCM"

`"A256GCM"` | `"A128GCM"`

#### Returns

`Encoder`

The encoder instance for chaining

#### Example

```typescript
const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(signKey)
  .encryptWith((algorithm, keyId, nonce, aad, plaintext) => {
    return myKms.encrypt({ keyId, nonce, aad, plaintext });
  }, "A256GCM")
  .encode();
```

#### Implementation of

`IEncoder`.`encryptWith`

***

### encryptWithAes128()

```ts
encryptWithAes128(key): Encoder;
```

Defined in: [src/index.ts:905](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L905)

Encrypt with AES-128-GCM.

#### Parameters

##### key

`Uint8Array`

16-byte AES-128 key

#### Returns

`Encoder`

The encoder instance for chaining

#### Implementation of

`IEncoder`.`encryptWithAes128`

***

### encryptWithAes256()

```ts
encryptWithAes256(key): Encoder;
```

Defined in: [src/index.ts:893](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L893)

Encrypt with AES-256-GCM.

#### Parameters

##### key

`Uint8Array`

32-byte AES-256 key

#### Returns

`Encoder`

The encoder instance for chaining

#### Implementation of

`IEncoder`.`encryptWithAes256`

***

### signWith()

```ts
signWith(
   signer, 
   algorithm, 
   keyId?): Encoder;
```

Defined in: [src/index.ts:949](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L949)

Sign with a custom signer callback.
Use for external crypto providers (HSM, cloud KMS, remote signing, etc.)

#### Parameters

##### signer

`SignerCallback`

Function that signs data

##### algorithm

Signature algorithm: "EdDSA" or "ES256"

`"EdDSA"` | `"ES256"`

##### keyId?

Optional key identifier passed to the signer callback

`Uint8Array`\<`ArrayBufferLike`\> | `null`

#### Returns

`Encoder`

The encoder instance for chaining

#### Example

```typescript
const qrData = new Encoder(claim169, cwtMeta)
  .signWith((algorithm, keyId, data) => {
    return myKms.sign({ keyId, data });
  }, "EdDSA")
  .encode();
```

#### Implementation of

`IEncoder`.`signWith`

***

### signWithEcdsaP256()

```ts
signWithEcdsaP256(privateKey): Encoder;
```

Defined in: [src/index.ts:881](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L881)

Sign with ECDSA P-256 private key.

#### Parameters

##### privateKey

`Uint8Array`

32-byte ECDSA P-256 private key (scalar)

#### Returns

`Encoder`

The encoder instance for chaining

#### Implementation of

`IEncoder`.`signWithEcdsaP256`

***

### signWithEd25519()

```ts
signWithEd25519(privateKey): Encoder;
```

Defined in: [src/index.ts:869](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L869)

Sign with Ed25519 private key.

#### Parameters

##### privateKey

`Uint8Array`

32-byte Ed25519 private key

#### Returns

`Encoder`

The encoder instance for chaining

#### Implementation of

`IEncoder`.`signWithEd25519`

***

### skipBiometrics()

```ts
skipBiometrics(): Encoder;
```

Defined in: [src/index.ts:926](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L926)

Skip biometric fields during encoding.

#### Returns

`Encoder`

The encoder instance for chaining

#### Implementation of

`IEncoder`.`skipBiometrics`

---

# Function: bytesToHex()

```ts
function bytesToHex(bytes): string;
```

Defined in: [src/index.ts:208](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L208)

Convert bytes to a lowercase hex string.

## Parameters

### bytes

`Uint8Array`

## Returns

`string`

---

# Function: decode()

```ts
function decode(qrText, options): DecodeResult;
```

Defined in: [src/index.ts:782](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L782)

Decode a Claim 169 QR string.

This is a convenience wrapper around the `Decoder` builder.
Security:
- If you do not pass a verification key, you must set `allowUnverified: true` (testing only).

## Parameters

### qrText

`string`

### options

`DecodeOptions` = `{}`

## Returns

`DecodeResult`

---

# Function: generateNonce()

```ts
function generateNonce(): Uint8Array;
```

Defined in: [src/index.ts:1031](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L1031)

Generate a random 12-byte nonce for AES-GCM encryption.

## Returns

`Uint8Array`

A 12-byte Uint8Array suitable for use as a nonce

---

# Function: hexToBytes()

```ts
function hexToBytes(hex): Uint8Array;
```

Defined in: [src/index.ts:182](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L182)

Convert a hex string to bytes.

Accepts optional `0x` prefix and ignores whitespace.

## Parameters

### hex

`string`

## Returns

`Uint8Array`

## Throws

If the input is not valid hex

---

# Function: inspect()

```ts
function inspect(qrText): InspectResult;
```

Defined in: [src/index.ts:1085](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L1085)

Inspect credential metadata without full decoding or verification.

Extracts metadata (issuer, key ID, algorithm, expiration) from a QR code
without verifying the signature. Useful for multi-issuer key lookup.

For encrypted credentials (COSE_Encrypt0), only COSE-level headers are
available; CWT-level fields (issuer, subject, expiresAt) will be `undefined`.

## Parameters

### qrText

`string`

The Base45-encoded QR code content

## Returns

`InspectResult`

Metadata extracted from the credential

## Throws

On parse errors

## Example

```typescript
import { inspect } from 'claim169';

const meta = inspect(qrText);
console.log(meta.issuer);     // "https://issuer.example.com"
console.log(meta.algorithm);  // "EdDSA"
console.log(meta.coseType);   // "Sign1"
```

---

# Function: isLoaded()

```ts
function isLoaded(): boolean;
```

Defined in: [src/index.ts:171](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L171)

Check if the WASM module is loaded correctly

## Returns

`boolean`

---

# Function: version()

```ts
function version(): string;
```

Defined in: [src/index.ts:164](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L164)

Get the library version

## Returns

`string`

---

# Interface: DecodeOptions

Defined in: [src/index.ts:763](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L763)

Options for the `decode()` convenience function.

Notes:
- If you don't provide a verification key, you must explicitly set `allowUnverified: true` (testing only).
- Timestamp validation is enabled by default in JS (host-side). Set `validateTimestamps: false` to disable.
- PEM keys and custom verifier/decryptor callbacks are not supported here.
  Use the `Decoder` builder class directly for those features.

## Properties

### allowUnverified?

```ts
optional allowUnverified: boolean;
```

Defined in: [src/index.ts:768](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L768)

***

### clockSkewToleranceSeconds?

```ts
optional clockSkewToleranceSeconds: number;
```

Defined in: [src/index.ts:771](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L771)

***

### decryptWithAes128?

```ts
optional decryptWithAes128: Uint8Array<ArrayBufferLike>;
```

Defined in: [src/index.ts:767](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L767)

***

### decryptWithAes256?

```ts
optional decryptWithAes256: Uint8Array<ArrayBufferLike>;
```

Defined in: [src/index.ts:766](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L766)

***

### maxDecompressedBytes?

```ts
optional maxDecompressedBytes: number;
```

Defined in: [src/index.ts:772](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L772)

***

### skipBiometrics?

```ts
optional skipBiometrics: boolean;
```

Defined in: [src/index.ts:769](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L769)

***

### validateTimestamps?

```ts
optional validateTimestamps: boolean;
```

Defined in: [src/index.ts:770](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L770)

***

### verifyWithEcdsaP256?

```ts
optional verifyWithEcdsaP256: Uint8Array<ArrayBufferLike>;
```

Defined in: [src/index.ts:765](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L765)

***

### verifyWithEd25519?

```ts
optional verifyWithEd25519: Uint8Array<ArrayBufferLike>;
```

Defined in: [src/index.ts:764](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/index.ts#L764)

---

# claim169

MOSIP Claim 169 QR Code library for TypeScript/JavaScript.

This library provides classes to encode and decode MOSIP Claim 169 identity
credentials from QR codes. It uses WebAssembly for high-performance binary
parsing and cryptographic operations.

## Installation

```bash
npm install claim169
```

## Decoding with Verification (Recommended)

```typescript
import { Decoder } from 'claim169';

// Decode with Ed25519 signature verification
const result = new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .decode();

// Access identity data
console.log(result.claim169.fullName);
console.log(result.claim169.dateOfBirth);

// Access metadata
console.log(result.cwtMeta.issuer);
console.log(result.cwtMeta.expiresAt);

// Check verification status
console.log(result.verificationStatus); // "verified"
```

## Decoding without Verification (Testing Only)

```typescript
const result = new Decoder(qrText)
  .allowUnverified()  // Explicit opt-out required
  .decode();
```

## Decoding Encrypted Credentials

```typescript
const result = new Decoder(qrText)
  .decryptWithAes256(aesKey)
  .verifyWithEd25519(publicKey)
  .decode();
```

## Encoding Credentials

```typescript
import { Encoder } from 'claim169';

const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(privateKey)
  .encode();
```

## Error Handling

```typescript
import { Decoder, Claim169Error } from 'claim169';

try {
  const result = new Decoder(qrText)
    .verifyWithEd25519(publicKey)
    .decode();
} catch (error) {
  if (error instanceof Claim169Error) {
    console.error('Decoding failed:', error.message);
  }
}
```

## Notes

- **Timestamp validation**: Enabled by default in JS (host-side). Disable with
  `.withoutTimestampValidation()` if you intentionally want to skip time checks.

## Classes

- Decoder
- Encoder

## Interfaces

- DecodeOptions

## Functions

- bytesToHex
- decode
- generateNonce
- hexToBytes
- inspect
- isLoaded
- version

## References

### Algorithm

Re-exports Algorithm

***

### AlgorithmName

Re-exports AlgorithmName

***

### Biometric

Re-exports Biometric

***

### BiometricFormat

Re-exports BiometricFormat

***

### CertificateHash

Re-exports CertificateHash

***

### Claim169

Re-exports Claim169

***

### Claim169Error

Re-exports Claim169Error

***

### Claim169ErrorCode

Re-exports Claim169ErrorCode

***

### Claim169Input

Re-exports Claim169Input

***

### CompressionMode

Re-exports CompressionMode

***

### CoseType

Re-exports CoseType

***

### CwtMeta

Re-exports CwtMeta

***

### CwtMetaInput

Re-exports CwtMetaInput

***

### DecodeResult

Re-exports DecodeResult

***

### DecryptorCallback

Re-exports DecryptorCallback

***

### DetectedCompression

Re-exports DetectedCompression

***

### EncodeResult

Re-exports EncodeResult

***

### EncodeWarning

Re-exports EncodeWarning

***

### EncryptorCallback

Re-exports EncryptorCallback

***

### Gender

Re-exports Gender

***

### IDecoder

Re-exports IDecoder

***

### IEncoder

Re-exports IEncoder

***

### InspectResult

Re-exports InspectResult

***

### MaritalStatus

Re-exports MaritalStatus

***

### PhotoFormat

Re-exports PhotoFormat

***

### SignerCallback

Re-exports SignerCallback

***

### VerificationStatus

Re-exports VerificationStatus

***

### VerifierCallback

Re-exports VerifierCallback

***

### X509Headers

Re-exports X509Headers

---

# Class: Claim169Error

Defined in: [src/types.ts:652](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L652)

## Extends

- `Error`

## Constructors

### Constructor

```ts
new Claim169Error(message, code?): Claim169Error;
```

Defined in: [src/types.ts:656](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L656)

#### Parameters

##### message

`string`

##### code?

`Claim169ErrorCode`

#### Returns

`Claim169Error`

#### Overrides

```ts
Error.constructor
```

## Properties

### code

```ts
readonly code: Claim169ErrorCode;
```

Defined in: [src/types.ts:654](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L654)

Programmatic error code for matching error types.

***

### message

```ts
message: string;
```

Defined in: node\_modules/typescript/lib/lib.es5.d.ts:1077

#### Inherited from

```ts
Error.message
```

***

### name

```ts
name: string;
```

Defined in: node\_modules/typescript/lib/lib.es5.d.ts:1076

#### Inherited from

```ts
Error.name
```

***

### stack?

```ts
optional stack: string;
```

Defined in: node\_modules/typescript/lib/lib.es5.d.ts:1078

#### Inherited from

```ts
Error.stack
```

---

# Interface: Biometric

Defined in: [src/types.ts:26](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L26)

A single biometric data entry from a Claim 169 credential.

Biometric data can be fingerprints, iris scans, face images, or voice samples.
Each entry contains the raw data and metadata about its format.

## Example

```typescript
// Access face biometric data
if (claim.face && claim.face.length > 0) {
  const faceData = claim.face[0];
  console.log(`Format: ${faceData.format}`);
  console.log(`Data size: ${faceData.data.byteLength} bytes`);
}
```

## Properties

### data

```ts
data: Uint8Array;
```

Defined in: [src/types.ts:28](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L28)

Raw biometric data bytes (image, template, or audio)

***

### format

```ts
format: number;
```

Defined in: [src/types.ts:36](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L36)

Biometric format code:
- 0: Image
- 1: Template
- 2: Sound
- 3: BioHash

***

### issuer?

```ts
optional issuer: string;
```

Defined in: [src/types.ts:45](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L45)

Biometric data issuer/provider identifier

***

### subFormat?

```ts
optional subFormat: number;
```

Defined in: [src/types.ts:43](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L43)

Sub-format code (depends on format type):
- For Image: 0=PNG, 1=JPEG, 2=JPEG2000, 3=AVIF, 4=WebP, 5=TIFF, 6=WSQ
- For Template: 0=ANSI378, 1=ISO19794-2, 2=NIST
- For Sound: 0=WAV, 1=MP3

---

# Interface: CertificateHash

Defined in: [src/types.ts:87](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L87)

X.509 certificate hash (COSE_CertHash).

Contains a hash algorithm identifier and the hash value.
Used in the x5t (thumbprint) header parameter.

## Properties

### algorithm

```ts
algorithm: string;
```

Defined in: [src/types.ts:92](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L92)

Hash algorithm identifier.
Can be a numeric COSE algorithm ID (e.g., "-16" for SHA-256) or a named algorithm.

***

### hashValue

```ts
hashValue: Uint8Array;
```

Defined in: [src/types.ts:94](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L94)

Hash value bytes

---

# Interface: Claim169

Defined in: [src/types.ts:166](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L166)

Decoded Claim 169 identity data.

This interface contains all identity fields defined in the MOSIP Claim 169
specification. All fields are optional since credentials may contain only
a subset of the available fields.

Fields are organized into:
- **Demographics** (id, name, DOB, address, etc.)
- **Biometrics** (fingerprints, iris, face, voice)

## Example

```typescript
// Access demographic data
console.log(`Name: ${claim.fullName}`);
console.log(`DOB: ${claim.dateOfBirth}`);

// Check for biometrics
const hasFace = claim.face && claim.face.length > 0;
const hasFingerprints = claim.rightThumb || claim.leftThumb;
```

## Properties

### address?

```ts
optional address: string;
```

Defined in: [src/types.ts:186](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L186)

Address

***

### bestQualityFingers?

```ts
optional bestQualityFingers: Uint8Array<ArrayBufferLike>;
```

Defined in: [src/types.ts:202](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L202)

Best quality fingers indicator

***

### countryOfIssuance?

```ts
optional countryOfIssuance: string;
```

Defined in: [src/types.ts:212](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L212)

Country of issuance

***

### dateOfBirth?

```ts
optional dateOfBirth: string;
```

Defined in: [src/types.ts:182](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L182)

Date of birth (ISO 8601 format)

***

### email?

```ts
optional email: string;
```

Defined in: [src/types.ts:188](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L188)

Email address

***

### face?

```ts
optional face: Biometric[];
```

Defined in: [src/types.ts:239](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L239)

Face biometrics

***

### firstName?

```ts
optional firstName: string;
```

Defined in: [src/types.ts:176](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L176)

First name

***

### fullName?

```ts
optional fullName: string;
```

Defined in: [src/types.ts:174](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L174)

Full name

***

### gender?

```ts
optional gender: number;
```

Defined in: [src/types.ts:184](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L184)

Gender code (1=Male, 2=Female, 3=Other)

***

### guardian?

```ts
optional guardian: string;
```

Defined in: [src/types.ts:196](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L196)

Guardian name

***

### id?

```ts
optional id: string;
```

Defined in: [src/types.ts:168](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L168)

Unique identifier (CBOR key 1)

***

### language?

```ts
optional language: string;
```

Defined in: [src/types.ts:172](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L172)

Primary language code

***

### lastName?

```ts
optional lastName: string;
```

Defined in: [src/types.ts:180](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L180)

Last name

***

### leftIris?

```ts
optional leftIris: Biometric[];
```

Defined in: [src/types.ts:237](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L237)

Left iris biometrics

***

### leftLittleFinger?

```ts
optional leftLittleFinger: Biometric[];
```

Defined in: [src/types.ts:233](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L233)

Left little finger biometrics

***

### leftMiddleFinger?

```ts
optional leftMiddleFinger: Biometric[];
```

Defined in: [src/types.ts:229](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L229)

Left middle finger biometrics

***

### leftPalm?

```ts
optional leftPalm: Biometric[];
```

Defined in: [src/types.ts:243](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L243)

Left palm biometrics

***

### leftPointerFinger?

```ts
optional leftPointerFinger: Biometric[];
```

Defined in: [src/types.ts:227](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L227)

Left pointer finger biometrics

***

### leftRingFinger?

```ts
optional leftRingFinger: Biometric[];
```

Defined in: [src/types.ts:231](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L231)

Left ring finger biometrics

***

### leftThumb?

```ts
optional leftThumb: Biometric[];
```

Defined in: [src/types.ts:225](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L225)

Left thumb biometrics

***

### legalStatus?

```ts
optional legalStatus: string;
```

Defined in: [src/types.ts:210](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L210)

Legal status

***

### locationCode?

```ts
optional locationCode: string;
```

Defined in: [src/types.ts:208](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L208)

Location code

***

### maritalStatus?

```ts
optional maritalStatus: number;
```

Defined in: [src/types.ts:194](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L194)

Marital status code

***

### middleName?

```ts
optional middleName: string;
```

Defined in: [src/types.ts:178](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L178)

Middle name

***

### nationality?

```ts
optional nationality: string;
```

Defined in: [src/types.ts:192](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L192)

Nationality

***

### phone?

```ts
optional phone: string;
```

Defined in: [src/types.ts:190](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L190)

Phone number

***

### photo?

```ts
optional photo: Uint8Array<ArrayBufferLike>;
```

Defined in: [src/types.ts:198](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L198)

Photo data

***

### photoFormat?

```ts
optional photoFormat: number;
```

Defined in: [src/types.ts:200](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L200)

Photo format code

***

### rightIris?

```ts
optional rightIris: Biometric[];
```

Defined in: [src/types.ts:235](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L235)

Right iris biometrics

***

### rightLittleFinger?

```ts
optional rightLittleFinger: Biometric[];
```

Defined in: [src/types.ts:223](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L223)

Right little finger biometrics

***

### rightMiddleFinger?

```ts
optional rightMiddleFinger: Biometric[];
```

Defined in: [src/types.ts:219](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L219)

Right middle finger biometrics

***

### rightPalm?

```ts
optional rightPalm: Biometric[];
```

Defined in: [src/types.ts:241](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L241)

Right palm biometrics

***

### rightPointerFinger?

```ts
optional rightPointerFinger: Biometric[];
```

Defined in: [src/types.ts:217](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L217)

Right pointer finger biometrics

***

### rightRingFinger?

```ts
optional rightRingFinger: Biometric[];
```

Defined in: [src/types.ts:221](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L221)

Right ring finger biometrics

***

### rightThumb?

```ts
optional rightThumb: Biometric[];
```

Defined in: [src/types.ts:215](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L215)

Right thumb biometrics

***

### secondaryFullName?

```ts
optional secondaryFullName: string;
```

Defined in: [src/types.ts:204](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L204)

Secondary full name

***

### secondaryLanguage?

```ts
optional secondaryLanguage: string;
```

Defined in: [src/types.ts:206](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L206)

Secondary language code

***

### version?

```ts
optional version: string;
```

Defined in: [src/types.ts:170](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L170)

Claim version

***

### voice?

```ts
optional voice: Biometric[];
```

Defined in: [src/types.ts:245](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L245)

Voice biometrics

---

# Interface: Claim169Input

Defined in: [src/types.ts:683](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L683)

Input data for creating a Claim 169 credential.

This interface contains all identity fields that can be encoded into
a Claim 169 QR code.

## Example

```typescript
const claim169: Claim169Input = {
  id: "123456789",
  fullName: "John Doe",
  dateOfBirth: "1990-01-15",
  gender: 1,  // Male
};
```

## Properties

### address?

```ts
optional address: string;
```

Defined in: [src/types.ts:703](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L703)

Address

***

### countryOfIssuance?

```ts
optional countryOfIssuance: string;
```

Defined in: [src/types.ts:727](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L727)

Country of issuance

***

### dateOfBirth?

```ts
optional dateOfBirth: string;
```

Defined in: [src/types.ts:699](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L699)

Date of birth (ISO 8601 format)

***

### email?

```ts
optional email: string;
```

Defined in: [src/types.ts:705](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L705)

Email address

***

### face?

```ts
optional face: Biometric[];
```

Defined in: [src/types.ts:754](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L754)

Face biometrics

***

### firstName?

```ts
optional firstName: string;
```

Defined in: [src/types.ts:693](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L693)

First name

***

### fullName?

```ts
optional fullName: string;
```

Defined in: [src/types.ts:691](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L691)

Full name

***

### gender?

```ts
optional gender: number;
```

Defined in: [src/types.ts:701](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L701)

Gender code (1=Male, 2=Female, 3=Other)

***

### guardian?

```ts
optional guardian: string;
```

Defined in: [src/types.ts:713](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L713)

Guardian name

***

### id?

```ts
optional id: string;
```

Defined in: [src/types.ts:685](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L685)

Unique identifier

***

### language?

```ts
optional language: string;
```

Defined in: [src/types.ts:689](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L689)

Primary language code

***

### lastName?

```ts
optional lastName: string;
```

Defined in: [src/types.ts:697](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L697)

Last name

***

### leftIris?

```ts
optional leftIris: Biometric[];
```

Defined in: [src/types.ts:752](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L752)

Left iris biometrics

***

### leftLittleFinger?

```ts
optional leftLittleFinger: Biometric[];
```

Defined in: [src/types.ts:748](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L748)

Left little finger biometrics

***

### leftMiddleFinger?

```ts
optional leftMiddleFinger: Biometric[];
```

Defined in: [src/types.ts:744](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L744)

Left middle finger biometrics

***

### leftPalm?

```ts
optional leftPalm: Biometric[];
```

Defined in: [src/types.ts:758](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L758)

Left palm biometrics

***

### leftPointerFinger?

```ts
optional leftPointerFinger: Biometric[];
```

Defined in: [src/types.ts:742](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L742)

Left pointer finger biometrics

***

### leftRingFinger?

```ts
optional leftRingFinger: Biometric[];
```

Defined in: [src/types.ts:746](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L746)

Left ring finger biometrics

***

### leftThumb?

```ts
optional leftThumb: Biometric[];
```

Defined in: [src/types.ts:740](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L740)

Left thumb biometrics

***

### legalStatus?

```ts
optional legalStatus: string;
```

Defined in: [src/types.ts:725](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L725)

Legal status

***

### locationCode?

```ts
optional locationCode: string;
```

Defined in: [src/types.ts:723](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L723)

Location code

***

### maritalStatus?

```ts
optional maritalStatus: number;
```

Defined in: [src/types.ts:711](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L711)

Marital status code (1=Unmarried, 2=Married, 3=Divorced)

***

### middleName?

```ts
optional middleName: string;
```

Defined in: [src/types.ts:695](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L695)

Middle name

***

### nationality?

```ts
optional nationality: string;
```

Defined in: [src/types.ts:709](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L709)

Nationality

***

### phone?

```ts
optional phone: string;
```

Defined in: [src/types.ts:707](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L707)

Phone number

***

### photo?

```ts
optional photo: Uint8Array<ArrayBufferLike>;
```

Defined in: [src/types.ts:715](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L715)

Photo data

***

### photoFormat?

```ts
optional photoFormat: number;
```

Defined in: [src/types.ts:717](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L717)

Photo format code (1=JPEG, 2=JPEG2000, 3=AVIF, 4=WebP)

***

### rightIris?

```ts
optional rightIris: Biometric[];
```

Defined in: [src/types.ts:750](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L750)

Right iris biometrics

***

### rightLittleFinger?

```ts
optional rightLittleFinger: Biometric[];
```

Defined in: [src/types.ts:738](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L738)

Right little finger biometrics

***

### rightMiddleFinger?

```ts
optional rightMiddleFinger: Biometric[];
```

Defined in: [src/types.ts:734](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L734)

Right middle finger biometrics

***

### rightPalm?

```ts
optional rightPalm: Biometric[];
```

Defined in: [src/types.ts:756](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L756)

Right palm biometrics

***

### rightPointerFinger?

```ts
optional rightPointerFinger: Biometric[];
```

Defined in: [src/types.ts:732](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L732)

Right pointer finger biometrics

***

### rightRingFinger?

```ts
optional rightRingFinger: Biometric[];
```

Defined in: [src/types.ts:736](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L736)

Right ring finger biometrics

***

### rightThumb?

```ts
optional rightThumb: Biometric[];
```

Defined in: [src/types.ts:730](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L730)

Right thumb biometrics

***

### secondaryFullName?

```ts
optional secondaryFullName: string;
```

Defined in: [src/types.ts:719](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L719)

Secondary full name

***

### secondaryLanguage?

```ts
optional secondaryLanguage: string;
```

Defined in: [src/types.ts:721](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L721)

Secondary language code

***

### version?

```ts
optional version: string;
```

Defined in: [src/types.ts:687](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L687)

Claim version

***

### voice?

```ts
optional voice: Biometric[];
```

Defined in: [src/types.ts:760](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L760)

Voice biometrics

---

# Interface: CwtMeta

Defined in: [src/types.ts:68](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L68)

CWT (CBOR Web Token) metadata from the credential.

Contains standard JWT/CWT claims that provide information about the
credential's validity, issuer, and subject.

## Example

```typescript
// Check if credential is expired
const now = Math.floor(Date.now() / 1000);
if (result.cwtMeta.expiresAt && result.cwtMeta.expiresAt < now) {
  console.log('Credential has expired!');
}

// Check issuer
if (result.cwtMeta.issuer === 'https://mosip.io') {
  console.log('Issued by MOSIP');
}
```

## Properties

### expiresAt?

```ts
optional expiresAt: number;
```

Defined in: [src/types.ts:74](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L74)

Expiration timestamp (Unix seconds) - credential invalid after this time

***

### issuedAt?

```ts
optional issuedAt: number;
```

Defined in: [src/types.ts:78](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L78)

Issued-at timestamp (Unix seconds) - when the credential was created

***

### issuer?

```ts
optional issuer: string;
```

Defined in: [src/types.ts:70](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L70)

Token issuer (typically a URL or identifier)

***

### notBefore?

```ts
optional notBefore: number;
```

Defined in: [src/types.ts:76](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L76)

Not-before timestamp (Unix seconds) - credential invalid before this time

***

### subject?

```ts
optional subject: string;
```

Defined in: [src/types.ts:72](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L72)

Token subject (typically the credential holder's ID)

---

# Interface: CwtMetaInput

Defined in: [src/types.ts:774](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L774)

CWT metadata input for creating a Claim 169 credential.

## Example

```typescript
const cwtMeta: CwtMetaInput = {
  issuer: "https://issuer.example.com",
  expiresAt: 1800000000,  // Unix timestamp
};
```

## Properties

### expiresAt?

```ts
optional expiresAt: number;
```

Defined in: [src/types.ts:780](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L780)

Expiration timestamp (Unix seconds)

***

### issuedAt?

```ts
optional issuedAt: number;
```

Defined in: [src/types.ts:784](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L784)

Issued-at timestamp (Unix seconds)

***

### issuer?

```ts
optional issuer: string;
```

Defined in: [src/types.ts:776](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L776)

Token issuer (typically a URL or identifier)

***

### notBefore?

```ts
optional notBefore: number;
```

Defined in: [src/types.ts:782](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L782)

Not-before timestamp (Unix seconds)

***

### subject?

```ts
optional subject: string;
```

Defined in: [src/types.ts:778](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L778)

Token subject (typically the credential holder's ID)

---

# Interface: DecodeResult

Defined in: [src/types.ts:424](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L424)

Result of decoding a Claim 169 QR code.

Contains the decoded identity data, CWT metadata, and verification status.

## Example

```typescript
// Decode with verification
const result = new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .decode();

// Access identity data
console.log(result.claim169.fullName);

// Access metadata
console.log(result.cwtMeta.issuer);

// Check verification status
console.log(result.verificationStatus); // "verified", "skipped", or "failed"
```

## Properties

### algorithm?

```ts
optional algorithm: string;
```

Defined in: [src/types.ts:448](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L448)

COSE algorithm name (e.g., "EdDSA", "ES256"), if present.

***

### claim169

```ts
claim169: Claim169;
```

Defined in: [src/types.ts:426](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L426)

Decoded Claim 169 identity data

***

### cwtMeta

```ts
cwtMeta: CwtMeta;
```

Defined in: [src/types.ts:428](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L428)

CWT metadata (issuer, expiration, etc.)

***

### detectedCompression

```ts
detectedCompression: DetectedCompression;
```

Defined in: [src/types.ts:442](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L442)

Compression format detected during decoding (e.g., "zlib", "brotli", "none")

***

### keyId?

```ts
optional keyId: Uint8Array<ArrayBufferLike>;
```

Defined in: [src/types.ts:446](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L446)

Key ID from the COSE header, if present.

***

### verificationStatus

```ts
verificationStatus: VerificationStatus;
```

Defined in: [src/types.ts:435](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L435)

Signature verification status.
- "verified": Signature verified successfully with provided public key
- "skipped": Verification skipped (allowUnverified() or decode(..., { allowUnverified: true }))
- "failed": Signature verification failed

***

### warnings

```ts
warnings: EncodeWarning[];
```

Defined in: [src/types.ts:444](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L444)

Warnings generated during decoding

***

### x509Headers

```ts
x509Headers: X509Headers;
```

Defined in: [src/types.ts:440](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L440)

X.509 headers from COSE protected/unprotected headers.
Contains certificate information for signature verification.

---

# Interface: EncodeResult

Defined in: [src/types.ts:504](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L504)

Result of encoding a Claim 169 credential.

Contains the QR-ready Base45 string, the compression method that was
actually used, and any warnings generated during encoding.

## Example

```typescript
const result = new Encoder(claim169, cwtMeta)
  .signWithEd25519(privateKey)
  .compression("zlib")
  .encode();

console.log(result.qrData);           // Base45 string
console.log(result.compressionUsed);  // "zlib"
console.log(result.warnings);         // []
```

## Properties

### compressionUsed

```ts
compressionUsed: string;
```

Defined in: [src/types.ts:508](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L508)

Compression method that was actually used (e.g., "zlib", "brotli", "none")

***

### qrData

```ts
qrData: string;
```

Defined in: [src/types.ts:506](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L506)

Base45-encoded string suitable for QR code generation

***

### warnings

```ts
warnings: EncodeWarning[];
```

Defined in: [src/types.ts:510](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L510)

Warnings generated during encoding

---

# Interface: EncodeWarning

Defined in: [src/types.ts:479](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L479)

Warning from the encode/decode pipeline.

## Properties

### code

```ts
code: string;
```

Defined in: [src/types.ts:481](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L481)

Warning code (e.g., "non_standard_compression")

***

### message

```ts
message: string;
```

Defined in: [src/types.ts:483](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L483)

Human-readable warning message

---

# Interface: IDecoder

Defined in: [src/types.ts:933](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L933)

Interface for the Decoder builder class.

Provides a fluent API for configuring and decoding Claim 169 QR codes.

## Example

```typescript
// With verification (recommended)
const result = new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .decode();

// Without verification (testing only)
const result = new Decoder(qrText)
  .allowUnverified()
  .skipBiometrics()
  .decode();

// With decryption and verification
const result = new Decoder(qrText)
  .decryptWithAes256(aesKey)
  .verifyWithEd25519(publicKey)
  .decode();
```

## Methods

### allowUnverified()

```ts
allowUnverified(): IDecoder;
```

Defined in: [src/types.ts:974](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L974)

Allow decoding without signature verification.
WARNING: Credentials decoded with verification skipped (`verificationStatus === "skipped"`)
cannot be trusted. Use for testing only.

#### Returns

`IDecoder`

The decoder instance for chaining

***

### clockSkewTolerance()

```ts
clockSkewTolerance(seconds): IDecoder;
```

Defined in: [src/types.ts:1019](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L1019)

Set clock skew tolerance in seconds.
Allows credentials to be accepted when clocks are slightly out of sync.
Applies when timestamp validation is enabled.

#### Parameters

##### seconds

`number`

The tolerance in seconds

#### Returns

`IDecoder`

The decoder instance for chaining

***

### decode()

```ts
decode(): DecodeResult;
```

Defined in: [src/types.ts:1074](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L1074)

Decode the QR code with the configured options.
Requires either a verifier (verifyWithEd25519/verifyWithEcdsaP256) or
explicit allowUnverified() to be called first.

#### Returns

`DecodeResult`

The decoded result

#### Throws

If decoding fails or no verification method specified

***

### decryptWith()

```ts
decryptWith(decryptor): IDecoder;
```

Defined in: [src/types.ts:1065](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L1065)

Decrypt with a custom decryptor callback.
Use for external crypto providers (HSM, cloud KMS, etc.)

#### Parameters

##### decryptor

`DecryptorCallback`

Function that decrypts ciphertext

#### Returns

`IDecoder`

The decoder instance for chaining

#### Example

```typescript
const result = new Decoder(qrText)
  .decryptWith((algorithm, keyId, nonce, aad, ciphertext) => {
    // Call your crypto provider here
    return myKms.decrypt(keyId, ciphertext, { nonce, aad });
  })
  .decode();
```

***

### decryptWithAes128()

```ts
decryptWithAes128(key): IDecoder;
```

Defined in: [src/types.ts:990](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L990)

Decrypt with AES-128-GCM.

#### Parameters

##### key

`Uint8Array`

16-byte AES-128 key

#### Returns

`IDecoder`

The decoder instance for chaining

#### Throws

If the key is invalid

***

### decryptWithAes256()

```ts
decryptWithAes256(key): IDecoder;
```

Defined in: [src/types.ts:982](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L982)

Decrypt with AES-256-GCM.

#### Parameters

##### key

`Uint8Array`

32-byte AES-256 key

#### Returns

`IDecoder`

The decoder instance for chaining

#### Throws

If the key is invalid

***

### maxDecompressedBytes()

```ts
maxDecompressedBytes(bytes): IDecoder;
```

Defined in: [src/types.ts:1027](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L1027)

Set maximum decompressed size in bytes.
Protects against decompression bomb attacks.

#### Parameters

##### bytes

`number`

The maximum size in bytes (default: 65536)

#### Returns

`IDecoder`

The decoder instance for chaining

***

### skipBiometrics()

```ts
skipBiometrics(): IDecoder;
```

Defined in: [src/types.ts:997](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L997)

Skip biometric data during decoding.
Useful when only demographic data is needed.

#### Returns

`IDecoder`

The decoder instance for chaining

***

### verifyWith()

```ts
verifyWith(verifier): IDecoder;
```

Defined in: [src/types.ts:1046](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L1046)

Verify signature with a custom verifier callback.
Use for external crypto providers (HSM, cloud KMS, remote signing, etc.)

#### Parameters

##### verifier

`VerifierCallback`

Function that verifies signatures

#### Returns

`IDecoder`

The decoder instance for chaining

#### Example

```typescript
const result = new Decoder(qrText)
  .verifyWith((algorithm, keyId, data, signature) => {
    // Call your crypto provider here
    myKms.verify(keyId, data, signature);
  })
  .decode();
```

***

### verifyWithEcdsaP256()

```ts
verifyWithEcdsaP256(publicKey): IDecoder;
```

Defined in: [src/types.ts:948](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L948)

Verify signature with ECDSA P-256 public key.

#### Parameters

##### publicKey

`Uint8Array`

SEC1-encoded P-256 public key (33 or 65 bytes)

#### Returns

`IDecoder`

The decoder instance for chaining

#### Throws

If the public key is invalid

***

### verifyWithEcdsaP256Pem()

```ts
verifyWithEcdsaP256Pem(pem): IDecoder;
```

Defined in: [src/types.ts:966](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L966)

Verify signature with ECDSA P-256 public key in PEM format.
Supports SPKI format with "BEGIN PUBLIC KEY" headers.

#### Parameters

##### pem

`string`

PEM-encoded P-256 public key

#### Returns

`IDecoder`

The decoder instance for chaining

#### Throws

If the PEM is invalid

***

### verifyWithEd25519()

```ts
verifyWithEd25519(publicKey): IDecoder;
```

Defined in: [src/types.ts:940](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L940)

Verify signature with Ed25519 public key.

#### Parameters

##### publicKey

`Uint8Array`

32-byte Ed25519 public key

#### Returns

`IDecoder`

The decoder instance for chaining

#### Throws

If the public key is invalid

***

### verifyWithEd25519Pem()

```ts
verifyWithEd25519Pem(pem): IDecoder;
```

Defined in: [src/types.ts:957](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L957)

Verify signature with Ed25519 public key in PEM format.
Supports SPKI format with "BEGIN PUBLIC KEY" headers.

#### Parameters

##### pem

`string`

PEM-encoded Ed25519 public key

#### Returns

`IDecoder`

The decoder instance for chaining

#### Throws

If the PEM is invalid

***

### withoutTimestampValidation()

```ts
withoutTimestampValidation(): IDecoder;
```

Defined in: [src/types.ts:1010](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L1010)

Disable timestamp validation.

#### Returns

`IDecoder`

The decoder instance for chaining

***

### withTimestampValidation()

```ts
withTimestampValidation(): IDecoder;
```

Defined in: [src/types.ts:1004](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L1004)

Re-enable timestamp validation (enabled by default).
When enabled, expired or not-yet-valid credentials will throw an error.

#### Returns

`IDecoder`

The decoder instance for chaining

---

# Interface: IEncoder

Defined in: [src/types.ts:799](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L799)

Interface for the Encoder builder class.

Provides a fluent API for configuring and encoding Claim 169 credentials.

## Example

```typescript
const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(privateKey)
  .encode();
```

## Methods

### allowUnsigned()

```ts
allowUnsigned(): IEncoder;
```

Defined in: [src/types.ts:833](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L833)

Allow encoding without a signature.
WARNING: Unsigned credentials cannot be verified. Use for testing only.

#### Returns

`IEncoder`

The encoder instance for chaining

***

### compression()

```ts
compression(mode): IEncoder;
```

Defined in: [src/types.ts:894](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L894)

Set compression mode for encoding.

#### Parameters

##### mode

`CompressionMode`

Compression mode: "zlib", "none", "adaptive", "brotli:N" (0-11), or "adaptive-brotli:N"

#### Returns

`IEncoder`

The encoder instance for chaining

#### Throws

If the mode is invalid or unsupported by the WASM build

***

### encode()

```ts
encode(): EncodeResult;
```

Defined in: [src/types.ts:901](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L901)

Encode the credential to a QR-ready result object.

#### Returns

`EncodeResult`

Encode result with QR data, compression info, and warnings

#### Throws

If encoding fails

***

### encryptWith()

```ts
encryptWith(encryptor, algorithm): IEncoder;
```

Defined in: [src/types.ts:883](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L883)

Encrypt with a custom encryptor callback.
Use for external crypto providers (HSM, cloud KMS, etc.)

#### Parameters

##### encryptor

`EncryptorCallback`

Function that encrypts data

##### algorithm

Encryption algorithm: "A256GCM" or "A128GCM"

`"A256GCM"` | `"A128GCM"`

#### Returns

`IEncoder`

The encoder instance for chaining

#### Example

```typescript
const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(signKey)
  .encryptWith((algorithm, keyId, nonce, aad, plaintext) => {
    return myKms.encrypt({ keyId, nonce, aad, plaintext });
  }, "A256GCM")
  .encode();
```

***

### encryptWithAes128()

```ts
encryptWithAes128(key): IEncoder;
```

Defined in: [src/types.ts:826](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L826)

Encrypt with AES-128-GCM.

#### Parameters

##### key

`Uint8Array`

16-byte AES-128 key

#### Returns

`IEncoder`

The encoder instance for chaining

***

### encryptWithAes256()

```ts
encryptWithAes256(key): IEncoder;
```

Defined in: [src/types.ts:819](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L819)

Encrypt with AES-256-GCM.

#### Parameters

##### key

`Uint8Array`

32-byte AES-256 key

#### Returns

`IEncoder`

The encoder instance for chaining

***

### signWith()

```ts
signWith(
   signer, 
   algorithm, 
   keyId?): IEncoder;
```

Defined in: [src/types.ts:859](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L859)

Sign with a custom signer callback.
Use for external crypto providers (HSM, cloud KMS, remote signing, etc.)

#### Parameters

##### signer

`SignerCallback`

Function that signs data

##### algorithm

Signature algorithm: "EdDSA" or "ES256"

`"EdDSA"` | `"ES256"`

##### keyId?

Optional key identifier passed to the signer callback

`Uint8Array`\<`ArrayBufferLike`\> | `null`

#### Returns

`IEncoder`

The encoder instance for chaining

#### Example

```typescript
const qrData = new Encoder(claim169, cwtMeta)
  .signWith((algorithm, keyId, data) => {
    return myKms.sign({ keyId, data });
  }, "EdDSA")
  .encode();
```

***

### signWithEcdsaP256()

```ts
signWithEcdsaP256(privateKey): IEncoder;
```

Defined in: [src/types.ts:812](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L812)

Sign with ECDSA P-256 private key.

#### Parameters

##### privateKey

`Uint8Array`

32-byte ECDSA P-256 private key (scalar)

#### Returns

`IEncoder`

The encoder instance for chaining

***

### signWithEd25519()

```ts
signWithEd25519(privateKey): IEncoder;
```

Defined in: [src/types.ts:805](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L805)

Sign with Ed25519 private key.

#### Parameters

##### privateKey

`Uint8Array`

32-byte Ed25519 private key

#### Returns

`IEncoder`

The encoder instance for chaining

***

### skipBiometrics()

```ts
skipBiometrics(): IEncoder;
```

Defined in: [src/types.ts:839](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L839)

Skip biometric fields during encoding.

#### Returns

`IEncoder`

The encoder instance for chaining

---

# Interface: InspectResult

Defined in: [src/types.ts:459](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L459)

Metadata extracted from a credential without full verification or decoding.

Useful for determining which key to use in multi-issuer scenarios.

## Properties

### algorithm?

```ts
optional algorithm: string;
```

Defined in: [src/types.ts:467](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L467)

COSE algorithm name (e.g., "EdDSA", "ES256"). `undefined` if not present.

***

### coseType

```ts
coseType: CoseType;
```

Defined in: [src/types.ts:473](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L473)

COSE structure type.

***

### expiresAt?

```ts
optional expiresAt: number;
```

Defined in: [src/types.ts:471](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L471)

Expiration time (Unix epoch seconds). `undefined` if not present or encrypted.

***

### issuer?

```ts
optional issuer: string;
```

Defined in: [src/types.ts:461](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L461)

Issuer from CWT claims. `undefined` if not present or encrypted.

***

### keyId?

```ts
optional keyId: Uint8Array<ArrayBufferLike>;
```

Defined in: [src/types.ts:465](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L465)

Key ID from the COSE header. `undefined` if not present.

***

### subject?

```ts
optional subject: string;
```

Defined in: [src/types.ts:463](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L463)

Subject from CWT claims. `undefined` if not present or encrypted.

***

### x509Headers

```ts
x509Headers: X509Headers;
```

Defined in: [src/types.ts:469](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L469)

X.509 certificate headers from the COSE structure.

---

# Interface: X509Headers

Defined in: [src/types.ts:120](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L120)

X.509 headers extracted from COSE protected/unprotected headers.

These headers provide X.509 certificate information for signature verification
as defined in RFC 9360.

## Example

```typescript
const result = new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .decode();

// Check for certificate chain
if (result.x509Headers.x5chain) {
  console.log(`Certificate chain has ${result.x509Headers.x5chain.length} certificates`);
}

// Check for certificate URL
if (result.x509Headers.x5u) {
  console.log(`Certificate URL: ${result.x509Headers.x5u}`);
}
```

## Properties

### x5bag?

```ts
optional x5bag: Uint8Array<ArrayBufferLike>[];
```

Defined in: [src/types.ts:125](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L125)

x5bag (COSE label 32): Unordered bag of X.509 certificates.
Each certificate is DER-encoded.

***

### x5chain?

```ts
optional x5chain: Uint8Array<ArrayBufferLike>[];
```

Defined in: [src/types.ts:131](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L131)

x5chain (COSE label 33): Ordered chain of X.509 certificates.
The first certificate contains the public key used for verification.
Each certificate is DER-encoded.

***

### x5t?

```ts
optional x5t: CertificateHash;
```

Defined in: [src/types.ts:136](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L136)

x5t (COSE label 34): Certificate thumbprint hash.
Used to identify the certificate by its hash.

***

### x5u?

```ts
optional x5u: string;
```

Defined in: [src/types.ts:141](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L141)

x5u (COSE label 35): URI pointing to an X.509 certificate.
Can be used to fetch the certificate for verification.

---

# claim169/types

Type definitions for MOSIP Claim 169 QR Code decoder.

This module contains TypeScript interfaces and types for the decoded
Claim 169 identity data.

## Classes

- Claim169Error

## Interfaces

- Biometric
- CertificateHash
- Claim169
- Claim169Input
- CwtMeta
- CwtMetaInput
- DecodeResult
- EncodeResult
- EncodeWarning
- IDecoder
- IEncoder
- InspectResult
- X509Headers

## Type Aliases

- Algorithm
- AlgorithmName
- BiometricFormat
- Claim169ErrorCode
- CompressionMode
- CoseType
- DecryptorCallback
- DetectedCompression
- EncryptorCallback
- Gender
- MaritalStatus
- PhotoFormat
- SignerCallback
- VerificationStatus
- VerifierCallback

## Variables

- BiometricFormat
- Gender
- MaritalStatus
- PhotoFormat

---

# Type Alias: Algorithm

```ts
type Algorithm = "EdDSA" | "ES256" | "A256GCM" | "A128GCM";
```

Defined in: [src/types.ts:292](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L292)

Algorithm identifier for COSE algorithms.
- "EdDSA" - Edwards-curve Digital Signature Algorithm (Ed25519)
- "ES256" - ECDSA with P-256 and SHA-256
- "A256GCM" - AES-256-GCM encryption
- "A128GCM" - AES-128-GCM encryption

---

# Type Alias: AlgorithmName

```ts
type AlgorithmName = 
  | Algorithm
  | string & object;
```

Defined in: [src/types.ts:300](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L300)

Algorithm identifier as surfaced by the underlying WASM bindings.

This preserves autocomplete for known values while still allowing
unknown strings for forwards compatibility.

---

# Type Alias: BiometricFormat

```ts
type BiometricFormat = typeof BiometricFormat[keyof typeof BiometricFormat];
```

Defined in: [src/types.ts:570](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L570)

Biometric format code type (0=Image, 1=Template, 2=Sound, 3=BioHash)

---

# Type Alias: Claim169ErrorCode

```ts
type Claim169ErrorCode = 
  | "BASE45_DECODE"
  | "DECOMPRESS"
  | "DECOMPRESS_LIMIT"
  | "COSE_PARSE"
  | "UNSUPPORTED_COSE_TYPE"
  | "SIGNATURE_INVALID"
  | "DECRYPTION_FAILED"
  | "CBOR_PARSE"
  | "CWT_PARSE"
  | "CLAIM169_NOT_FOUND"
  | "CLAIM169_INVALID"
  | "UNSUPPORTED_ALGORITHM"
  | "KEY_NOT_FOUND"
  | "EXPIRED"
  | "NOT_YET_VALID"
  | "CRYPTO"
  | "CBOR_ENCODE"
  | "SIGNATURE_FAILED"
  | "ENCRYPTION_FAILED"
  | "ENCODING_CONFIG"
  | "DECODING_CONFIG"
  | "UNKNOWN";
```

Defined in: [src/types.ts:599](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L599)

Error code for programmatic error handling.

Maps to error types from the Rust core library pipeline stages.

---

# Type Alias: CompressionMode

```ts
type CompressionMode = 
  | "zlib"
  | "none"
  | "adaptive"
  | `brotli:${number}`
  | `adaptive-brotli:${number}`;
```

Defined in: [src/types.ts:257](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L257)

Compression mode for encoding.

- `"zlib"`: Standard zlib compression (default)
- `"none"`: No compression
- `"adaptive"`: Automatically choose best compression
- `` `brotli:${number}` ``: Brotli compression with quality level 0-11
- `` `adaptive-brotli:${number}` ``: Adaptive with Brotli at quality level 0-11

---

# Type Alias: CoseType

```ts
type CoseType = "Sign1" | "Encrypt0";
```

Defined in: [src/types.ts:452](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L452)

COSE structure type

---

# Type Alias: DecryptorCallback()

```ts
type DecryptorCallback = (algorithm, keyId, nonce, aad, ciphertext) => Uint8Array;
```

Defined in: [src/types.ts:346](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L346)

Custom decryptor callback.
Use for external crypto providers (HSM, cloud KMS, etc.)

## Parameters

### algorithm

`AlgorithmName`

COSE algorithm identifier (e.g., "A256GCM", "A128GCM")

### keyId

Optional key identifier from the COSE header

`Uint8Array` | `null`

### nonce

`Uint8Array`

Nonce/IV for decryption (12 bytes for AES-GCM)

### aad

`Uint8Array`

Additional authenticated data

### ciphertext

`Uint8Array`

Ciphertext with authentication tag

## Returns

`Uint8Array`

Decrypted plaintext

## Example

```typescript
const myDecryptor: DecryptorCallback = (algorithm, keyId, nonce, aad, ciphertext) => {
  return myKms.decrypt({ keyId, nonce, aad, ciphertext });
};
```

---

# Type Alias: DetectedCompression

```ts
type DetectedCompression = "zlib" | "brotli" | "none" | string & object;
```

Defined in: [src/types.ts:270](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L270)

Detected compression format from decoding.

Known values are `"zlib"`, `"brotli"`, or `"none"`.
Unknown formats are preserved as strings for forward compatibility.

---

# Type Alias: EncryptorCallback()

```ts
type EncryptorCallback = (algorithm, keyId, nonce, aad, plaintext) => Uint8Array;
```

Defined in: [src/types.ts:394](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L394)

Custom encryptor callback.
Use for external crypto providers (HSM, cloud KMS, etc.)

## Parameters

### algorithm

`AlgorithmName`

COSE algorithm identifier (e.g., "A256GCM", "A128GCM")

### keyId

Optional key identifier

`Uint8Array` | `null`

### nonce

`Uint8Array`

Nonce/IV for encryption (12 bytes for AES-GCM)

### aad

`Uint8Array`

Additional authenticated data

### plaintext

`Uint8Array`

Data to encrypt

## Returns

`Uint8Array`

Ciphertext with authentication tag

## Example

```typescript
const myEncryptor: EncryptorCallback = (algorithm, keyId, nonce, aad, plaintext) => {
  return myKms.encrypt({ keyId, nonce, aad, plaintext });
};
```

---

# Type Alias: Gender

```ts
type Gender = typeof Gender[keyof typeof Gender];
```

Defined in: [src/types.ts:525](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L525)

Gender code type (1=Male, 2=Female, 3=Other)

---

# Type Alias: MaritalStatus

```ts
type MaritalStatus = typeof MaritalStatus[keyof typeof MaritalStatus];
```

Defined in: [src/types.ts:537](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L537)

Marital status code type (1=Unmarried, 2=Married, 3=Divorced)

---

# Type Alias: PhotoFormat

```ts
type PhotoFormat = typeof PhotoFormat[keyof typeof PhotoFormat];
```

Defined in: [src/types.ts:553](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L553)

Photo format code type (1=JPEG, 2=JPEG2000, 3=AVIF, 4=WebP)

---

# Type Alias: SignerCallback()

```ts
type SignerCallback = (algorithm, keyId, data) => Uint8Array;
```

Defined in: [src/types.ts:370](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L370)

Custom signer callback.
Use for external crypto providers (HSM, cloud KMS, remote signing, etc.)

## Parameters

### algorithm

`AlgorithmName`

COSE algorithm identifier (e.g., "EdDSA", "ES256")

### keyId

Optional key identifier

`Uint8Array` | `null`

### data

`Uint8Array`

Data to sign (the COSE Sig_structure)

## Returns

`Uint8Array`

Signature bytes

## Example

```typescript
const mySigner: SignerCallback = (algorithm, keyId, data) => {
  return myKms.sign({ keyId, algorithm, data });
};
```

---

# Type Alias: VerificationStatus

```ts
type VerificationStatus = "verified" | "skipped" | "failed";
```

Defined in: [src/types.ts:279](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L279)

Signature verification status of the decoded credential.

- `"verified"`: Signature was verified successfully with the provided public key
- `"skipped"`: Verification was explicitly skipped using `allowUnverified()`
- `"failed"`: Signature verification failed (invalid signature or wrong key)

---

# Type Alias: VerifierCallback()

```ts
type VerifierCallback = (algorithm, keyId, data, signature) => void;
```

Defined in: [src/types.ts:321](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L321)

Custom signature verifier callback.
Use for external crypto providers (HSM, cloud KMS, remote signing, etc.)

The callback should throw an error if verification fails.

## Parameters

### algorithm

`AlgorithmName`

COSE algorithm identifier (e.g., "EdDSA", "ES256")

### keyId

Optional key identifier from the COSE header

`Uint8Array` | `null`

### data

`Uint8Array`

Data that was signed (the COSE Sig_structure)

### signature

`Uint8Array`

Signature to verify

## Returns

`void`

## Example

```typescript
const myVerifier: VerifierCallback = (algorithm, keyId, data, signature) => {
  const result = myKms.verify({ keyId, algorithm, data, signature });
  if (!result.valid) throw new Error("Verification failed");
};
```

---

# Variable: BiometricFormat

```ts
const BiometricFormat: object;
```

Defined in: [src/types.ts:570](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L570)

Biometric data format values as defined in MOSIP Claim 169 (0-indexed).

## Type Declaration

### BioHash

```ts
readonly BioHash: 3 = 3;
```

### Image

```ts
readonly Image: 0 = 0;
```

### Sound

```ts
readonly Sound: 2 = 2;
```

### Template

```ts
readonly Template: 1 = 1;
```

## Example

```typescript
if (biometric.format === BiometricFormat.Image) { ... }
```

---

# Variable: Gender

```ts
const Gender: object;
```

Defined in: [src/types.ts:525](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L525)

Gender values as defined in MOSIP Claim 169 (1-indexed).

## Type Declaration

### Female

```ts
readonly Female: 2 = 2;
```

### Male

```ts
readonly Male: 1 = 1;
```

### Other

```ts
readonly Other: 3 = 3;
```

## Example

```typescript
if (claim.gender === Gender.Female) { ... }
```

---

# Variable: MaritalStatus

```ts
const MaritalStatus: object;
```

Defined in: [src/types.ts:537](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L537)

Marital status values as defined in MOSIP Claim 169 (1-indexed).

## Type Declaration

### Divorced

```ts
readonly Divorced: 3 = 3;
```

### Married

```ts
readonly Married: 2 = 2;
```

### Unmarried

```ts
readonly Unmarried: 1 = 1;
```

## Example

```typescript
if (claim.maritalStatus === MaritalStatus.Married) { ... }
```

---

# Variable: PhotoFormat

```ts
const PhotoFormat: object;
```

Defined in: [src/types.ts:553](https://github.com/jeremi/claim-169/blob/dd0d92962d1f8c89290080bdf695d1304de8a76f/sdks/typescript/src/types.ts#L553)

Photo format values as defined in MOSIP Claim 169 (1-indexed).

## Type Declaration

### Avif

```ts
readonly Avif: 3 = 3;
```

### Jpeg

```ts
readonly Jpeg: 1 = 1;
```

### Jpeg2000

```ts
readonly Jpeg2000: 2 = 2;
```

### Webp

```ts
readonly Webp: 4 = 4;
```

## Example

```typescript
if (claim.photoFormat === PhotoFormat.Jpeg) { ... }
```

---

# claim169

## Modules

- claim169
- claim169/types

---

# claim169

> **Alpha Software**: This library is under active development. APIs may change without notice. Not recommended for production use without thorough testing.

[![npm](https://img.shields.io/npm/v/claim169.svg)](https://www.npmjs.com/package/claim169)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A TypeScript/JavaScript library for decoding MOSIP Claim 169 QR codes. Built on Rust/WebAssembly for performance and security.

## Installation

```bash
npm install claim169
```

## Overview

MOSIP Claim 169 defines a standard for encoding identity data in QR codes using:
- CBOR encoding with numeric keys for compactness
- CWT (CBOR Web Token) for standard claims
- COSE_Sign1 for digital signatures
- COSE_Encrypt0 for optional encryption
- zlib compression + Base45 encoding for QR-friendly output

## Quick Start

### Builder Pattern (Recommended)

```typescript
import { Decoder } from 'claim169';

// Decode with Ed25519 signature verification (recommended)
const publicKey = new Uint8Array(32);  // Your 32-byte public key
const result = new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .decode();

console.log(`ID: ${result.claim169.id}`);
console.log(`Name: ${result.claim169.fullName}`);
console.log(`Issuer: ${result.cwtMeta.issuer}`);
console.log(`Verified: ${result.verificationStatus}`);  // "verified"

// Decode without verification (testing only)
const result = new Decoder(qrText)
  .allowUnverified()
  .decode();
```

### `decode()` Convenience Function

> **Security note**: `decode()` requires a verification key unless you explicitly set `allowUnverified: true` (testing only). Use the `Decoder` builder API in production.

```typescript
import { decode, type DecodeOptions } from 'claim169';

// Simple decode (testing only)
const result = decode(qrText, { allowUnverified: true });

// With options
const options: DecodeOptions = {
  maxDecompressedBytes: 32768,  // 32KB limit
  skipBiometrics: true,         // Skip biometric parsing
  // Timestamp validation is enabled by default (host-side). Set to false to disable:
  validateTimestamps: false,
  allowUnverified: true,        // Explicit opt-out (testing only)
};

const result = decode(qrText, options);
```

## Decoder Class

The `Decoder` class provides a fluent builder API:

```typescript
import { Decoder } from 'claim169';

// Decode with Ed25519 verification
const result = new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .decode();

// Decode with ECDSA P-256 verification
const result = new Decoder(qrText)
  .verifyWithEcdsaP256(publicKey)  // 33 or 65 bytes SEC1 encoded
  .decode();

// Decrypt then verify (for encrypted credentials)
const result = new Decoder(qrText)
  .decryptWithAes256(aesKey)
  .verifyWithEd25519(publicKey)
  .decode();

// With additional options
const result = new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .skipBiometrics()              // Skip biometric data
  .clockSkewTolerance(60)        // 60 seconds tolerance
  .maxDecompressedBytes(32768)   // 32KB max size
  .decode();
```

### Decoder Methods

| Method | Description |
|--------|-------------|
| `verifyWithEd25519(publicKey)` | Verify with Ed25519 (32 bytes) |
| `verifyWithEcdsaP256(publicKey)` | Verify with ECDSA P-256 (33 or 65 bytes) |
| `verifyWith(callback)` | Verify with custom callback (HSM, cloud KMS, etc.) |
| `decryptWithAes256(key)` | Decrypt with AES-256-GCM (32 bytes) |
| `decryptWithAes128(key)` | Decrypt with AES-128-GCM (16 bytes) |
| `decryptWith(callback)` | Decrypt with custom callback (HSM, cloud KMS, etc.) |
| `allowUnverified()` | Skip verification (testing only) |
| `skipBiometrics()` | Skip biometric data parsing |
| `withTimestampValidation()` | Enable timestamp validation (host-side) |
| `withoutTimestampValidation()` | Disable timestamp validation |
| `clockSkewTolerance(seconds)` | Set clock skew tolerance |
| `maxDecompressedBytes(bytes)` | Set max decompressed size |
| `decode()` | Execute the decode operation |

## Encoding

The `Encoder` class creates MOSIP Claim 169 QR code data from identity information.
In production, keys should be provisioned and managed externally (HSM/KMS or secure key management). The examples below assume you already have key material.

```typescript
import { Encoder, Decoder, Claim169Input, CwtMetaInput, generateNonce } from 'claim169';

// Create identity data
const claim169: Claim169Input = {
  id: "123456789",
  fullName: "John Doe",
  dateOfBirth: "1990-01-15",
  gender: 1,  // Male
};

// Create CWT metadata
const cwtMeta: CwtMetaInput = {
  issuer: "https://issuer.example.com",
  expiresAt: 1800000000,
};

// Encode with Ed25519 signature
const privateKey = new Uint8Array(32);  // Your 32-byte private key
const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(privateKey)
  .encode();

// Encode with signature and AES-256 encryption
const aesKey = new Uint8Array(32);  // Your 32-byte AES key
const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(privateKey)
  .encryptWithAes256(aesKey)
  .encode();

// Unsigned (testing only)
const qrData = new Encoder(claim169, cwtMeta)
  .allowUnsigned()
  .encode();
```

### Encoder Methods

| Method | Description |
|--------|-------------|
| `signWithEd25519(privateKey)` | Sign with Ed25519 |
| `signWithEcdsaP256(privateKey)` | Sign with ECDSA P-256 |
| `signWith(callback, algorithm, keyId?)` | Sign with custom callback (HSM, cloud KMS, etc.) |
| `encryptWithAes256(key)` | Encrypt with AES-256-GCM |
| `encryptWithAes128(key)` | Encrypt with AES-128-GCM |
| `encryptWith(callback, algorithm)` | Encrypt with custom callback (HSM, cloud KMS, etc.) |
| `allowUnsigned()` | Allow unsigned (testing only) |
| `skipBiometrics()` | Skip biometric fields |
| `encode()` | Produce the QR string |

### generateNonce()

Generate a cryptographically secure random nonce for encryption:

```typescript
import { generateNonce } from 'claim169';

const nonce = generateNonce();  // Returns 12-byte Uint8Array
```

## Custom Crypto Providers

For integrating with external key management systems like HSMs, cloud KMS (AWS KMS, Google Cloud KMS, Azure Key Vault), smart cards, TPMs, or remote signing services, use the custom callback methods.

### Custom Signer

```typescript
import { Encoder, SignerCallback, Claim169Input, CwtMetaInput } from 'claim169';

// Example: Sign with a cloud KMS
const mySigner: SignerCallback = (algorithm, keyId, data) => {
  // Call your crypto provider
  // algorithm: "EdDSA" or "ES256"
  // keyId: optional key identifier (Uint8Array or null)
  // data: the COSE Sig_structure to sign (Uint8Array)
  const signature = myKms.sign({ keyId, data, algorithm });
  return signature;  // Uint8Array: 64 bytes for EdDSA, 64 bytes for ES256
};

const claim: Claim169Input = { id: "123", fullName: "John Doe" };
const meta: CwtMetaInput = { issuer: "https://issuer.example" };

const qrData = new Encoder(claim, meta)
  .signWith(mySigner, "EdDSA", new Uint8Array([1, 2, 3])) // optional keyId
  .encode();
```

### Custom Verifier

```typescript
import { Decoder, VerifierCallback } from 'claim169';

// Example: Verify with an HSM
const myVerifier: VerifierCallback = (algorithm, keyId, data, signature) => {
  // Call your crypto provider
  // Throw an error if verification fails
  const result = myHsm.verify({ keyId, data, signature, algorithm });
  if (!result.valid) {
    throw new Error("Signature verification failed");
  }
};

const result = new Decoder(qrText)
  .verifyWith(myVerifier)
  .decode();
```

### Custom Encryptor

```typescript
import { Encoder, EncryptorCallback } from 'claim169';

// Example: Encrypt with cloud KMS
const myEncryptor: EncryptorCallback = (algorithm, keyId, nonce, aad, plaintext) => {
  // algorithm: "A256GCM" or "A128GCM"
  // nonce: 12-byte IV
  // aad: additional authenticated data
  // plaintext: data to encrypt
  const ciphertext = myKms.encrypt({ keyId, nonce, aad, plaintext });
  return ciphertext;  // Uint8Array: plaintext + 16-byte auth tag
};

const qrData = new Encoder(claim, meta)
  .signWithEd25519(signingKey)
  .encryptWith(myEncryptor, "A256GCM")
  .encode();
```

### Custom Decryptor

```typescript
import { Decoder, DecryptorCallback } from 'claim169';

// Example: Decrypt with cloud KMS
const myDecryptor: DecryptorCallback = (algorithm, keyId, nonce, aad, ciphertext) => {
  // algorithm: "A256GCM" or "A128GCM"
  // ciphertext includes the auth tag
  const plaintext = myKms.decrypt({ keyId, nonce, aad, ciphertext });
  return plaintext;  // Uint8Array
};

const result = new Decoder(qrText)
  .decryptWith(myDecryptor)
  .verifyWithEd25519(publicKey)
  .decode();
```

### Callback Type Definitions

```typescript
// Signer: (algorithm, keyId, data) => signature
type SignerCallback = (
  algorithm: string,
  keyId: Uint8Array | null,
  data: Uint8Array
) => Uint8Array;

// Verifier: (algorithm, keyId, data, signature) => void (throw on failure)
type VerifierCallback = (
  algorithm: string,
  keyId: Uint8Array | null,
  data: Uint8Array,
  signature: Uint8Array
) => void;

// Encryptor: (algorithm, keyId, nonce, aad, plaintext) => ciphertext
type EncryptorCallback = (
  algorithm: string,
  keyId: Uint8Array | null,
  nonce: Uint8Array,
  aad: Uint8Array,
  plaintext: Uint8Array
) => Uint8Array;

// Decryptor: (algorithm, keyId, nonce, aad, ciphertext) => plaintext
type DecryptorCallback = (
  algorithm: string,
  keyId: Uint8Array | null,
  nonce: Uint8Array,
  aad: Uint8Array,
  ciphertext: Uint8Array
) => Uint8Array;
```

## Data Model

### DecodeResult

```typescript
interface DecodeResult {
  claim169: Claim169;                    // Identity data
  cwtMeta: CwtMeta;                      // Token metadata
  verificationStatus: VerificationStatus; // "verified" | "skipped" | "failed"
}
```

### Claim169

```typescript
interface Claim169 {
  // Demographics
  id?: string;                  // Unique identifier
  fullName?: string;            // Full name
  firstName?: string;           // First name
  middleName?: string;          // Middle name
  lastName?: string;            // Last name
  dateOfBirth?: string;         // ISO 8601 format
  gender?: number;              // 1=Male, 2=Female, 3=Other
  address?: string;             // Address
  email?: string;               // Email address
  phone?: string;               // Phone number
  nationality?: string;         // Nationality code
  maritalStatus?: number;       // Marital status code
  guardian?: string;            // Guardian name

  // Additional fields
  version?: string;             // Claim version
  language?: string;            // Primary language code
  secondaryFullName?: string;   // Secondary full name
  secondaryLanguage?: string;   // Secondary language code
  locationCode?: string;        // Location code
  legalStatus?: string;         // Legal status
  countryOfIssuance?: string;   // Country of issuance

  // Photo
  photo?: Uint8Array;           // Photo data
  photoFormat?: number;         // Photo format code

  // Biometrics (when present)
  face?: Biometric[];           // Face biometrics
  rightThumb?: Biometric[];     // Right thumb fingerprint
  // ... (all finger/iris/palm biometrics)
}
```

### CwtMeta

```typescript
interface CwtMeta {
  issuer?: string;              // Token issuer
  subject?: string;             // Token subject
  expiresAt?: number;           // Expiration timestamp (Unix seconds)
  notBefore?: number;           // Not-before timestamp
  issuedAt?: number;            // Issued-at timestamp
}
```

### Biometric

```typescript
interface Biometric {
  data: Uint8Array;             // Raw biometric data
  format: number;               // Biometric format code
  subFormat?: number;           // Sub-format code
  issuer?: string;              // Issuer identifier
}
```

## Error Handling

```typescript
import { decode, Claim169Error } from 'claim169';

try {
  const result = decode(qrText, { allowUnverified: true });
} catch (error) {
  if (error instanceof Claim169Error) {
    // Handle decode error
    console.error("Decode failed:", error.message);
  }
}
```

Error messages indicate the specific failure:
- `Base45Decode`: Invalid Base45 encoding
- `Decompress`: zlib decompression failed
- `CoseParse`: Invalid COSE structure
- `CwtParse`: Invalid CWT structure
- `Claim169NotFound`: Missing claim 169
- `SignatureError`: Signature verification failed
- `DecryptionError`: Decryption failed

## Notes

### Timestamp Validation

Timestamp validation is performed in the host (JavaScript) and is enabled by default. Disable it explicitly if you intentionally want to skip time checks:

```typescript
const result = new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .withoutTimestampValidation()
  .clockSkewTolerance(60)  // Allow 60 seconds clock drift
  .decode();
```

### Secure by Default

The decoder requires explicit verification configuration. You must call one of:
- `verifyWithEd25519(publicKey)` - Verify with Ed25519
- `verifyWithEcdsaP256(publicKey)` - Verify with ECDSA P-256
- `allowUnverified()` - Explicitly skip verification (testing only)

This prevents accidentally processing unverified credentials.

## Browser Usage

```html
<script type="module">
  import { Decoder } from './node_modules/claim169/dist/index.js';

  // Your issuer's public key (32 bytes for Ed25519)
  const publicKey = new Uint8Array([/* ... */]);

  const qrText = "6BF5YZB2...";
  const result = new Decoder(qrText)
    .verifyWithEd25519(publicKey)
    .decode();

  if (result.verificationStatus === "verified") {
    console.log("Verified:", result.claim169.fullName);
  }
</script>
```

## Bundler Configuration

### Vite

```typescript
// vite.config.ts
import { defineConfig } from 'vite';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';

export default defineConfig({
  plugins: [wasm(), topLevelAwait()],
});
```

### Webpack

```javascript
// webpack.config.js
module.exports = {
  experiments: {
    asyncWebAssembly: true,
  },
};
```

## Utility Functions

```typescript
import { version, isLoaded } from 'claim169';

// Get library version
console.log(version());  // "0.1.0"

// Check if WASM module is loaded
console.log(isLoaded());  // true
```

## Development

### Building from Source

```bash
# Install dependencies
npm install

# Build WASM (requires Rust and wasm-pack)
npm run build:wasm

# Build TypeScript
npm run build:ts

# Or build everything
npm run build
```

### Running Tests

```bash
npm test
```

### Prerequisites

- Node.js 18+
- Rust toolchain (for building WASM)
- wasm-pack (`cargo install wasm-pack`)

## License

MIT License - See LICENSE file for details.
