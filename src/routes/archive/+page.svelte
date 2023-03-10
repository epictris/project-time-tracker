<script lang="ts">
	import { DB_getActiveProjects } from '../../scripts/queries';
	import { DB_getArchivedProjects } from '../../scripts/queries';
  import { activeProjects } from '../stores';
  import type {  Project } from '../../scripts/objects'
	import type PopupProjectData from '../../components/Popup_ProjectData.svelte';
	import type { SvelteComponent } from 'svelte';
	import PopupEditProject from '../../components/Popup_EditProject.svelte';
  import ArchivedProjectOptions from '../../components/ArchivedProjectOptions.svelte';
	import { showHours, showMinutes, showSeconds, getHours, getMinutes, getSeconds } from '../../scripts/helpers';
  
let projects : Project[] = [];

DB_getArchivedProjects().then((data) => {
  projects = JSON.parse(data);
});

let active : HTMLElement;
let archived : HTMLElement;
let popupWindow : any;
let selectedProject : Project | undefined;

async function showProjects() {
  if (active && archived) {
    active.classList.add("selected")
    archived.classList.remove("selected");
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
  showProjects();
}

</script>

<style>


#data-labels {
  padding: 0 15px;
  display: flex;
  color: #A9A9A9;
  font-size: 8pt;
  justify-content: space-between;
  font-family: "poppinsregular"
}

.project {
  line-height: 40px;
  padding: 0px 15px;
  font-family: "poppinsregular";
  font-size: 12pt;
  border-radius:10px;
  color: var(--color);
  margin: 6px 0;
  background: none;
  transition: all 200ms ease;
  display: flex;
  justify-content: space-between;
  background: #1e1e1e;
  box-shadow: 1px 1px 3px #000;
}

.project:first-child {
  margin: 0;
}

.target {
  color: #EECB43;
  pointer-events: none;
  line-height: 40px;
  pointer-events: none;
}

.number {
  font-family: "latoregular";
  line-height: 0px;
}

.unit {
  font-size: 8pt;
  line-height: 0px;
}

.empty {
  color: #9a9a9a;
  pointer-events: none;
}


#projects {
  flex: 1;
  overflow: scroll;
}
</style>

<div id="data-labels">
  <p>Project</p>
  <p>Daily Target</p>
</div>

<div id="projects">

{#each projects as project (project.id)} 
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <div class="project" style="--color: #{project.color}" id="p{project.id}" data-id={project.id} on:click={(e) => {handleProjectClick(e.target)}}>
    {project.name}
    {#if project.target > 0}
    <span class="target">
      {#if showHours(project.target * 60000)}
        <span class="number">{getHours(project.target * 60000)}</span><span class="unit">h</span>
      {/if}
      {#if getMinutes(project.target * 60000) > 0}
        <span class="number">{getMinutes(project.target * 60000)}</span><span class="unit">m</span>
      {/if}
    </span>
    {:else}
      <span class="empty">None</span>
    {/if}
  </div>
{/each}
</div>
<svelte:component this={popupWindow} {selectedProject} on:refreshData={showProjects} on:close={ () => {popupWindow = null}} on:deleted={confirmDeletion}/>
