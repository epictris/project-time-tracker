<script lang="ts">
  import { activeProjects } from './stores';
  import type { Session, Project } from '../scripts/objects'
  import { DB_getActiveProjects } from '../scripts/queries';
  import { DB_startSession } from '../scripts/queries';
  import { DB_getDailySessions } from '../scripts/queries';
  import { DB_completeSession } from '../scripts/queries';
  import Timer from '../components/Timer.svelte';
  import { canOpenMenu } from './stores';

  // Get active projects and daily sessions when page is first loaded.

  $activeProjects = [];
  $canOpenMenu = true;
  let dailySessions : Session[] = [];

  refreshPageData();

  function handleProjectClick(selectedElement : EventTarget | null) {
    if(!selectedElement) return;
    let projectID : number = Number((selectedElement as HTMLElement).dataset.id);
    if(!projectID) return;
    let clickedProject = $activeProjects.find(project => {return project.id === Number(projectID)});
    if(!clickedProject) return;
    let activeSession : Session | null = getActiveSession();

    // If there are no active sessions, start a new session.
    if(!activeSession) return DB_startSession(clickedProject!.id, clickedProject!.target).then(() => refreshPageData());

    // Else if the selected project was active, stop the session.
    if(activeSession.project_id === clickedProject!.id) return DB_completeSession().then(() => refreshPageData());

    // Else, stop the session then start a new session.
    return DB_startSession(clickedProject!.id, clickedProject!.target).then(() => refreshPageData());
  }

  function getActiveSession() : Session | null {
    for (let session of dailySessions) {
      if(!session.end) return session;
    }
    return null;
  }

  async function refreshPageData() : Promise<any> {
    return Promise.all([DB_getActiveProjects(), DB_getDailySessions()]).then((values) => {
      $activeProjects = JSON.parse(values[0]);
      dailySessions = JSON.parse(values[1]);
    }).catch((error) => {
      window.alert(error);
      $activeProjects = [];
      dailySessions = [];
    });
  }

</script>

<style>
* {
  color: #fff;
}

.project {
  flex: 1;
  text-align: center;
  line-height: 40px;
  padding: 0;
  font-family: "poppinsregular";
  font-size: 14pt;
  border-radius:10px;
  color: var(--color);
  border: 2px solid var(--color);
  margin: 8px 0;
  background: none;
  transition: all 200ms ease;
}

.active {
  background: var(--color);
  color: #121212;
}

</style>
<Timer {dailySessions} on:updateSessions{refreshPageData}/>
<!-- svelte-ignore a11y-click-events-have-key-events -->
{#each $activeProjects as project (project.id)} 
  <div class="project {project.active? "active" : ""}" style="--color: #{project.color}" id="p{project.id}" data-id={project.id} on:click={(e) => {handleProjectClick(e.target)}}>{project.name}</div>
{/each}

{#if $activeProjects.length == 0}
  <div id="finger"/>
{/if}
