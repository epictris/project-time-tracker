<script lang="ts">
	import { invoke } from "@tauri-apps/api";
	import { onDestroy, onMount } from "svelte";
  import { activeProjects } from "../routes/stores";
  import type { Session, Project } from '../scripts/objects'
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();

  export let dailySessions : any;
  
  const minZoomLevel = 30 * 1000; //minimum zoom level (in milliseconds)
  const minTimerHeight = 24;
  const topTimerBuffer = 27;
  let containerElement : HTMLElement;

  type Timer = {
    id: number
    color: string
    target: number
    targetPixelHeight: number
    dailySessionTotal : number
    pixelHeight : number
    active : boolean
  }

  let renderedTimers : Timer[] = [];
  let timerLoop : NodeJS.Timeout;
  $: updateTimerData($activeProjects, dailySessions);

  onMount(() => {
    updateTimerData($activeProjects, dailySessions);
  })

  onDestroy(() => {
    clearInterval(timerLoop);
  })

  function updateTimerData(projects : Project[], sessions : Session[]) {
    if(!containerElement) return;

    let containerHeight = containerElement.getBoundingClientRect().height - (minTimerHeight + topTimerBuffer);
    let longestDuration = 0;
    let timers : Timer[] = [];
    for(let project of projects) {
      let timer: Timer = { id: project.id, color: project.color, target: project.target, targetPixelHeight: 0, dailySessionTotal: 0, pixelHeight: minTimerHeight, active: project.active }
      for(let session of sessions) {
        if(session.project_id === project.id) {
          timer.dailySessionTotal += calculateSessionDuration(UTCStringToMillis(session.start), UTCStringToMillis(session.end));
        }
      }

      // Compile list of timers
      if(timer.active || timer.dailySessionTotal > 0 || timer.target > 0) {
        longestDuration = timer.dailySessionTotal > longestDuration ? timer.dailySessionTotal : longestDuration;
        timers.push(timer);
      }
    }

    // Calculate pixel height of each timer
    for (let timer of timers) {
      timer.pixelHeight = minTimerHeight + calculateDisplayHeight(longestDuration, containerHeight, timer.dailySessionTotal);
      timer.targetPixelHeight = minTimerHeight + calculateTargetHeight(longestDuration, containerHeight, timer.target)
    }

    renderedTimers = timers;

    clearInterval(timerLoop);

    let activeSession = getActiveSession();
    if(!activeSession) return;

    let activeTimer = renderedTimers.find(timer => {return timer.active});
    if(!activeTimer) return;
    
    let timerIndex = renderedTimers.indexOf(activeTimer);
    let timerStartMillis = new Date().getTime();
    let timerStartTotal = activeTimer.dailySessionTotal;

    let currentDay  = new Date().getDate();

    if(longestDuration < minZoomLevel) longestDuration = minZoomLevel;

    timerLoop = setInterval(() => {
      let newCurrentDay = new Date().getDate();
      if (newCurrentDay != currentDay) {
        dispatch("updateSessions");
        return clearInterval(timerLoop);
      }
      if(!activeTimer) return clearInterval(timerLoop);
      let newDailySessionTotal = timerStartTotal + calculateSessionDuration(timerStartMillis, null);
      if(newDailySessionTotal > longestDuration) longestDuration = newDailySessionTotal;

      // Set the active timer session total
      renderedTimers[timerIndex].dailySessionTotal = newDailySessionTotal;
      // Set new active timer height
      renderedTimers[timerIndex].pixelHeight = minTimerHeight + calculateDisplayHeight(longestDuration, containerHeight, newDailySessionTotal);
      for(let index = 0; index < renderedTimers.length; index++) {
        if(renderedTimers[index].target > 0) {renderedTimers[index].targetPixelHeight = minTimerHeight + calculateTargetHeight(longestDuration, containerHeight, renderedTimers[index].target)}
        if(index != timerIndex) {renderedTimers[index].pixelHeight = minTimerHeight + calculateDisplayHeight(longestDuration, containerHeight, renderedTimers[index].dailySessionTotal)} 
      }
    }, 16);
  }

  function UTCStringToMillis(UTCString : string | null) {
    if(UTCString) return new Date(UTCString + " UTC").getTime()
    return null;
  }

  function getActiveSession() : Session | null {
    for (let session of dailySessions) {
      if(!session.end) return session;
    }
    return null;
  }

  function calculateSessionDuration(start : number | null, end : number | null) {
    if(!start) throw new Error("session start undefined");
    let dateNow = new Date();
    let millisNow = dateNow.getTime();
    let midnight = dateNow.setHours(0, 0, 0, 0);
    if(end) {
      return start > midnight ? end - start : end - midnight;
    }
    return  start > midnight ? millisNow - start : millisNow - midnight;
  }

  function calculateDisplayHeight(longestDuration : number, containerHeight: number, sessionTotal: number) {
    if(longestDuration < minZoomLevel) longestDuration = minZoomLevel;
    return (containerHeight/longestDuration * sessionTotal);
  }

  function calculateTargetHeight(longestDuration : number, containerHeight: number, target: number) {
    target = target * 60 * 1000;
    if(longestDuration < minZoomLevel) longestDuration = minZoomLevel;
    return (containerHeight/longestDuration * target);
  }

  function getSeconds(millis : number) {
    return Math.floor(millis/1000) % 60;
  }
  function getMinutes(millis : number) {
    return Math.floor(millis/(1000 * 60)) % 60;
  }
  function getHours(millis : number) {
    return Math.floor(millis/(1000 * 60 * 60)) % 24;
  }

  function showSeconds(millis : number) : Boolean {
    // show seconds if time is less than 10 minutes and the number of seconds after a minute is not 0
    return millis < 10 * 60 * 1000 && (getSeconds(millis) > 0 || (millis < 1000 && millis > 0));
  }
  function showMinutes(millis : number) : Boolean {
    // show minutes if time is less than 10 hours and the number of seconds after a minute is not 0
    return getMinutes(millis) > 0 && millis < 60 * 60 * 10 * 1000;
  }
  function showHours(millis : number) : Boolean {
    return getHours(millis) > 0;
  }

  function showTargetHours(target : number) {
    return target >= 60;
  }

  function showTargetMinutes(target : number) {
    return target % 60 != 0;
  }

  function getTargetHours(target: number) {
    let hours = Math.floor(target / 60);
    return hours > 0 ? hours : "";
  }

  function getTargetMinutes(target: number) {
    let minutes = target % 60;
    return minutes > 0 ? minutes : ""
  }

  function setTimerGap(length : number) {
    switch (length) {
      case 1:
        return "36%";
      case 2:
        return "20%";
      case 3:
        return "9%";
      case 4:
        return "4%";
      case 5:
        return "3%";
      case 6:
        return "2%";
      default:
        return "1%";
    }
  }

  function setTimerFontSize(length : number) {
    if(length < 6) return "15px";
    if(length === 6) return "14px";
    if(length === 7) return "13px";
    if(length === 8) return "12px";
    if(length === 9) return "10px";
    return "8px";
  }

  function calculateTimerTextHeight(height : number) {
    let containerHeight = containerElement.getBoundingClientRect().height
    return containerHeight > height ? height : containerHeight;
  }
