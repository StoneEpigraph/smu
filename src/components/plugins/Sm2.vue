<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useInvoke } from '../../composables/useInvoke'

const { incrementUseCount } = useInvoke()

const privateKey = ref('')
const publicKey = ref('')
const plaintext = ref('')
const ciphertext = ref('')

// 加密解密使用的密钥（可以手动输入或使用生成的）
const encryptPublicKey = ref('')
const decryptPrivateKey = ref('')

// 加密解密模式
const encryptMode = ref<'hex' | 'base64'>('hex')
const decryptMode = ref<'hex' | 'base64'>('hex')

// SM2 加密密文格式
const cipherMode = ref<'c1c3c2' | 'c1c2c3' | 'asn1' | 'java'>('c1c3c2')

// 使用的加密库
const cryptoLibrary = ref<'smcrypto' | 'gmssl'>('smcrypto')
const message = ref('')
const signature = ref('')
const verifyResult = ref<boolean | null>(null)

// 签名和验证使用的密钥（可以手动输入或使用生成的）
const signPrivateKey = ref('')
const verifyPublicKey = ref('')

const loading = ref({
    generate: false,
    encrypt: false,
    decrypt: false,
    sign: false,
    verify: false
})

const errorMessage = ref({
    generate: '',
    encrypt: '',
    decrypt: '',
    sign: '',
    verify: ''
})

const clearError = (type: keyof typeof errorMessage.value) => {
    errorMessage.value[type] = ''
}

// 使用生成的密钥对（加密/解密）
const useGeneratedKeyPairForEncrypt = () => {
    if (publicKey.value && privateKey.value) {
        encryptPublicKey.value = publicKey.value
        decryptPrivateKey.value = privateKey.value
    }
}

// 使用生成的密钥对（签名/验证）
const useGeneratedKeyPairForSign = () => {
    if (publicKey.value && privateKey.value) {
        signPrivateKey.value = privateKey.value
        verifyPublicKey.value = publicKey.value
    }
}

const generateKeypair = async () => {
    clearError('generate')
    loading.value.generate = true

    try {
        const keypair = await invoke<any>('generate_sm2_keypair')
        privateKey.value = keypair.private_key
        publicKey.value = keypair.public_key
        await incrementUseCount('sm2')
    } catch (error) {
        errorMessage.value.generate = `生成密钥对失败: ${error}`
        console.error('生成密钥对失败:', error)
    } finally {
        loading.value.generate = false
    }
}

const encrypt = async () => {
    if (!plaintext.value || !encryptPublicKey.value) {
        errorMessage.value.encrypt = '请输入明文和公钥'
        return
    }

    clearError('encrypt')
    loading.value.encrypt = true

    try {
        if (cipherMode.value === 'java') {
            const result = await invoke<string>('sm2_encrypt_base64', {
                plaintext: plaintext.value,
                publicKey: encryptPublicKey.value,
            })
            ciphertext.value = result
        } else {
            const result = await invoke<string>('sm2_encrypt', {
                plaintext: plaintext.value,
                publicKey: encryptPublicKey.value,
                cipherMode: cipherMode.value,
                useGmssl: cryptoLibrary.value === 'gmssl'
            })

            // 根据模式转换输出
            if (encryptMode.value === 'base64') {
                const hexBytes = hexToBytes(result)
                ciphertext.value = bytesToBase64(hexBytes)
            } else {
                ciphertext.value = result
            }
        }

        await incrementUseCount('sm2')
    } catch (error) {
        errorMessage.value.encrypt = `加密失败: ${error}`
        console.error('加密失败:', error)
    } finally {
        loading.value.encrypt = false
    }
}

