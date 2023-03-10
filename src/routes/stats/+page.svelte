<script lang="ts">
	import Graph from "../../components/Graph.svelte";
	import { getHours, getMinutes, getSeconds, showHours, showMinutes, showSeconds, UTCStringToMillis } from "../../scripts/helpers";
	import { DB_getActiveProjects, DB_getAllVisibleSessions, DB_getSessionsAfterDate, DB_getSessionsInRange, DB_hideProject, DB_showProject } from "../../scripts/queries";
  import { data } from "../stores";

  const Ranges = {
    WEEK: 0,
    MONTH: 1,
    YEAR: 2,
    ALL: 3
  }

  let dateRange : any = {};
  let selectedRange : number = Ranges.WEEK
  let activeProjectsData : any = [];

  $: refreshData(selectedRange);

  function processData(data : any) {
      let projects = JSON.parse(data[0]);
      let sessions = JSON.parse(data[1]);

      let projectDataMap : any = {}
      for(let project of projects) {
        projectDataMap[project.id] = {id: project.id, name: project.name, color: project.color, visible: project.visible, data: {}}
      }

      if(selectedRange == Ranges.ALL) {
        if(sessions.length > 0) {
          let start = new Date(UTCStringToMillis(sessions[sessions.length-1].start)!)
          start.setHours(0, 0, 0, 0)
          let end = new Date(sessions[0].end != null ? UTCStringToMillis(sessions[0].end)! : Date.now())
          end.setHours(23, 59, 59, 999)
          dateRange = { start: start.getTime(), end: end.getTime() }
        } else {
          dateRange = {start: new Date(new Date(Date.now()).toDateString()), end: Date.now()}
        }
      }
      for(let session of sessions) {
        // ignore if session is for archived project.
        let sessionStart = new Date(session.start! + " UTC")
        let sessionEnd;
        if(session.end) {
          sessionEnd = new Date(session.end! + " UTC")
        } else {
          sessionEnd = new Date(Date.now());
        }
        // Ignore time outside of specified range
        if (sessionStart.getTime() < dateRange.start) { sessionStart = new Date(dateRange.start) }
        // When the start and end of the session occur on the same date (simple case)
        if(sessionStart.toDateString() == sessionEnd.toDateString()) {
          let date = sessionStart.toDateString();
          let duration = sessionEnd.getTime() - sessionStart.getTime();
          projectDataMap[session.project_id].data[date] = projectDataMap[session.project_id].data[date]? projectDataMap[session.project_id].data[date] + duration : 0 + duration;
          continue;
        }

        
        // Calculate the number of days the session occurs across
        let numberOfDays = 1 + (new Date(sessionEnd.toDateString()).getTime() - new Date(sessionStart.toDateString()).getTime()) / 86400000
        for(let day = 0; day < numberOfDays; day++) {
          let date;
          let duration;
          if(day == 0) {
            date = sessionStart.toDateString();
            let endOfDay = new Date(date).setHours(24, 0, 0, 0)
            duration = (sessionEnd.getTime() < endOfDay ? sessionEnd.getTime() : endOfDay) - sessionStart.getTime();
          } else if (day == numberOfDays - 1) {
            date = sessionEnd.toDateString();
            duration = sessionEnd.getTime() - new Date(date).setHours(0, 0, 0, 0);
          } else {
            date = new Date(new Date(sessionStart.toDateString()).getTime() + day * 86400000).toDateString();
            duration = 86400000
          }
          projectDataMap[session.project_id].data[date] = projectDataMap[session.project_id].data[date]? projectDataMap[session.project_id].data[date] + duration : 0 + duration;
        }
      }

      // Calculate average and total for each active, visible project. (for projects list)
      const daysInRange = 1 + Math.floor((dateRange.end - dateRange.start) / 86400000)

      let tempActiveProjectsData = []
      for(const key of Object.keys(projectDataMap)) {
        if(projectDataMap[key].visible) {
          let total = calculateTotal(projectDataMap[key].data)
          let average = total / daysInRange;
          projectDataMap[key].total = total
          projectDataMap[key].average = average
        } else {
          projectDataMap[key].total = 0
          projectDataMap[key].average = 0
        }
        tempActiveProjectsData.push(projectDataMap[key])
      }
      activeProjectsData = tempActiveProjectsData;

      // console.log(projectDataMap);
      // (selectedRange == Ranges.ALL ? 2 : 1)
      // Generate data points for each visible project (for graph)

      let tempData : any = []
      for(const key of Object.keys(projectDataMap)) {
        if(!projectDataMap[key].visible) {continue}
      // populate data for all dates in range
        let tempDays = []
        let cumulativeTotal = 0;
        for(let day = 0; day < daysInRange; day++) {
          let date = new Date(dateRange.start + day * 86400000);
        
          let rawDuration = projectDataMap[key].data[date.toDateString()] | 0
          cumulativeTotal += rawDuration;
          tempDays[day] = { date: day, dateString: formatDate(date), cumulative: cumulativeTotal, duration: projectDataMap[key].data[date.toDateString()] | 0 }
        }
        projectDataMap[key].data = tempDays;
        tempData.push(projectDataMap[key])
      }
      $data = tempData;
    }

  function refreshData(rangeSelection : any) : any {

    let range = {};

    switch(rangeSelection) {
      case Ranges.WEEK:
        let weekStart = new Date(Date.now() - 1000 * 60 * 60 * 24 * 6)
        weekStart.setHours(0, 0, 0, 0)
        dateRange = {start: weekStart.getTime(), end: Date.now()}
        return Promise.all([DB_getActiveProjects(), DB_getSessionsInRange(dateRange)]).then((data) => processData(data));
      case Ranges.MONTH:
        let monthStart = new Date(Date.now() - 1000 * 60 * 60 * 24 * 29)
        monthStart.setHours(0, 0, 0, 0)
        dateRange = {start: monthStart.getTime(), end: Date.now()}
        return Promise.all([DB_getActiveProjects(), DB_getSessionsInRange(dateRange)]).then((data) => processData(data));
      case Ranges.YEAR:
        let yearStart = new Date(Date.now() - 1000 * 60 * 60 * 24 * 364)
        yearStart.setHours(0, 0, 0, 0)
        dateRange = {start: yearStart.getTime(), end: Date.now()}
        return Promise.all([DB_getActiveProjects(), DB_getSessionsInRange(dateRange)]).then((data) => processData(data));
      case Ranges.ALL:
        return Promise.all([DB_getActiveProjects(), DB_getAllVisibleSessions()]).then((data) => processData(data));
    }
  }
    function formatDate(date : Date) {
    let segments = date.toDateString().split(" ");
    let day = segments[2]
    let month = segments[1]
    let year = segments[3].slice(-2)
    return day + " " + month + " " + year
  }
  function calculateTotal(data : any) {
    let total = 0;
    for(let dailyTotal of Object.values(data)) {
      total += (dailyTotal as number)
    }
    return total;
  }

  function calculateNumberOfDays(data : any) {
    let dates = Object.keys(data);
    if (dates.length == 0) { return 0 }
    let min = null
    let max = null
    for(let date of Object.keys(data)) {
      let time = new Date(date).getTime()
      min = min != null ? (time < min ? time : min) : time
      max = max != null ? (time > max ? time : max) : time
    }
    return (1 + (max! - min!) / 86400000)
  }
    function viewWeek() {
      selectedRange = Ranges.WEEK;
    }
    function viewMonth() {
      selectedRange = Ranges.MONTH;
    }
    function viewYear() {
      selectedRange = Ranges.YEAR;
    }
    function viewCustom() {
      selectedRange = Ranges.ALL;
    }

    function setButtonActive(e : MouseEvent) {
      for(let rangeElement of document.getElementsByClassName("range")) {
        (rangeElement as HTMLElement).classList.remove("selected");
      }
      (e.target! as HTMLElement).classList.add("selected")
    }
    function toggleProjectVisibility(id : number, visible: boolean) {
      if(visible) {
        DB_hideProject(id).then(refreshData(selectedRange))
      } else {
        DB_showProject(id).then(refreshData(selectedRange))
      }
    }
