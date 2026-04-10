use sm2::{
    SecretKey,
    PublicKey,
    dsa::{SigningKey, VerifyingKey, Signature, signature::{Signer, Verifier}},
};
use smcrypto::sm2::{Encrypt, Decrypt};
use rand_core::OsRng;
use base64::{Engine as _, engine::general_purpose};

const DEFAULT_DISTID: &str = "1234567812345678";

#[tauri::command(rename_all = "camelCase")]
pub fn sm2_encrypt(plaintext: String, public_key: String, cipher_mode: Option<String>, use_gmssl: Option<bool>) -> Result<String, String> {
    let use_gmssl = use_gmssl.unwrap_or(false);
    
    if use_gmssl {
        // 使用 gmssl 加密
        sm2_encrypt_gmssl(plaintext, public_key, cipher_mode)
    } else {
        // 使用 smcrypto 加密
        sm2_encrypt_smcrypto(plaintext, public_key, cipher_mode)
    }
}

#[tauri::command(rename_all = "camelCase")]
pub fn sm2_decrypt(ciphertext: String, private_key: String, cipher_mode: Option<String>, is_base64: Option<bool>, use_gmssl: Option<bool>) -> Result<String, String> {
    let use_gmssl = use_gmssl.unwrap_or(false);
    
    if use_gmssl {
        // 使用 gmssl 解密
        sm2_decrypt_gmssl(ciphertext, private_key, cipher_mode, is_base64)
    } else {
        // 使用 smcrypto 解密
        sm2_decrypt_smcrypto(ciphertext, private_key, cipher_mode, is_base64)
    }
}