const decrypt = async () => {
    if (!ciphertext.value || !decryptPrivateKey.value) {
        errorMessage.value.decrypt = '请输入密文和私钥'
        return
    }

    clearError('decrypt')
    loading.value.decrypt = true

    try {
        if (cipherMode.value === 'java') {
            const result = await invoke<string>('sm2_decrypt_base64', {
                ciphertext: ciphertext.value,
                privateKey: decryptPrivateKey.value,
            })
            plaintext.value = result
        } else {
            // 根据模式转换输入
            let ciphertextRaw = ciphertext.value
            let isBase64 = false

            if (decryptMode.value === 'base64') {
                isBase64 = true
            } else {
                // Hex 模式，验证格式
                if (!/^[0-9a-fA-F]+$/.test(ciphertext.value)) {
                    throw new Error('无效的 Hex 格式密文')
                }
            }

            const result = await invoke<string>('sm2_decrypt', {
                ciphertext: ciphertextRaw,
                privateKey: decryptPrivateKey.value,
                cipherMode: cipherMode.value,
                isBase64: isBase64,
                useGmssl: cryptoLibrary.value === 'gmssl'
            })
            plaintext.value = result
        }
        await incrementUseCount('sm2')
    } catch (error) {
        errorMessage.value.decrypt = `解密失败: ${error}`
        console.error('解密失败:', error)
    } finally {
        loading.value.decrypt = false
    }
}

// 辅助函数：hex 转 bytes
const hexToBytes = (hex: string): number[] => {
    const bytes: number[] = []
    for (let i = 0; i < hex.length; i += 2) {
        bytes.push(parseInt(hex.substr(i, 2), 16))
    }
    return bytes
}

// 辅助函数：bytes 转 base64
const bytesToBase64 = (bytes: number[]): string => {
    const binary = String.fromCharCode(...bytes)
    return btoa(binary)
}

const sign = async () => {
    if (!message.value || !signPrivateKey.value) {
        errorMessage.value.sign = '请输入消息和私钥'
        return
    }

    clearError('sign')
    loading.value.sign = true

    try {
        const result = await invoke<string>('sm2_sign', {
            message: message.value,
            privateKey: signPrivateKey.value
        })
        signature.value = result
        await incrementUseCount('sm2')
    } catch (error) {
        errorMessage.value.sign = `签名失败: ${error}`
        console.error('签名失败:', error)
    } finally {
        loading.value.sign = false
    }
}

const verify = async () => {
    if (!message.value || !signature.value || !verifyPublicKey.value) {
        errorMessage.value.verify = '请输入消息、签名和公钥'
        return
    }

    clearError('verify')
    loading.value.verify = true
    verifyResult.value = null

    try {
        const result = await invoke<boolean>('sm2_verify', {
            message: message.value,
            signatureHex: signature.value,
            publicKey: verifyPublicKey.value
        })
        verifyResult.value = result
        await incrementUseCount('sm2')
    } catch (error) {
        errorMessage.value.verify = `验签失败: ${error}`
        console.error('验签失败:', error)
    } finally {
        loading.value.verify = false
    }
}
</script>

