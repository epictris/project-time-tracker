<script lang="ts">
  import { page } from "$app/stores";
	import { onMount } from "svelte";
	import Graph from "../../../components/Graph.svelte";
	import type { Session } from "../../../scripts/objects";
	import { DB_getProjectSessionsAfterDate, DB_getSessionsAfterDate } from "../../../scripts/queries";
  import { rangeStart } from "../../stores";
  let id : number | null = Number($page.url.searchParams.get("id"));
  let color : string | null = $page.url.searchParams.get("color");
  let name : string | null = $page.url.searchParams.get("name");
  let data : any = []
  $: populateDays($rangeStart);
  $:total = findTotal(data)
  $:average = findTotal(data) / data.length;
  function findTotal(data : any) {
    let total = 0;
    for(let day of data) {
      total += day.duration;
    }
    return total;
  }
  function populateDays(startOfRange : any) {
    data = []
    if(!id) {return console.log("no ID")}
    DB_getProjectSessionsAfterDate(startOfRange, id).then((result) => {
      console.log(result)
      let sessions : Session[] = JSON.parse(result);
      let dateDurations : any = {}
      for(let session of sessions) {
        if(!session.end) continue;
        let sessionStart = new Date(session.start! + " UTC")
        let sessionEnd = new Date(session.end! + " UTC")
        // Ignore time outside of specified range
        if (sessionStart.getTime() < startOfRange) { sessionStart = new Date($rangeStart) }
        // When the start and end of the session occur on the same date (simple case)
        if(sessionStart.toDateString() == sessionEnd.toDateString()) {
          let date = sessionStart.toDateString();
          let duration = sessionEnd.getTime() - sessionStart.getTime();
          dateDurations[date] = dateDurations[date]? dateDurations[date] + duration : 0 + duration;
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
          dateDurations[date] = dateDurations[date]? dateDurations[date] + duration : 0 + duration;
        }
      }
      // populate data for all dates in range
      let daysInRange = 1 + Math.floor((new Date().getTime() - startOfRange) / 86400000)
      let tempDays = []
      for(let day = 0; day < daysInRange; day++) {
        let date = new Date(startOfRange + day * 86400000).toDateString();
        tempDays[day] = { date: day, duration: dateDurations[date] | 0 }
      }
      data = tempDays;
    })
  }
  function formatDuration(duration : number) {
    let hours = Math.floor(duration / 3600000)
    let minutes = Math.floor((duration / 60000) % 60)
    let seconds = Math.floor((duration / 1000) % 60)
    if (hours >= 10) return hours + "h";
    if (hours > 0) return hours + "h" + (minutes > 0 ? " " + minutes + "m" : "")
    if (minutes > 0) return minutes + "m" + (seconds > 0 ? " " + seconds + "s" : "")
    return seconds + "s";
  }
  // function formatDate(date : Date) {
  //   let segments = date.toDateString().split(" ");
  //   let day = segments[2]
  //   let month = segments[1]
  //   let year = segments[3].slice(-2)
  //   return day + " " + month + " " + year
  // }
</script>


{#if data.length > 0}
<div id="chart">
  <div id="header">
    <div style="--color: #{color}" id="title">
      {name}
    </div>
    <div id="toggle">

    </div>
  </div>
    <Graph {data} {color}/>
</div>

<div id="tiles" style="--color: #{color}">
  <div class="row">
    <div class="info">
      <div class="name">
        Total
      </div>
      <div class="value">
        {formatDuration(total)}
      </div>
    </div>
    <div class="info">
      <div class="name">
        Daily Average
      </div>
      <div class="value">
        {formatDuration(average)}
      </div>
    </div>
  </div>  
</div>

{/if}
<style>
.name {
  color: var(--color);
  font-family: "poppinssemibold";
  margin-top: 10px;
  font-size: 18px;
}
.row {
  display: flex;
  justify-content: space-evenly;
  gap: 10px;
  margin-top: 10px;
}
.value {
  color: #fff;
  font-size: 24px;
  font-family: "poppinssemibold";
  padding: 20px 0;
}
.info {
  text-align: center;
  flex: 1;
  background: #1e1e1e;
  border-radius: 8px;
}
  #chart {
    padding-bottom: 30px;
    padding-top: 60px;
    background-color: #1e1e1e;
    margin-top: 10px;
    border-radius: 8px;
    position: relative;
  }
  
  #header {
    position: absolute;
    top: 0;
  }
  
  #title {
    color: var(--color);
    font-size: 22px;
    padding: 18px;
    font-family: "poppinssemibold"
  }
  </style>