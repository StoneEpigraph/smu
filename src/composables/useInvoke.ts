import { invoke } from '@tauri-apps/api/core'

export const useInvoke = () => {
  const incrementUseCount = async (pluginId: string) => {
    try {
      await invoke('increment_use_count', {
        pluginId
      })
    } catch (error) {
      console.error('Error incrementing use count:', error)
    }
  }

  return {
    incrementUseCount
  }
}