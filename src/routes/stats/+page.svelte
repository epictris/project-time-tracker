<script lang="ts">
	import type { Session } from "../../scripts/objects";
	import { DB_getActiveProjects, DB_getSessionsAfterDate, DB_getSessionsInRange } from "../../scripts/queries";

  let projects : any = [];
  let range = {}

  viewWeek();

  $: data = refreshData(range);

  function refreshData(range : any) {
    return Promise.all([DB_getActiveProjects(), DB_getSessionsInRange(range)]).then((data) => {
      projects = JSON.parse(data[0]);
      let sessions = JSON.parse(data[1]);


      console.log(sessions);
    })
  }

  // function refreshPageData(range : any) {
  //   return Promise.all([DB_getActiveProjects(), DB_getSessionsAfterDate($rangeStart)]).then((result : any) => {
    
  //   let projects = JSON.parse(result[0]);
  //   let sessions = JSON.parse(result[1]);

  //   let temporaryList = []

  //   for (let project of projects) {
  //     let projectEntry = {id: project.id, name: project.name, color: project.color, duration: 0}
  //     for(let session of sessions) {
  //         console.log(session)
  //       if(project.id == session.project_id) {
  //         if (!session.end) continue;
  //           projectEntry.duration += calculateSessionTime(session)
  //         }
  //       }
  //     temporaryList.push(projectEntry);
  //     }
  //   projectsList = temporaryList;
  //   })
  // }

  // function calculateSessionTime(session : Session) {
  //     let start = new Date(session.start).getTime();
  //     let end = new Date(session.end!).getTime();

  //     return end - (start > $rangeStart ? start : $rangeStart);
  //   }

    function viewWeek() {
      let start = new Date(Date.now() - 1000 * 60 * 60 * 24 * 6)
      start.setHours(0, 0, 0, 0)
      range = {start: start.getTime(), end: Date.now()}
    }

    function viewMonth() {
      let start = new Date(Date.now() - 1000 * 60 * 60 * 24 * 30)
      start.setHours(0, 0, 0, 0)
      range = {start: start.getTime(), end: Date.now()}
    }

    function viewYear() {
      let start = new Date(Date.now() - 1000 * 60 * 60 * 24 * 365)
      start.setHours(0, 0, 0, 0)
      range = {start: start.getTime(), end: Date.now()}
    }

    function viewCustom() {
    }

    function setButtonActive(e : MouseEvent) {
      for(let rangeElement of document.getElementsByClassName("range")) {
        (rangeElement as HTMLElement).classList.remove("selected");
      }
      (e.target! as HTMLElement).classList.add("selected")
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

  #select-range {
    display: flex;
    padding: 10px;
    gap: 5px;
  }

  .range {
    flex: 1;
    font-family: "poppinsregular";
    font-size: 12pt;
    line-height: 20pt;
    border-radius: 20px;
    background: #1e1e1e;
    border: none;
    color: #fff;
    transition: all 150ms;
  }

  .range.selected {
    color: #1e1e1e;
    background: #d3d3d3;
  }

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

<!-- <Chart {data}/> -->

<div id="select-range">
  <button on:click={(e) => {viewWeek(); setButtonActive(e)}} class="range selected">
    Week
  </button>
  <button on:click={(e) => {viewMonth(); setButtonActive(e)}} class="range">
    Month
  </button>
  <button on:click={(e) => {viewYear(); setButtonActive(e)}} class="range">
    Year
  </button>
  <button on:click={(e) => {viewCustom(); setButtonActive(e)}} class="range">
    Custom
  </button>
</div>

{#each projects as project}
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
