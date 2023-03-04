<script lang="ts">
	import type { Session } from "../../scripts/objects";
	import { DB_getActiveProjects, DB_getSessionsAfterDate } from "../../scripts/queries";
  import { activeProjects, rangeStart } from "../stores";   

  let projectsList : any = [];

  $: refreshPageData($rangeStart);

  function refreshPageData(range : any) {
    return Promise.all([DB_getActiveProjects(), DB_getSessionsAfterDate($rangeStart)]).then((result : any) => {
    
    let projects = JSON.parse(result[0]);
    let sessions = JSON.parse(result[1]);

    let temporaryList = []

    for (let project of projects) {
      let projectEntry = {id: project.id, name: project.name, color: project.color, duration: 0}
      for(let session of sessions) {
          console.log(session)
        if(project.id == session.project_id) {
          if (!session.end) continue;
            projectEntry.duration += calculateSessionTime(session)
          }
        }
      temporaryList.push(projectEntry);
      }
    projectsList = temporaryList;
    })
  }

  function calculateSessionTime(session : Session) {
      let start = new Date(session.start).getTime();
      let end = new Date(session.end!).getTime();

      return end - (start > $rangeStart ? start : $rangeStart);
    }

    function showHours(duration : number) {
      let hours = Math.floor(duration / 3600000)
      if(hours > 0) return true
      return false
    }

    function showMinutes(duration : number) {
      let hours = Math.floor(duration / 3600000)
      let minutes = Math.floor((duration / 60000) % 60)
      if(hours >= 10) return false
      if(minutes > 0) return true;
      return false;
    }

    function showSeconds(duration : number) {
      let hours = Math.floor(duration / 3600000)
      let minutes = Math.floor((duration / 60000) % 60)
      let seconds = Math.floor((duration / 1000) % 60)
      if (hours > 0 || minutes > 10) return false;
      if (seconds > 0) return true;
      return false;
    }

    function getHours(duration : number) {
      return Math.floor(duration / 3600000)
    }

    function getMinutes(duration : number) {
      return Math.floor((duration / 60000) % 60)
    }

    function getSeconds(duration : number) {
      return Math.floor((duration / 1000) % 60)
    }

</script>

<style>
  .project {
    font-family: "poppinsregular";
    font-size: 12pt;
    background-color: #1e1e1e;
    color: var(--color);
    line-height: 45px;
    padding: 0px 15px;
    border-radius: 8px;
    margin-top: 8px;
    box-shadow: 1px 1px 3px #000;
    display: flex;
    justify-content: space-between;
    text-decoration: none;
    -webkit-tap-highlight-color: transparent; /* for removing the highlight */
  }

  .duration {
    font-size: 15px;
    font-family: "poppinssemibold";
  }

  .duration > span {
    font-size: 12px;
  }
  
</style>

{#each projectsList as project}
  <a href="reports/single?id={project.id}&color={project.color}&name={project.name}" style="--color: #{project.color}"class="project">
    <span>{project.name}</span>
    <span class="duration">
      {#if showHours(project.duration)}
        {getHours(project.duration)}<span>h</span>
      {/if}
      {#if showMinutes(project.duration)}
        {getMinutes(project.duration)}<span>m</span>
      {/if}
      {#if showSeconds(project.duration)}
        {getSeconds(project.duration)}<span>s</span>
      {/if}
    </span>
  </a>
{/each}