</script>

<style>

  :global(#page) {
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  #select-range {
    display: flex;
    padding: 12px 0px 5px 0px;
    gap: 5px;
  }
  .range {
    flex: 1;
    font-family: "poppinsregular";
    font-size: 12pt;
    line-height: 24pt;
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
    line-height: 40px;
    padding: 0px 15px;
    font-family: "poppinsregular";
    font-size: 12pt;
    border-radius:10px;
    color: var(--color);
    margin: 6px 0;
    background: none;
    transition: all 150ms ease;
    display: flex;
    justify-content: space-between;
    background: #1e1e1e;
    box-shadow: 1px 1px 3px #000;
  }
  .project p {
    padding: 0;
    margin: 0;
  }
  #data-labels {
  box-shadow: 0px 0px 4px 4px #121212;
  padding: 0 15px;
  display: flex;
  color: #A9A9A9;
  font-size: 8pt;
  justify-content: space-between;
  font-family: "poppinsregular";
  z-index: 1;
  position: static;
  }
  #label-name {
    flex: 5;
  }
  #label-average {
    flex: 3;
    text-align: right;
  }
  #label-total {
    flex: 2;
    text-align: right;
  }
  .name {
    flex: 5;
  }
  .average {
    flex: 3;
    text-align: right;
    color: var(--color);
    transition: all 150ms ease;
  }
  .total {
    flex: 2;
    text-align: right;
    color: var(--color);
    transition: all 150ms ease;
  }
  
  .number {
  font-family: "latoregular";
  line-height: 0px;
}
.unit {
  font-size: 8pt;
  line-height: 0px;
}

#projects {
  flex: 1;
  overflow: scroll;
}
</style>

<Graph {dateRange}/>

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
    All
  </button>
</div>

<div id="data-labels">
  <p id="label-name">Project</p>
  <p id="label-average">Daily Average</p>
  <p id="label-total">Total</p>
</div>

<div id="projects">
{#each activeProjectsData as project}
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <div class="project" style="--color: #{project.visible ? project.color : 121212}" on:click={() => toggleProjectVisibility(project.id, project.visible)}>
    <p class="name">
      {project.name}
    </p>
    {#if project.visible}
      <p class="average" style="--color: #{project.visible ? "d3d3d3" : 121212}">
        {#if showHours(project.average)}
          <span class="number">{getHours(project.average)}</span><span class="unit">h</span>
        {/if}
        {#if showMinutes(project.average)}
        <span class="number">{getMinutes(project.average)}</span><span class="unit">m</span>
        {/if}
        {#if showSeconds(project.average) || project.average < 1000}
        <span class="number">{getSeconds(project.average)}</span><span class="unit">s</span>
        {/if}
      </p>
      <p class="total" style="--color: #{project.visible ? "d3d3d3" : 121212}">
        {#if showHours(project.total)}
        <span class="number">{getHours(project.total)}</span><span class="unit">h</span>
        {/if}
        {#if showMinutes(project.total)}
        <span class="number">{getMinutes(project.total)}</span><span class="unit">m</span>
        {/if}
        {#if showSeconds(project.total) || project.total < 1000}
        <span class="number">{getSeconds(project.total)}</span><span class="unit">s</span>
        {/if}
      </p>
    {/if}
  </div>
{/each}
  
</div>