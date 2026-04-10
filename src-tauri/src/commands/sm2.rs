use sm2::{
    SecretKey,
    PublicKey,
    dsa::{SigningKey, VerifyingKey, Signature, signature::{Signer, Verifier}},
};
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
    
    Ok(Sm2KeyPair {
        private_key: private_key_hex,
        public_key: public_key_hex,
        private_key_pem,
        public_key_pem,
    })
}

// 注意：sm2 0.13.3 版本不支持 PKE（公钥加密）功能
// 如需加密功能，请升级到 0.14.x 版本或使用其他 crate
// #[tauri::command(rename_all = "camelCase")]
// pub fn sm2_encrypt(plaintext: String, public_key: String) -> Result<String, String> {
//     // 从十六进制字符串解析公钥
//     let public_key_bytes = hex::decode(&public_key).map_err(|e| format!("Invalid public key hex: {}", e))?;
//     let pk = PublicKey::from_sec1_bytes(&public_key_bytes)
//         .map_err(|e| format!("Invalid public key: {}", e))?;
//     
//     // 创建加密密钥
//     let encrypting_key = EncryptingKey::new(&pk);
//     
//     // 加密
//     let encrypted = encrypting_key.encrypt(plaintext.as_bytes(), &mut OsRng)
//         .map_err(|e| format!("Encryption failed: {}", e))?;
//     
//     Ok(hex::encode(encrypted))
// }

// #[tauri::command(rename_all = "camelCase")]
// pub fn sm2_decrypt(ciphertext: String, private_key: String) -> Result<String, String> {
//     // 从十六进制字符串解析私钥
//     let private_key_bytes = hex::decode(&private_key).map_err(|e| format!("Invalid private key hex: {}", e))?;
//     let sk = SecretKey::from_bytes(&private_key_bytes)
//         .map_err(|e| format!("Invalid private key: {}", e))?;
//     
//     // 从十六进制字符串解析密文
//     let encrypted_bytes = hex::decode(&ciphertext).map_err(|e| format!("Invalid ciphertext hex: {}", e))?;
//     
//     // 创建解密密钥
//     let decrypting_key = DecryptingKey::new(&sk);
//     
//     // 解密
//     let decrypted = decrypting_key.decrypt(&encrypted_bytes)
//         .map_err(|e| format!("Decryption failed: {}", e))?;
//     
//     String::from_utf8(decrypted).map_err(|e| format!("Invalid UTF-8: {}", e))
// }

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