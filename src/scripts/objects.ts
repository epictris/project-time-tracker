export type Session = {
  id:         number
  project_id: number
  start:      string
  end:        string | null
  target:     number
}

export type Project = {
  id:         number
  name:       string
  color:      string
  target:     number
  archived:   boolean
  active:     boolean
}

export type ProjectSession = {
  id:         number
  start:      string
  end:        string | null
  target:     number
  name:       string
  color:      string
}