// smcrypto 实现
fn sm2_encrypt_smcrypto(plaintext: String, public_key: String, cipher_mode: Option<String>) -> Result<String, String> {
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

fn sm2_decrypt_smcrypto(ciphertext: String, private_key: String, cipher_mode: Option<String>, is_base64: Option<bool>) -> Result<String, String> {
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
    
    let decrypt_ctx = Decrypt::new(&private_key_hex);
    let decrypted = match cipher_mode.as_deref() {
        Some("c1c2c3") => decrypt_ctx.decrypt_c1c2c3(&encrypted_bytes),
        Some("asn1") => decrypt_ctx.decrypt_asna1(&encrypted_bytes),
        _ => decrypt_ctx.decrypt(&encrypted_bytes),
    };
    
    if decrypted.is_empty() {
        return Err("解密失败：密文格式错误或密钥不匹配".to_string());
    }
    
    String::from_utf8(decrypted).map_err(|e| format!("解密后数据不是有效 UTF-8: {}", e))
}

// gmssl 实现 (使用 smcrypto 的 ASN.1 模式，更接近标准)
fn sm2_encrypt_gmssl(plaintext: String, public_key: String, cipher_mode: Option<String>) -> Result<String, String> {
    let public_key_bytes = hex::decode(&public_key).map_err(|e| format!("Invalid public key hex: {}", e))?;
    let public_key_hex = hex::encode(&public_key_bytes);
    
    let encrypt_ctx = Encrypt::new(&public_key_hex);
    // gmssl 模式使用 ASN.1 编码，更符合标准
    let encrypted = encrypt_ctx.encrypt_asna1(plaintext.as_bytes());
    
    Ok(hex::encode(encrypted))
}

fn sm2_decrypt_gmssl(ciphertext: String, private_key: String, cipher_mode: Option<String>, is_base64: Option<bool>) -> Result<String, String> {
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
    
    // gmssl 模式使用 ASN.1 解密
    let decrypt_ctx = Decrypt::new(&private_key_hex);
    let decrypted = decrypt_ctx.decrypt_asna1(&encrypted_bytes);
    
    if decrypted.is_empty() {
        return Err("解密失败：密文格式错误或密钥不匹配".to_string());
    }
    
    String::from_utf8(decrypted).map_err(|e| format!("解密后数据不是有效 UTF-8: {}", e))
}

#[tauri::command(rename_all = "camelCase")]
pub fn test_java_sm2_decrypt() -> Result<String, String> {
    // 使用你提供的数据
    let java_ciphertext_base64 = "BIXxtC/yoo5CqjEZFCsWf10CBdYTwgy4j3V1nVkoMkwQFX5BjdZSDfH1iE90c7VF81gPd10lg4t3d2RqMzpBvW3jEVdcJtxhNTpnKDQAbSIKi5sfO0+vB6m0IMBY8uIRoL485wW9Amm9hWUxfMpWiP2Fgg==";
    let private_key_upper = "7FB4065021676E4EBD94926F3D0A59C4167C0173A8DF4145778D3867F1C31197";
    let private_key_lower = "7fb4065021676e4ebd94926f3d0a59c4167c0173a8df4145778d3867f1c31197";
    let public_key = "049b11aae9c43ae851e3e4e6144be257e096f7d857458e5e5c426258dbde1a6126391b4c3604254876c1ae9b361b21746a1eeddcd230497158aa6e28f79e8aed35";
    
    eprintln!("\n========== 测试 Java SM2 解密 ==========");
    eprintln!("公钥: {}", public_key);
    eprintln!("私钥(大写): {}", private_key_upper);
    eprintln!("私钥(小写): {}", private_key_lower);
    eprintln!("密文(Base64): {}", java_ciphertext_base64);
    
    // 解析密文
    let encrypted_bytes = general_purpose::STANDARD.decode(java_ciphertext_base64)
        .map_err(|e| format!("Base64 解码失败: {}", e))?;
    eprintln!("\n密文字节长度: {}", encrypted_bytes.len());
    
    // 测试大写私钥
    eprintln!("\n--- 使用大写私钥测试 ---");
    let decrypt_ctx_upper = Decrypt::new(private_key_upper);
    let decrypted_upper = decrypt_ctx_upper.decrypt(&encrypted_bytes);
    eprintln!("大写私钥解密结果长度: {}", decrypted_upper.len());
    eprintln!("大写私钥解密结果(Hex): {}", hex::encode(&decrypted_upper));
    if let Ok(text) = String::from_utf8(decrypted_upper.clone()) {
        eprintln!("✓ 大写私钥解密成功: {}", text);
    } else {
        eprintln!("✗ 大写私钥解密失败：不是有效 UTF-8");
    }
    
    // 测试小写私钥
    eprintln!("\n--- 使用小写私钥测试 ---");
    let decrypt_ctx_lower = Decrypt::new(private_key_lower);
    let decrypted_lower = decrypt_ctx_lower.decrypt(&encrypted_bytes);
    eprintln!("小写私钥解密结果长度: {}", decrypted_lower.len());
    eprintln!("小写私钥解密结果(Hex): {}", hex::encode(&decrypted_lower));
    if let Ok(text) = String::from_utf8(decrypted_lower.clone()) {
        eprintln!("✓ 小写私钥解密成功: {}", text);
    } else {
        eprintln!("✗ 小写私钥觥密失败：不是有效 UTF-8");
    }
    
    // 比较两种私钥的解密结果
    eprintln!("\n--- 比较结果 ---");
    eprintln!("大写和小写解密结果相同: {}", decrypted_upper == decrypted_lower);
    
    // 测试 Rust 自己加密解密
    eprintln!("\n--- 测试 Rust 自身加密解密 ---");
    let test_message = b"Hello, SM2!";
    eprintln!("原始消息: {}", String::from_utf8_lossy(test_message));
    
    let encrypt_ctx = Encrypt::new(public_key);
    let encrypted = encrypt_ctx.encrypt(test_message);
    eprintln!("Rust 加密结果(Hex): {}", hex::encode(&encrypted));
    
    let decrypt_ctx3 = Decrypt::new(private_key_lower);
    let decrypted = decrypt_ctx3.decrypt(&encrypted);
    eprintln!("Rust 解密结果(Hex): {}", hex::encode(&decrypted));
    if let Ok(text) = String::from_utf8(decrypted) {
        eprintln!("✓ Rust 解密成功: {}", text);
    } else {
        eprintln!("✗ Rust 解密失败");
    }
    
    eprintln!("\n==========================================\n");
    
    if let Ok(text) = String::from_utf8(decrypted_lower.clone()) {
        Ok(format!("解密成功！明文: {}", text))
    } else {
        Ok("大小写私钥都无法解密。smcrypto 库可能与 Java Bouncy Castle 不兼容。".to_string())
    }
}

#[derive(serde::Serialize)]
pub struct Sm2KeyPair {
    pub private_key: String,
    pub public_key: String,
    pub private_key_pem: String,
    pub public_key_pem: String,
}

#[tauri::command(rename_all = "camelCase")]
pub fn generate_sm2_keypair() -> Result<Sm2KeyPair, String> {
    // 生成 SM2 密钥对
    let secret_key = SecretKey::random(&mut OsRng);
    let public_key = secret_key.public_key();
    
    // 转换为十六进制字符串
    let private_key_bytes = secret_key.to_bytes();
    let public_key_bytes = public_key.to_sec1_bytes();
    
    let private_key_hex = hex::encode(&private_key_bytes);
    let public_key_hex = hex::encode(&public_key_bytes);
    
    // 生成 PEM 格式
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
    // 从十六进制字符串解析公钥
    let public_key_bytes = hex::decode(&public_key).map_err(|e| format!("Invalid public key hex: {}", e))?;
    let public_key_hex = hex::encode(&public_key_bytes);
    
    // 使用 smcrypto 进行加密
    let encrypt_ctx = Encrypt::new(&public_key_hex);
    let encrypted = match cipher_mode.as_deref() {
        Some("c1c2c3") => encrypt_ctx.encrypt_c1c2c3(plaintext.as_bytes()),
        Some("asn1") => encrypt_ctx.encrypt_asna1(plaintext.as_bytes()),
        _ => encrypt_ctx.encrypt(plaintext.as_bytes()), // 默认 C1C3C2
    };
    
    Ok(hex::encode(encrypted))
}

#[tauri::command(rename_all = "camelCase")]
pub fn sm2_decrypt(ciphertext: String, private_key: String, cipher_mode: Option<String>, is_base64: Option<bool>) -> Result<String, String> {
    // 从十六进制字符串解析私钥
    let private_key_bytes = hex::decode(&private_key).map_err(|e| format!("Invalid private key hex: {}", e))?;
    
    if private_key_bytes.len() != 32 {
        return Err(format!("私钥长度错误: 期望 32 字节，实际 {} 字节", private_key_bytes.len()));
    }
    
    let private_key_hex = hex::encode(&private_key_bytes);
    
    // 解析密文（支持 Hex 和 Base64）
    let encrypted_bytes = if is_base64.unwrap_or(false) {
        use base64::{Engine as _, engine::general_purpose};
        general_purpose::STANDARD.decode(&ciphertext)
            .map_err(|e| format!("Invalid base64 ciphertext: {}", e))?
    } else {
        hex::decode(&ciphertext).map_err(|e| format!("Invalid hex ciphertext: {}", e))?
    };
    
    // 输出调试信息
    eprintln!("[SM2 Decrypt] 私钥长度: {} 字节", private_key_bytes.len());
    eprintln!("[SM2 Decrypt] 密文长度: {} 字节", encrypted_bytes.len());
    eprintln!("[SM2 Decrypt] 密文模式: {:?}", cipher_mode);
    eprintln!("[SM2 Decrypt] Base64: {:?}", is_base64);
    
    // 使用 smcrypto 进行解密，根据模式选择解密方法
    let decrypt_ctx = Decrypt::new(&private_key_hex);
    let decrypted = match cipher_mode.as_deref() {
        Some("c1c2c3") => decrypt_ctx.decrypt_c1c2c3(&encrypted_bytes),
        Some("asn1") => decrypt_ctx.decrypt_asna1(&encrypted_bytes),
        _ => decrypt_ctx.decrypt(&encrypted_bytes), // 默认 C1C3C2
    };
    
    // 检查解密结果
    if decrypted.is_empty() {
        return Err("解密失败：密文格式错误或密钥不匹配。请检查：1.密文格式(C1C3C2/C1C2C3) 2.密钥对是否匹配".to_string());
    }
    
    // 尝试 UTF-8 转换
    match String::from_utf8(decrypted.clone()) {
        Ok(text) => Ok(text),
        Err(e) => {
            // 输出前 16 字节的 hex 用于调试
            let preview: Vec<String> = decrypted.iter().take(16).map(|b| format!("{:02x}", b)).collect();
            Err(format!("解密后数据不是有效 UTF-8。前16字节: {}。可能原因：1.密钥不匹配 2.密文损坏 3.编码格式错误", preview.join(" ")))
        }
    }
}

#[tauri::command(rename_all = "camelCase")]
pub fn sm2_sign(message: String, private_key: String) -> Result<String, String> {
    // 从十六进制字符串解析私钥
    let private_key_bytes = hex::decode(&private_key).map_err(|e| format!("Invalid private key hex: {}", e))?;
    let private_key_array: [u8; 32] = private_key_bytes.try_into()
        .map_err(|_| "Invalid private key length, expected 32 bytes")?;
    let sk = SecretKey::from_bytes(&private_key_array.into())
        .map_err(|e| format!("Invalid private key: {}", e))?;
    
    // 创建签名密钥
    let signing_key = SigningKey::new(DEFAULT_DISTID, &sk)
        .map_err(|e| format!("Failed to create signing key: {}", e))?;
    
    // 签名
    let signature: Signature = signing_key.sign(message.as_bytes());
    
    Ok(hex::encode(signature.to_bytes()))
}

#[tauri::command(rename_all = "camelCase")]
pub fn sm2_verify(message: String, signature_hex: String, public_key: String) -> Result<bool, String> {
    // 从十六进制字符串解析公钥
    let public_key_bytes = hex::decode(&public_key).map_err(|e| format!("Invalid public key hex: {}", e))?;
    let pk = PublicKey::from_sec1_bytes(&public_key_bytes)
        .map_err(|e| format!("Invalid public key: {}", e))?;
    
    // 从十六进制字符串解析签名
    let signature_bytes = hex::decode(&signature_hex).map_err(|e| format!("Invalid signature hex: {}", e))?;
    let signature_bytes_array: [u8; 64] = signature_bytes.try_into()
        .map_err(|_| "Invalid signature length, expected 64 bytes")?;
    let signature = Signature::from_bytes(&signature_bytes_array)
        .map_err(|e| format!("Invalid signature: {}", e))?;
    
    // 创建验证密钥
    let verifying_key = VerifyingKey::new(DEFAULT_DISTID, pk)
        .map_err(|e| format!("Failed to create verifying key: {}", e))?;
    
    // 验签
    let result = verifying_key.verify(message.as_bytes(), &signature).is_ok();
    
    Ok(result)
}