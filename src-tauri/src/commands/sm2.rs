use sm2::{
    SecretKey,
    PublicKey,
    dsa::{SigningKey, VerifyingKey, Signature, signature::{Signer, Verifier}},
};
use smcrypto::sm2::{Encrypt, Decrypt};
use rand_core::OsRng;
use base64::{Engine as _, engine::general_purpose};

const DEFAULT_DISTID: &str = "1234567812345678";

#[derive(serde::Serialize)]
pub struct Sm2KeyPair {
    pub private_key: String,
    pub public_key: String,
    pub private_key_pem: String,
    pub public_key_pem: String,
}

#[tauri::command(rename_all = "camelCase")]
pub fn generate_sm2_keypair() -> Result<Sm2KeyPair, String> {
    let secret_key = SecretKey::random(&mut OsRng);
    let public_key = secret_key.public_key();
    
    let private_key_bytes = secret_key.to_bytes();
    let public_key_bytes = public_key.to_sec1_bytes();
    
    let private_key_hex = hex::encode(&private_key_bytes);
    let public_key_hex = hex::encode(&public_key_bytes);
    
    let private_key_pem = format!(
        "-----BEGIN EC PRIVATE KEY-----\n{}\n-----END EC PRIVATE KEY-----",
        general_purpose::STANDARD.encode(&private_key_bytes)
    );
    
    let public_key_pem = format!(
        "-----BEGIN PUBLIC KEY-----\n{}\n-----END PUBLIC KEY-----",
        general_purpose::STANDARD.encode(&public_key_bytes)
    );
    
    eprintln!("[SM2 KeyGen] 私钥: {}", private_key_hex);
    eprintln!("[SM2 KeyGen] 公钥: {}", public_key_hex);
    
    Ok(Sm2KeyPair {
        private_key: private_key_hex,
        public_key: public_key_hex,
        private_key_pem,
        public_key_pem,
    })
}

#[tauri::command(rename_all = "camelCase")]
pub fn sm2_encrypt(plaintext: String, public_key: String, cipher_mode: Option<String>) -> Result<String, String> {
    let public_key_bytes = hex::decode(&public_key).map_err(|e| format!("Invalid public key hex: {}", e))?;
    let public_key_hex = hex::encode(&public_key_bytes);
    
    let encrypt_ctx = Encrypt::new(&public_key_hex);
    let encrypted = match cipher_mode.as_deref() {
        Some("c1c2c3") => encrypt_ctx.encrypt_c1c2c3(plaintext.as_bytes()),
        Some("asn1") => encrypt_ctx.encrypt_asna1(plaintext.as_bytes()),
        _ => encrypt_ctx.encrypt(plaintext.as_bytes()),
    };
    
    Ok(hex::encode(encrypted))
}

