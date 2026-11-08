export interface CommandItem {
  id: string
  name: string
  command: string
  workdir: string
  runAsAdmin: boolean
  createdAt: number
  lastRunAt: number | null
}

export interface StorePayload {
  commands: CommandItem[]
  dataDir: string
}
