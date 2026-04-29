import {
  GetRules,
  AddRule,
  DeleteRule,
  ToggleRule,
  GetLogs,
  ClearLogs,
  GetStatus,
} from '../../wailsjs/go/main/App'

export type { ForwardRule, LogEntry } from '../../wailsjs/go/main/App'

export const api = {
  getRules: GetRules,
  addRule: AddRule,
  deleteRule: DeleteRule,
  toggleRule: ToggleRule,
  getLogs: GetLogs,
  clearLogs: ClearLogs,
  getStatus: GetStatus,
}
