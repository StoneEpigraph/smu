<script setup lang="ts">
import { ref, onMounted, nextTick } from 'vue'
import Database from '@tauri-apps/plugin-sql'

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
let db: any = null

const loadNotes = async () => {
  if (!db) return
  try {
    const result = await db.select(
      'SELECT * FROM quick_notes ORDER BY created_at DESC LIMIT 10'
    ) as Note[]
    notes.value = result
  } catch (e) {
    console.log('Load notes failed:', e)
  }
}

const addNote = async () => {
  if (!newNote.value.trim() || !db) return
  try {
    await db.execute(
      'INSERT INTO quick_notes (content) VALUES (?)',
      [newNote.value]
    )
    newNote.value = ''
    await loadNotes()
  } catch (e) {
    console.log('Add note failed:', e)
  }
}

const deleteNote = async (id: number) => {
  if (!db) return
  try {
    await db.execute('DELETE FROM quick_notes WHERE id = ?', [id])
    await loadNotes()
  } catch (e) {
    console.log('Delete note failed:', e)
  }
}

onMounted(async () => {
  try {
    db = await Database.load('sqlite:krunner.db')
    await db.execute(`
      CREATE TABLE IF NOT EXISTS quick_notes (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        content TEXT,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
      )
    `)
    await loadNotes()
  } catch (e) {
    console.log('Database init failed:', e)
  }
  
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
        placeholder="写下你的笔记..."
        @keydown.ctrl.enter="addNote"
      ></textarea>
      <button @click="addNote">添加笔记</button>
    </div>
    
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

.input-section textarea {
  background: rgba(255, 255, 255, 0.1);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 10px;
  padding: 12px;
  color: #fff;
  font-size: 14px;
  resize: none;
  height: 80px;
  outline: none;
}

.input-section textarea:focus {
  border-color: rgba(100, 150, 255, 0.5);
}

.input-section button {
  background: rgba(100, 150, 255, 0.3);
  border: none;
  border-radius: 8px;
  padding: 10px;
  color: #fff;
  cursor: pointer;
}

.input-section button:hover {
  background: rgba(100, 150, 255, 0.5);
}

.notes-list {
  flex: 1;
  overflow-y: auto;
}

.note-item {
  background: rgba(255, 255, 255, 0.05);
  border-radius: 10px;
  padding: 12px;
  margin-bottom: 8px;
}

.note-content {
  color: #fff;
  font-size: 14px;
  white-space: pre-wrap;
  word-break: break-word;
}

.note-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 8px;
}

.note-time {
  color: rgba(255, 255, 255, 0.4);
  font-size: 11px;
}

.delete-btn {
  background: transparent;
  border: none;
  cursor: pointer;
  opacity: 0.5;
  padding: 4px;
}

.delete-btn:hover {
  opacity: 1;
}

.no-notes {
  text-align: center;
  color: rgba(255, 255, 255, 0.4);
  padding: 40px;
}
</style>