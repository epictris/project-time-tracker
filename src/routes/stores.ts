import type { Project } from '../scripts/objects'
import { writable } from 'svelte/store';

export let activeProjects = writable<Project[]>();

export let canOpenMenu = writable<Boolean>();

export let range = writable<any>();