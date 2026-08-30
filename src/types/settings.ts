export interface PluginConfig {
  id: string
  name: string
  nameZh: string
  icon: string
  enabled: boolean
  config: Record<string, any>
}

export interface AppSettings {
  system?: Record<string, any>
  plugins: PluginConfig[]
}
