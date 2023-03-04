<script lang="ts">
	import { DB_getActiveProjects } from '../../scripts/queries';
	import { DB_getArchivedProjects } from '../../scripts/queries';
  import { activeProjects } from '../stores';
  import type {  Project } from '../../scripts/objects'
	import type PopupProjectData from '../../components/Popup_ProjectData.svelte';
	import type { SvelteComponent } from 'svelte';
	import PopupEditProject from '../../components/Popup_EditProject.svelte';
  import ArchivedProjectOptions from '../../components/ArchivedProjectOptions.svelte';
  
let projects : Project[] = [];

DB_getActiveProjects().then((data) => {
  projects = JSON.parse(data);
});

let active : HTMLElement;
let archived : HTMLElement;
let popupWindow : any;
let selectedProject : Project | undefined;

async function showActive() {
  if (active && archived) {
    active.classList.add("selected")
    archived.classList.remove("selected");
  }
  await DB_getActiveProjects().then((data) => {
    projects = JSON.parse(data);
  });
}

async function showArchived() {
  if (active && archived) {
    archived.classList.add("selected")
    active.classList.remove("selected");
  }
  await DB_getArchivedProjects().then((data) => {
    projects = JSON.parse(data);
  });
}

function handleProjectClick(target : EventTarget  | null) {
  if (!target) return
  let element : HTMLElement = <HTMLElement>target
  if (!element.dataset) return
  if (!element.dataset.id) return
  selectedProject = projects.find(project => {
    return project.id === Number(element.dataset.id)
  })

  if(selectedProject?.archived) {
    popupWindow = ArchivedProjectOptions;
  } else {
    popupWindow = PopupEditProject;
  }
}

function confirmDeletion() {
  popupWindow = null;
  showArchived();
}

</script>

<style>
:global(body) {
  padding: 10px;
}

.header {
  display: flex;
  justify-content: center;
  padding: 5px;
  gap: 15px;
}

.header > button {
  font-size: 18pt;
  color: #fff;
  font-family: "poppinssemibold";
  border-radius: 55px;
  flex: 1;
  height: 55px;
  background: #121212;
  border: 2px solid #fff;
}

.header > button.selected {
  color: #121212;
  background: #fff;
}

.project {
  flex: 1;
  text-align: center;
  height: 55px;
  line-height: 45px;
  padding: 0;
  font-family: "poppinsregular";
  font-size: 15pt;
  border-radius: 8px;
  color: var(--color);
  border: 2px solid var(--color);
  margin: 8px 0;
  background: none;
  transition: all 200ms ease;
}
</style>

<div class="header">
  <button class="selected" bind:this={active} on:click={showActive}>Active</button>
  <button bind:this={archived} on:click={showArchived}>Archived</button>
</div>
{#each projects as project (project.id)} 
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <div class="project" style="--color: #{project.color}" id="p{project.id}" data-id={project.id} on:click={(e) => {handleProjectClick(e.target)}}>{project.name}</div>
{/each}

<svelte:component this={popupWindow} {selectedProject} on:showArchived={showArchived} on:showActive={showActive} on:close={ () => {popupWindow = null}} on:deleted={confirmDeletion}/>
