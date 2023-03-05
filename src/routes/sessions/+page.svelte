<script lang="ts">
	import { DB_getActiveProjects, DB_loadNSessions } from '../../scripts/queries';
	import { DB_getArchivedProjects } from '../../scripts/queries';
  import { activeProjects } from '../stores';
  import type {  Project, ProjectSession, Session } from '../../scripts/objects'
	import type { SvelteComponent } from 'svelte';
	import PopupEditProject from '../../components/Popup_EditProject.svelte';
  import ArchivedProjectOptions from '../../components/ArchivedProjectOptions.svelte';
  import { inview } from "svelte-inview/dist/index";
	import { onMount } from 'svelte';
  import SessionData from '../../components/SessionData.svelte';
  import { getDate, getHours, getMinutes, getMonth, getSeconds, getYear, showHours, showMinutes, showSeconds, UTCStringToMillis } from '../../scripts/helpers';

let sessions : any[] = [];
const SESSIONS_PER_PAGE = 20;
let hasMore = true;

async function loadSessions(startAt : number) {
  DB_loadNSessions(startAt, SESSIONS_PER_PAGE).then((data) => {
    if(startAt == 0) {sessions = [];}
    if(data.length > 0) {
      let newSessions = JSON.parse(data);
      let processedSessions = []
      for(let session of newSessions) {
        session.duration = UTCStringToMillis(session.end)! - UTCStringToMillis(session.start)!
        processedSessions.push(session);
      }
      sessions = [...sessions, ...processedSessions]
    } else {
      hasMore = false;
    }
  })
}

let popupWindow : any;
let selectedSession : ProjectSession | undefined;

async function handleClose() {
  popupWindow = null
  loadSessions(0);
}

function handleSessionClick(target : EventTarget  | null) {
  if (!target) return
  let element : HTMLElement = <HTMLElement>target
  if (!element.dataset) return
  if (!element.dataset.id) return
  selectedSession = sessions.find(session => {
    return session.id === Number(element.dataset.id)
  })
  if (selectedSession) popupWindow = SessionData;
}

function handleChange() {
  loadSessions(sessions.length)
}

function renderDate(date : any) {
  let dateTime = new Date(UTCStringToMillis(date)!)
  let day = ("0" + dateTime.getDate()).slice(-2);
  let month = dateTime.toDateString().slice(3, 7);
  let year = dateTime.getFullYear().toString().slice(2, 4);

  return day + " " + month + " " + year;
}

</script>

<style>

.session {
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

.session:first-child {
  margin: 0;
}

.name {
  text-align: left;
  pointer-events: none;
  flex: 2;
}

.duration {
  pointer-events: none;
  flex: 1;
  text-align: right;
  color: #a9a9a9;
}

.date {
  pointer-events: none;
  flex: 1;
  text-align: right;
  color: #a9a9a9;
  font-size: 10pt;
}

#data-labels {
  padding: 0 15px;
  display: flex;
  color: #A9A9A9;
  font-size: 8pt;
  justify-content: space-between;
  font-family: "poppinsregular"
}

#label-name {
  flex: 2;
}

#label-duration {
  flex: 1;
  text-align: right;
}

#label-date {
  flex: 1;
  text-align: right;
}

.number {
  font-family: "latoregular";
  line-height: 0px;
}

.unit {
  font-size: 8pt;
  line-height: 0px;
}

</style>

<div id="data-labels">
  <p id="label-name">Project</p>
  <p id="label-duration">Duration</p>
  <p id="label-date">Date</p>
</div>

{#each sessions as session } 
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <div class="session" style="--color: #{session.color}" id="s{session.id}" data-id={session.id} on:click={(e) => handleSessionClick(e.target)}>
    <span class="name">
      {session.name}
    </span>
    <span class="duration">
      {#if showHours(session.duration)}
      <span class="number">{getHours(session.duration)}</span><span class="unit">h</span>
      {/if}
      {#if showMinutes(session.duration)}
      <span class="number">{getMinutes(session.duration)}</span><span class="unit">m</span>
      {/if}
      {#if showSeconds(session.duration)}
      <span class="number">{getSeconds(session.duration)}</span><span class="unit">s</span>
      {/if}
    </span>
    <span class="date">
      <span class="number">{getDate(session.start)}</span>
      <span>{getMonth(session.start)}</span>
      <span class="number">{getYear(session.start)}</span>
    </span>
  </div>
{/each}
<div use:inview={{}} on:change={handleChange}></div>

<svelte:component this={popupWindow} {selectedSession} on:close={handleClose}/>