#[tauri::command(rename_all = "camelCase")]
pub fn sm2_decrypt(ciphertext: String, private_key: String, cipher_mode: Option<String>, is_base64: Option<bool>, add_prefix: Option<bool>, output_format: Option<String>) -> Result<String, String> {
    let private_key_bytes = hex::decode(&private_key).map_err(|e| format!("Invalid private key hex: {}", e))?;
    
    if private_key_bytes.len() != 32 {
        return Err(format!("私钥长度错误: 期望 32 字节，实际 {} 字节", private_key_bytes.len()));
    }
    
    let private_key_hex = hex::encode(&private_key_bytes);
    
    let encrypted_bytes = if is_base64.unwrap_or(false) {
        general_purpose::STANDARD.decode(&ciphertext)
            .map_err(|e| format!("Invalid base64 ciphertext: {}", e))?
    } else {
        hex::decode(&ciphertext).map_err(|e| format!("Invalid hex ciphertext: {}", e))?
    };
    
    eprintln!("[SM2 Decrypt] 私钥长度: {} 字节", private_key_bytes.len());
    eprintln!("[SM2 Decrypt] 密文长度: {} 字节", encrypted_bytes.len());
    eprintln!("[SM2 Decrypt] 密文模式: {:?}", cipher_mode);
    eprintln!("[SM2 Decrypt] Base64: {:?}", is_base64);
    eprintln!("[SM2 Decrypt] 密文前16字节: {}", hex::encode(&encrypted_bytes[..encrypted_bytes.len().min(16)]));
    
    // Java 兼容：根据参数决定是否添加 "04" 前缀
    let encrypted_hex = hex::encode(&encrypted_bytes);
    let mut encrypted_bytes = encrypted_bytes;
    if add_prefix.unwrap_or(false) && !encrypted_hex.starts_with("04") {
        eprintln!("[SM2 Decrypt] Java 兼容模式：密文不以 04 开头，添加 04 前缀");
        let mut prefixed = vec![0x04u8];
        prefixed.extend_from_slice(&encrypted_bytes);
        encrypted_bytes = prefixed;
    }
    
    let decrypt_ctx = Decrypt::new(&private_key_hex);
    let decrypted = match cipher_mode.as_deref() {
        Some("c1c2c3") => decrypt_ctx.decrypt_c1c2c3(&encrypted_bytes),
        Some("asn1") => decrypt_ctx.decrypt_asna1(&encrypted_bytes),
        Some("c1c3c2") => decrypt_ctx.decrypt(&encrypted_bytes),
        _ => decrypt_ctx.decrypt(&encrypted_bytes), // 默认使用 C1C3C2 模式
    };
    
    eprintln!("[SM2 Decrypt] 解密结果长度: {} 字节", decrypted.len());
    
    if decrypted.is_empty() {
        return Err("解密失败：密文格式错误或密钥不匹配。请检查：1.密文格式(C1C3C2/C1C2C3/ASN.1) 2.密钥对是否匹配 3.密文是否完整".to_string());
    }
    
    eprintln!("[SM2 Decrypt] 解密结果前16字节: {}", hex::encode(&decrypted[..decrypted.len().min(16)]));
    
    let output_format = output_format.unwrap_or_else(|| "auto".to_string());
    
    match output_format.as_str() {
        "hex" => {
            let hex_result = hex::encode(&decrypted);
            eprintln!("[SM2 Decrypt] 输出格式: Hex, 结果: {}", hex_result);
            Ok(hex_result)
        },
        "base64" => {
            let base64_result = general_purpose::STANDARD.encode(&decrypted);
            eprintln!("[SM2 Decrypt] 输出格式: Base64, 结果: {}", base64_result);
            Ok(base64_result)
        },
        "text" | "auto" => {
            match String::from_utf8(decrypted.clone()) {
                Ok(text) => {
                    eprintln!("[SM2 Decrypt] 输出格式: Text, 结果: {}", text);
                    Ok(text)
                },
                Err(e) => {
                    eprintln!("[SM2 Decrypt] UTF-8 转换失败: {}", e);
                    eprintln!("[SM2 Decrypt] 解密结果(Hex): {}", hex::encode(&decrypted));
                    
                    // UTF-8 转换失败，使用 Java byte[] 方式输出
                    let hex_result = hex::encode(&decrypted);
                    let base64_result = general_purpose::STANDARD.encode(&decrypted);
                    
                    // 生成 Java byte[] 数组格式的字符串
                    let java_byte_array: Vec<String> = decrypted.iter()
                        .map(|b| {
                            // Java 的 byte 是有符号的 (-128 到 127)，需要转换
                            let signed_byte = if *b > 127 { *b as i32 - 256 } else { *b as i32 };
                            format!("{}", signed_byte)
                        })
                        .collect();
                    let java_byte_array_str = format!("[{}]", java_byte_array.join(", "));
                    
                    eprintln!("[SM2 Decrypt] Java byte[] 格式: {}", java_byte_array_str);
                    
                    if output_format == "text" {
                        Err(format!(
                            "解密后数据不是有效的UTF-8文本。\n\n可用的输出格式：\n1. Hex: {}\n2. Base64: {}\n3. Java byte[]: {}\n\n请使用 output_format='hex' 或 'base64' 来获取二进制数据。",
                            hex_result, base64_result, java_byte_array_str
                        ))
                    } else {
                        eprintln!("[SM2 Decrypt] 自动切换到 Java byte[] 兼容格式 (Hex)");
                        Ok(format!(
                            "[JAVA_BYTE_ARRAY]\nHex: {}\nBase64: {}\nArray: {}",
                            hex_result, base64_result, java_byte_array_str
                        ))
                    }
                }
            }
        },
        _ => {
            Err(format!("不支持的输出格式: {}。支持的格式: auto, text, hex, base64", output_format))
        }
    }
}

#[tauri::command(rename_all = "camelCase")]
pub fn sm2_sign(message: String, private_key: String) -> Result<String, String> {
    let private_key_bytes = hex::decode(&private_key).map_err(|e| format!("Invalid private key hex: {}", e))?;
    let private_key_array: [u8; 32] = private_key_bytes.try_into()
        .map_err(|_| "Invalid private key length, expected 32 bytes")?;
    let sk = SecretKey::from_bytes(&private_key_array.into())
        .map_err(|e| format!("Invalid private key: {}", e))?;
    
    let signing_key = SigningKey::new(DEFAULT_DISTID, &sk)
        .map_err(|e| format!("Failed to create signing key: {}", e))?;
    
    let signature: Signature = signing_key.sign(message.as_bytes());
    
    Ok(hex::encode(signature.to_bytes()))
}

#[tauri::command(rename_all = "camelCase")]
pub fn sm2_verify(message: String, signature_hex: String, public_key: String) -> Result<bool, String> {
    let public_key_bytes = hex::decode(&public_key).map_err(|e| format!("Invalid public key hex: {}", e))?;
    let pk = PublicKey::from_sec1_bytes(&public_key_bytes)
        .map_err(|e| format!("Invalid public key: {}", e))?;
    
    let signature_bytes = hex::decode(&signature_hex).map_err(|e| format!("Invalid signature hex: {}", e))?;
    let signature_bytes_array: [u8; 64] = signature_bytes.try_into()
        .map_err(|_| "Invalid signature length, expected 64 bytes")?;
    let signature = Signature::from_bytes(&signature_bytes_array)
        .map_err(|e| format!("Invalid signature: {}", e))?;
    
    let verifying_key = VerifyingKey::new(DEFAULT_DISTID, pk)
        .map_err(|e| format!("Failed to create verifying key: {}", e))?;
    
    let result = verifying_key.verify(message.as_bytes(), &signature).is_ok();
    
    Ok(result)
}