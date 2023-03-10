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

  
  function handleProjectClick(active: boolean, id : number, target: number) {

    if(active) {
      DB_completeSession(id).then(() => refreshPageData());
    } else {
      DB_startSession(id, target).then(() => refreshPageData());
    }
    
    // for(let session of activeSessions) {

    //   // If the project has an active session, stop that session
    //   if(session.project_id == id) {
    //   }
    // }

    
    // DB_startSession(id, target).then(() => refreshPageData());

    // // If there are no active sessions, start a new session.
    // if(!activeSession) return DB_startSession(clickedProject!.id, clickedProject!.target).then(() => refreshPageData());

    // // Else if the selected project was active, stop the session.
    // if(activeSession.project_id === clickedProject!.id) return DB_completeSession().then(() => refreshPageData());

    // // Else, stop the session then start a new session.
    // return DB_startSession(clickedProject!.id, clickedProject!.target).then(() => refreshPageData());
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
  text-align: center;
  line-height: 28pt;
  padding: 0;
  font-family: "poppinsregular";
  font-size: 14pt;
  border-radius:10px;
  color: var(--color);
  background: #1e1e1e;
  margin: 6px 0;
  transition: all 200ms ease;
  box-shadow: 1px 1px 3px #000;
}

.project:first-child {
  margin: 0;
}

.active {
  background: var(--color);
  color: #121212;
}

:global(#page) {
  height: calc(100vh - 60px);
  max-height: calc(100vh - 60px);
  overflow: hidden;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

#timer-container {
  box-shadow: 0px 2px 10px 6px #121212;
  z-index:1;
  position: relative;
}

#projects-container {
  padding-top: 12px;
  flex: 1;
  overflow: scroll;
  padding-bottom: 12px;
}

:global(#page:after) {
  content: "";
  box-shadow: 0px 0px 10px 6px #000;
  width: 100%;
  height: 1px;
}

</style>
<div id="timer-container">
  <Timer {dailySessions} on:updateSessions={refreshPageData}/>
</div>
<!-- svelte-ignore a11y-click-events-have-key-events -->
<div id="projects-container">
{#each $activeProjects as project (project.id)} 
  <div class="project {project.active? "active" : ""}" style="--color: #{project.color}" id="p{project.id}" data-id={project.id} on:click={(e) => {handleProjectClick(project.active, project.id, project.target)}}>{project.name}</div>
{/each}

</div>