<template>
    <div class="sm2-container">
        <div class="section">
            <h3>1. 生成密钥对</h3>
            <div class="controls">
                <button @click="generateKeypair" class="btn" :disabled="loading.generate">
                    {{ loading.generate ? '生成中...' : '生成密钥对' }}
                </button>
            </div>
            <div v-if="errorMessage.generate" class="error-message">
                {{ errorMessage.generate }}
            </div>
            <div class="key-pair">
                <div class="key-item">
                    <label>私钥:</label>
                    <textarea v-model="privateKey" readonly rows="3" class="key-input" placeholder="点击生成密钥对"></textarea>
                </div>
                <div class="key-item">
                    <label>公钥:</label>
                    <textarea v-model="publicKey" readonly rows="3" class="key-input" placeholder="点击生成密钥对"></textarea>
                </div>
            </div>
        </div>

        <div class="section">
            <h3>2. 加密/解密</h3>

            <!-- 加密库选择 -->
            <div class="library-selector">
                <label class="section-label">加密库:</label>
                <div class="mode-selector">
                    <label>
                        <input type="radio" v-model="cryptoLibrary" value="smcrypto" />
                        <span class="mode-label">smcrypto</span>
                    </label>
                    <label>
                        <input type="radio" v-model="cryptoLibrary" value="gmssl" />
                        <span class="mode-label">gmssl (推荐)</span>
                    </label>
                </div>
            </div>

            <div v-if="errorMessage.encrypt" class="error-message">
                {{ errorMessage.encrypt }}
            </div>
            <div v-if="errorMessage.decrypt" class="error-message">
                {{ errorMessage.decrypt }}
            </div>

            <!-- 加密解密密钥输入区域 -->
            <div class="key-input-section">
                <div class="key-input-header">
                    <label>加密公钥:</label>
                    <button @click="useGeneratedKeyPairForEncrypt" class="btn-small"
                        :disabled="!privateKey || !publicKey">
                        使用生成的密钥对
                    </button>
                </div>
                <textarea v-model="encryptPublicKey" rows="2" class="key-input-field"
                    placeholder="输入用于加密的公钥（十六进制，130字符）或点击右侧按钮使用生成的公钥"></textarea>

                <div class="key-input-header">
                    <label>解密私钥:</label>
                </div>
                <textarea v-model="decryptPrivateKey" rows="2" class="key-input-field"
                    placeholder="输入用于解密的私钥（十六进制，64字符）或点击上方按钮使用生成的私钥"></textarea>
            </div>

            <div class="encryption-section">
                <div class="input-group">
                    <label>明文:</label>
                    <textarea v-model="plaintext" rows="3" class="text-input" placeholder="输入要加密的明文"></textarea>
                    <div class="button-row">
                        <button @click="encrypt" class="btn" :disabled="loading.encrypt || !encryptPublicKey">
                            {{ loading.encrypt ? '加密中...' : '加密' }}
                        </button>
                        <div v-if="cipherMode !== 'java'" class="mode-selector">
                            <label>
                                <input type="radio" v-model="encryptMode" value="hex" /> Hex
                            </label>
                            <label>
                                <input type="radio" v-model="encryptMode" value="base64" /> Base64
                            </label>
                        </div>
                        <span v-else class="java-mode-hint">Base64 输出</span>
                    </div>
                </div>
                <div class="input-group">
                    <label>密文:</label>
                    <textarea v-model="ciphertext" rows="3" class="text-input" placeholder="加密后的密文"></textarea>
                    <div class="button-row">
                        <button @click="decrypt" class="btn" :disabled="loading.decrypt || !decryptPrivateKey">
                            {{ loading.decrypt ? '解密中...' : '解密' }}
                        </button>
                        <div v-if="cipherMode !== 'java'" class="mode-selector">
                            <label>
                                <input type="radio" v-model="decryptMode" value="hex" /> Hex
                            </label>
                            <label>
                                <input type="radio" v-model="decryptMode" value="base64" /> Base64
                            </label>
                        </div>
                        <span v-else class="java-mode-hint">Base64 输入，自动补 04 前缀</span>
                    </div>
                </div>

                <!-- 密文格式选择 -->
                <div class="cipher-mode-section">
                    <label class="section-label">密文格式:</label>
                    <div class="mode-selector">
                        <label>
                            <input type="radio" v-model="cipherMode" value="c1c3c2" />
                            <span class="mode-label">
                                C1C3C2
                                <span class="mode-desc">(国标标准)</span>
                            </span>
                        </label>
                        <label>
                            <input type="radio" v-model="cipherMode" value="c1c2c3" />
                            <span class="mode-label">
                                C1C2C3
                                <span class="mode-desc">(旧标准)</span>
                            </span>
                        </label>
                        <label>
                            <input type="radio" v-model="cipherMode" value="asn1" />
                            <span class="mode-label">
                                ASN.1
                                <span class="mode-desc">(DER编码)</span>
                            </span>
                        </label>
                        <label>
                            <input type="radio" v-model="cipherMode" value="java" />
                            <span class="mode-label">
                                Java兼容
                                <span class="mode-desc">(Base64/C1C3C2/自动04前缀)</span>
                            </span>
                        </label>
                    </div>
                </div>
            </div>
        </div>

        <div class="section">
            <h3>3. 签名/验签</h3>
            <div v-if="errorMessage.sign" class="error-message">
                {{ errorMessage.sign }}
            </div>
            <div v-if="errorMessage.verify" class="error-message">
                {{ errorMessage.verify }}
            </div>

            <!-- 密钥输入区域 -->
            <div class="key-input-section">
                <div class="key-input-header">
                    <label>签名私钥:</label>
                    <button @click="useGeneratedKeyPairForSign" class="btn-small" :disabled="!privateKey || !publicKey">
                        使用生成的密钥对
                    </button>
                </div>
                <textarea v-model="signPrivateKey" rows="2" class="key-input-field"
                    placeholder="输入用于签名的私钥（十六进制，64字符）或点击右侧按钮使用生成的私钥"></textarea>

                <div class="key-input-header">
                    <label>验证公钥:</label>
                </div>
                <textarea v-model="verifyPublicKey" rows="2" class="key-input-field"
                    placeholder="输入用于验证的公钥（十六进制，130字符）或点击上方按钮使用生成的公钥"></textarea>
            </div>

            <div class="signature-section">
                <div class="input-group">
                    <label>消息:</label>
                    <textarea v-model="message" rows="3" class="text-input" placeholder="输入要签名的消息"></textarea>
                    <button @click="sign" class="btn" :disabled="loading.sign || !signPrivateKey">
                        {{ loading.sign ? '签名中...' : '签名' }}
                    </button>
                </div>
                <div class="input-group">
                    <label>签名结果:</label>
                    <textarea v-model="signature" rows="3" class="text-input" placeholder="签名生成后显示在这里"></textarea>
                </div>
                <div class="verify-section">
                    <button @click="verify" class="btn" :disabled="loading.verify || !verifyPublicKey">
                        {{ loading.verify ? '验签中...' : '验签' }}
                    </button>
                    <div v-if="verifyResult !== null" class="verify-result">
                        验签结果: <span :class="verifyResult ? 'success' : 'error'">
                            {{ verifyResult ? '✓ 成功' : '✗ 失败' }}
                        </span>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
