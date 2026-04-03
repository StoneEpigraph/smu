import { invoke } from '@tauri-apps/api'

export const useInvoke = () => {
  const incrementUseCount = async (toolName: string) => {
    try {
      await invoke('increment_use_count', {
        toolName
      })
    } catch (error) {
      console.error('Error incrementing use count:', error)
    }
  }

  return {
    incrementUseCount
  }
}