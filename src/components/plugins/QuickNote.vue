<script setup lang="ts">
import { ref, onMounted, nextTick } from 'vue'
import { invoke } from '@tauri-apps/api/core'

interface Note {
  id: number
  content: string
  created_at: string
}

const props = defineProps<{
  initialInput?: string
}>()

const inputRef = ref<HTMLTextAreaElement | null>(null)
const notes = ref<Note[]>([])
const newNote = ref('')
const errorMsg = ref('')
const successMsg = ref('')

const loadNotes = async () => {
  try {
    const result = await invoke<[number, string, string][]>('get_notes')
    notes.value = result.map(([id, content, created_at]) => ({ id, content, created_at }))
  } catch (e: any) {
    console.error('Load notes failed:', e)
    errorMsg.value = `加载失败: ${e}`
  }
}

const addNote = async () => {
  if (!newNote.value.trim()) {
    errorMsg.value = '笔记内容不能为空'
    return
  }
  try {
    await invoke('add_note', { content: newNote.value })
    newNote.value = ''
    successMsg.value = '笔记保存成功'
    await loadNotes()
  } catch (e: any) {
    console.error('Add note failed:', e)
    errorMsg.value = `保存失败: ${e}`
  }
}

const deleteNote = async (id: number) => {
  try {
    await invoke('delete_note', { id })
    await loadNotes()
  } catch (e: any) {
    console.error('Delete note failed:', e)
    errorMsg.value = `删除失败: ${e}`
  }
}

onMounted(async () => {
  await loadNotes()
  
  if (props.initialInput) {
    newNote.value = props.initialInput
  }
  
  await nextTick()
  inputRef.value?.focus()
})
</script>

<template>
  <div class="quick-note">
    <div class="input-section">
      <textarea
        ref="inputRef"
        v-model="newNote"
        placeholder="写下你的笔记... (Ctrl+Enter 保存)"
        @keydown.ctrl.enter="addNote"
      ></textarea>
      <button @click="addNote">添加笔记</button>
    </div>
    
    <div v-if="errorMsg" class="error-msg">{{ errorMsg }}</div>
    <div v-if="successMsg" class="success-msg">{{ successMsg }}</div>
    
    <div class="notes-list">
      <div v-for="note in notes" :key="note.id" class="note-item">
        <div class="note-content">{{ note.content }}</div>
        <div class="note-footer">
          <span class="note-time">{{ new Date(note.created_at).toLocaleString() }}</span>
          <button class="delete-btn" @click="deleteNote(note.id)">🗑️</button>
        </div>
      </div>
      <div v-if="notes.length === 0" class="no-notes">
        暂无笔记
      </div>
    </div>
  </div>
</template>

<style scoped>
.quick-note {
  flex: 1;
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.input-section {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

textarea {
  width: 100%;
  height: 80px;
  padding: 12px;
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 10px;
  background: rgba(255, 255, 255, 0.1);
  color: #fff;
  font-size: 14px;
  resize: none;
}

textarea:focus {
  outline: none;
  border-color: #4CAF50;
}

button {
  padding: 10px 20px;
  background: #4CAF50;
  color: white;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  font-size: 14px;
}

button:hover {
  opacity: 0.8;
}

.error-msg {
  padding: 10px;
  background: #ffebee;
  color: #c62828;
  border-radius: 8px;
  font-size: 14px;
}

.success-msg {
  padding: 10px;
  background: #e8f5e9;
  color: #2e7d32;
  border-radius: 8px;
  font-size: 14px;
}

.notes-list {
  flex: 1;
  overflow-y: auto;
}

.note-item {
  background: rgba(255, 255, 255, 0.05);
  border-radius: 8px;
  padding: 12px;
  margin-bottom: 12px;
}

.note-content {
  color: #fff;
  font-size: 14px;
  margin-bottom: 8px;
  white-space: pre-wrap;
  word-break: break-word;
}

.note-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.note-time {
  color: rgba(255, 255, 255, 0.4);
  font-size: 12px;
}

.delete-btn {
  background: transparent;
  padding: 4px 8px;
  font-size: 16px;
}

.delete-btn:hover {
  opacity: 0.6;
}

.no-notes {
  text-align: center;
  color: rgba(255, 255, 255, 0.4);
  padding: 40px;
}
</style>