.sm2-container {
    padding: 24px;
    color: #fff;
    height: 100%;
    overflow-y: auto;
}

.section {
    margin-bottom: 32px;
    padding: 20px;
    background: rgba(255, 255, 255, 0.05);
    border-radius: 8px;
}

.section h3 {
    margin-bottom: 16px;
    color: #4CAF50;
    font-size: 16px;
    font-weight: 500;
}

.controls {
    margin-bottom: 16px;
}

.btn {
    background: #4CAF50;
    color: white;
    border: none;
    padding: 8px 16px;
    border-radius: 4px;
    cursor: pointer;
    font-size: 14px;
    transition: background 0.2s;
}

.btn:hover {
    background: #45a049;
}

.key-pair {
    display: flex;
    gap: 16px;
    margin-top: 16px;
}

.key-item {
    flex: 1;
}

.key-item label {
    display: block;
    margin-bottom: 8px;
    font-size: 14px;
    color: rgba(255, 255, 255, 0.7);
}

.key-input {
    width: 100%;
    padding: 8px;
    background: rgba(0, 0, 0, 0.3);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 4px;
    color: #fff;
    font-family: monospace;
    font-size: 12px;
    resize: vertical;
}

.encryption-section,
.signature-section {
    display: flex;
    flex-direction: column;
    gap: 16px;
}

.input-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
}

.input-group label {
    font-size: 14px;
    color: rgba(255, 255, 255, 0.7);
}

