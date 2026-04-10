<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useInvoke } from '../../composables/useInvoke'

const { incrementUseCount } = useInvoke()

const privateKey = ref('')
const publicKey = ref('')
const plaintext = ref('')
const ciphertext = ref('')
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

// 使用生成的私钥
const useGeneratedPrivateKey = () => {
  if (privateKey.value) {
    signPrivateKey.value = privateKey.value
  }
}

// 使用生成的公钥
const useGeneratedPublicKey = () => {
  if (publicKey.value) {
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
  if (!plaintext.value || !publicKey.value) {
    errorMessage.value.encrypt = '请输入明文和公钥'
    return
  }
  
  clearError('encrypt')
  loading.value.encrypt = true

  try {
    const result = await invoke<string>('sm2_encrypt', {
      plaintext: plaintext.value,
      publicKey: publicKey.value
    })
    ciphertext.value = result
    await incrementUseCount('sm2')
  } catch (error) {
    errorMessage.value.encrypt = `加密失败: ${error}`
    console.error('加密失败:', error)
  } finally {
    loading.value.encrypt = false
  }
}

const decrypt = async () => {
  if (!ciphertext.value || !privateKey.value) {
    errorMessage.value.decrypt = '请输入密文和私钥'
    return
  }
  
  clearError('decrypt')
  loading.value.decrypt = true

  try {
    const result = await invoke<string>('sm2_decrypt', {
      ciphertext: ciphertext.value,
      privateKey: privateKey.value
    })
    plaintext.value = result
    await incrementUseCount('sm2')
  } catch (error) {
    errorMessage.value.decrypt = `解密失败: ${error}`
    console.error('解密失败:', error)
  } finally {
    loading.value.decrypt = false
  }
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
            <div v-if="errorMessage.encrypt" class="error-message">
                {{ errorMessage.encrypt }}
            </div>
            <div v-if="errorMessage.decrypt" class="error-message">
                {{ errorMessage.decrypt }}
            </div>
            <div class="encryption-section">
                <div class="input-group">
                    <label>明文:</label>
                    <textarea v-model="plaintext" rows="3" class="text-input" placeholder="输入要加密的明文"></textarea>
                    <button @click="encrypt" class="btn" :disabled="loading.encrypt">
                        {{ loading.encrypt ? '加密中...' : '加密' }}
                    </button>
                </div>
                <div class="input-group">
                    <label>密文:</label>
                    <textarea v-model="ciphertext" rows="3" class="text-input" placeholder="加密后的密文"></textarea>
                    <button @click="decrypt" class="btn" :disabled="loading.decrypt">
                        {{ loading.decrypt ? '解密中...' : '解密' }}
                    </button>
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
                    <button @click="useGeneratedPrivateKey" class="btn-small" :disabled="!privateKey">
                        使用生成的私钥
                    </button>
                </div>
                <textarea 
                    v-model="signPrivateKey" 
                    rows="2" 
                    class="key-input-field" 
                    placeholder="输入用于签名的私钥（十六进制，64字符）或点击右侧按钮使用生成的私钥"
                ></textarea>
                
                <div class="key-input-header">
                    <label>验证公钥:</label>
                    <button @click="useGeneratedPublicKey" class="btn-small" :disabled="!publicKey">
                        使用生成的公钥
                    </button>
                </div>
                <textarea 
                    v-model="verifyPublicKey" 
                    rows="2" 
                    class="key-input-field" 
                    placeholder="输入用于验证的公钥（十六进制，130字符）或点击右侧按钮使用生成的公钥"
                ></textarea>
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
</style>