</script>

<style>
.timer {
  height: 50vh;
  background: #1e1e1e;
  border-radius:8px;
  display: flex;
  flex-direction: row;
  align-items:flex-end;
  justify-content: space-around;
  gap: var(--gap);
  padding: 0 var(--gap);
  overflow: hidden;
}

.timer-bar-container {
  height: 100%;
  position: relative;
  flex: 1;
}

.timer-bar-container > div {
  margin: 0;
  position: relative;
  height: 100%;
  display: flex;
  align-items: center;
  flex-direction: column;
  justify-content: end;
}

.target {
  position: absolute;
  width: 100%;
  background: var(--color);
  border-radius: 8px 8px 0px 0px;
  color: #1e1e1e;
  opacity: 0.15;
}

.target > div {
  position: absolute;
  bottom: 0;
  width: 100%;
  height: 200px;
}

.timercount,
.targetcount {
  line-height: 24px;
  height: 24px;
  font-family: "latobold";
  color: #121212;
  margin: 0;
  padding: 0;
  font-size: var(--font-size);
  text-align: center;
  width: 100%;
}

.timercount > span,
.targetcount > span {
  font-family: "poppinsmedium";
  font-size: calc(var(--font-size) - 3px);
  margin: 0;
}

.timerbar {
  position: relative;
  box-sizing: border-box;
  background: var(--color);
  border-radius: 10px 10px 1px 1px;
  width: 100%;
}
</style>

<div class="timer" style="--gap: {setTimerGap(renderedTimers.length)}" bind:this={containerElement}>
  {#each renderedTimers as timer (timer.id)}
    <div class="timer-bar-container">
      <div>
        <div class="target" style="--color: #{timer.color}; height: {timer.targetPixelHeight}px">
          <div style="height: {calculateTimerTextHeight(timer.targetPixelHeight)}px;">
            <p class="targetcount" style="--font-size: {setTimerFontSize(renderedTimers.length)};">
              {#if showTargetHours(timer.target)}
                {getTargetHours(timer.target)}<span>h</span>
              {/if}
              {#if showTargetMinutes(timer.target)}
                {getTargetMinutes(timer.target)}<span>m</span>
              {/if}
            </p>
          </div>
        </div>
        <div class="timerbar" id="timer{timer.id}" style="--color: #{timer.color}; height: {timer.pixelHeight}px">
          <p class="timercount" style="--font-size: {setTimerFontSize(renderedTimers.length)}">
            {#if showHours(timer.dailySessionTotal)}
              {getHours(timer.dailySessionTotal)}<span>h</span>
            {/if}
            {#if showMinutes(timer.dailySessionTotal)}
              {getMinutes(timer.dailySessionTotal)}<span>m</span>
            {/if}
            {#if showSeconds(timer.dailySessionTotal)}
              {getSeconds(timer.dailySessionTotal)}<span>s</span>
            {/if}          
          </p>
        </div>
      </div>
    </div>  
  {/each}
</div>