.text-input {
    width: 100%;
    padding: 8px;
    background: rgba(0, 0, 0, 0.3);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 4px;
    color: #fff;
    font-family: monospace;
    font-size: 12px;
    resize: vertical;
}

.verify-section {
    display: flex;
    align-items: center;
    gap: 16px;
    margin-top: 8px;
}

.verify-result {
    font-size: 14px;
    font-weight: 500;
}

.success {
    color: #4CAF50;
}

.error {
    color: #f44336;
}

.error-message {
    background: rgba(244, 67, 54, 0.1);
    border: 1px solid rgba(244, 67, 54, 0.3);
    color: #f44336;
    padding: 12px;
    border-radius: 4px;
    margin-bottom: 16px;
    font-size: 14px;
}

.btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
}

textarea::placeholder {
    color: rgba(255, 255, 255, 0.3);
}

.key-input-section {
    margin-bottom: 24px;
    padding: 16px;
    background: rgba(0, 0, 0, 0.2);
    border-radius: 6px;
    border: 1px solid rgba(255, 255, 255, 0.1);
}

.key-input-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 8px;
    margin-top: 12px;
}

.key-input-header:first-child {
    margin-top: 0;
}

.key-input-header label {
    font-size: 14px;
    color: rgba(255, 255, 255, 0.7);
    font-weight: 500;
}

.key-input-field {
    width: 100%;
    padding: 10px;
    background: rgba(0, 0, 0, 0.3);
    border: 1px solid rgba(255, 255, 255, 0.15);
    border-radius: 4px;
    color: #fff;
    font-family: monospace;
    font-size: 12px;
    resize: vertical;
    transition: border-color 0.2s;
}

.key-input-field:focus {
    outline: none;
    border-color: rgba(76, 175, 80, 0.5);
}

.key-input-field::placeholder {
    color: rgba(255, 255, 255, 0.25);
    font-size: 11px;
}

.btn-small {
    background: rgba(76, 175, 80, 0.2);
    color: #4CAF50;
    border: 1px solid rgba(76, 175, 80, 0.4);
    padding: 4px 12px;
    border-radius: 4px;
    cursor: pointer;
    font-size: 12px;
    transition: all 0.2s;
    white-space: nowrap;
}

.btn-small:hover:not(:disabled) {
    background: rgba(76, 175, 80, 0.3);
    border-color: rgba(76, 175, 80, 0.6);
}

.btn-small:disabled {
    opacity: 0.4;
    cursor: not-allowed;
}

.button-row {
    display: flex;
    align-items: center;
    gap: 16px;
    margin-top: 8px;
}

.mode-selector {
    display: flex;
    gap: 12px;
    align-items: center;
}

.mode-selector label {
    display: flex;
    align-items: center;
    gap: 4px;
    cursor: pointer;
    font-size: 13px;
    color: rgba(255, 255, 255, 0.7);
}

.mode-selector input[type="radio"] {
    cursor: pointer;
}

.cipher-mode-section {
    margin-top: 16px;
    padding: 12px;
    background: rgba(0, 0, 0, 0.2);
    border-radius: 6px;
    border: 1px solid rgba(255, 255, 255, 0.1);
}

.section-label {
    display: block;
    margin-bottom: 10px;
    font-size: 14px;
    color: rgba(255, 255, 255, 0.7);
    font-weight: 500;
}

.mode-label {
    display: inline-flex;
    align-items: center;
    gap: 4px;
}

.mode-desc {
    font-size: 11px;
    color: rgba(255, 255, 255, 0.4);
}

.library-selector {
    margin-bottom: 16px;
    padding: 12px;
    background: rgba(76, 175, 80, 0.1);
    border-radius: 6px;
    border: 1px solid rgba(76, 175, 80, 0.3);
}

.java-mode-hint {
    font-size: 12px;
    color: rgba(255, 193, 7, 0.8);
    font-style: italic;
}
</style>