# CyberCryptoLibs

## Symmetric Cryptography

### Introduction
Symmetric cryptography, also known as Symmetric-key encryption, are algorithms that uses the same cryptographic key to encrypt and then decrypt some data. 

## Block cipher mode of operation
The **block cipher mode of operation** does use the **block cipher algorithm** without modifying it. The mode of operation allows to get some additional capabilities in a symmetric algorithm (like integrity and authenticity). 

There are several type of mode of operation:
- AEAD - Authenticated Encryption with Additional Data
- Confidential only modes

Here are some of known "confiential-only" block cipher modes of operation:
|  Block Cipher Mode of Operation           |  Type of mode                         |  Examples of use                 |
|-------------------------------------------|---------------------------------------|----------------------------------|
| CBC - Cipher Block Chaining               |  Confidential Only                    |  TLSv1.2                         |
| CTR - CounTeR                             |  Confidential Only                    |  -                               |
| ECB - Electronic Codebook Block           |  Confidential Only                    | **INSECURE!!!**                  |


A "confidential only" mode can be associated with a Message Authentication Code (MAC) mechanism to make it support integrity check. There mainly exists 3 types of MAC:



Then, "Authenticated Encryption with Additional Data" (AEAD) mode can be used. This mode assure confidentiality and authenticity of data.

| Block Cipher Mode of Operation            |  Type of mode                         |  Examples of use                 |
|-------------------------------------------|---------------------------------------|----------------------------------|
| GCM - Galois/Counter Mode                 |  AEAD                                 |  TLSv1.3                         |
| CCM - Counter(CTR) and CBC-MAC            |  AEAD                                 |  TLSv1.3                         |



