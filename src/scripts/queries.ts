import { invoke } from '@tauri-apps/api'
import { off } from 'hammerjs';
import type { Session } from './objects';

export function DB_createProject(name: string, color: string, target: number) {
  return invoke('create_project', {name: name, color: color, target: target});
}

export function DB_getActiveProjects() : Promise<any> {
  return invoke('get_active_projects');
}

export function DB_startSession(project_id : number, target: number) : Promise<any> {
    return invoke('start_session', {projectId: project_id, target: target});
}

export function DB_getDailySessions() : Promise<any> {
  let now = new Date();
  now.setHours(0);
  now.setMinutes(0);
  now.setSeconds(0);
  now.setMilliseconds(0);
  let midnight = now.toISOString().replace("T", " ").replace("Z", "")
  return invoke('get_sessions_after_date', {date: midnight});
}

export function DB_getSessionsAfterDate(date : number) : Promise<any> {
  let result = new Date(date).toISOString().replace("T", " ").replace("Z", "");
  return invoke('get_sessions_after_date', {date: result});
}

export function DB_getSessionsInRange(range : any) : Promise<any> {
  let start = new Date(range.start).toISOString().replace("T", " ").replace("Z", "");
  let end = new Date(range.end).toISOString().replace("T", " ").replace("Z", "");
  console.log({start: start, end: end})
  return invoke('get_sessions_in_range', {start: start, end: end});
}

export function DB_getProjectSessionsAfterDate(date : number, project_id : number) : Promise<any> {
  let result = new Date(date).toISOString().replace("T", " ").replace("Z", "");
  return invoke('get_project_sessions_after_date', {date: result, projectId: project_id});
}

export function DB_completeSession() : Promise<any> {
  return invoke('complete_session');
}

export function DB_getArchivedProjects() : Promise<any> {
  return invoke('get_archived_projects');
}

export function DB_saveProject(project_id : number, name : string, color: string, target: number) {
  return invoke('save_project', {projectId: project_id, name: name, color: color, target: target});
}

export function DB_archiveProject(project_id: number) {
  return invoke('archive_project', {projectId: project_id})
}

export function DB_unarchiveProject(project_id: number) {
  return invoke('unarchive_project', {projectId: project_id})
}

export function DB_deleteProject(project_id: number) {
  return invoke('delete_project', {projectId: project_id})
}

export function DB_loadNSessions(offset: number, limit: number) : Promise<string> {
  return invoke('load_n_sessions', {offset: offset, limit: limit})
}

export function DB_deleteSession(id: number) {
  return invoke('delete_session', {id: id})
}

export function DB_saveSession(id: number, start: string, end: string | null) {
  return invoke('save_session', {id: id, start: start, end: end})
}