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

let sessions : ProjectSession[] = [];
const SESSIONS_PER_PAGE = 20;
let hasMore = true;

async function loadSessions(startAt : number) {
  DB_loadNSessions(startAt, SESSIONS_PER_PAGE).then((data) => {
    if(data.length > 0) {
      sessions = [...sessions, ...(JSON.parse(data))]
    } else {
      hasMore = false;
    }
  })
}

let popupWindow : any;
let selectedSession : ProjectSession | undefined;

async function handleClose() {
  popupWindow = null
  sessions = [];
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

function UTCStringToMillis(UTCString : string | null) {
    if(UTCString) return new Date(UTCString + " UTC").getTime()
    return null;
  }

</script>

<style>
:global(body) {
  padding: 10px;
}

.session {
  flex: 1;
  line-height: 45px;
  padding: 0px 15px;
  font-family: "poppinsregular";
  font-size: 12pt;
  border-radius: 8px;
  color: var(--color);
  margin: 8px 0;
  background: none;
  transition: all 200ms ease;
  display: flex;
  justify-content: space-between;
  background: #1e1e1e;
  box-shadow: 1px 1px 3px #000;
}

.name {
  text-align: left;
  pointer-events: none;
}

.details {
  pointer-events: none;
}

.date {
  text-align: right;
}

</style>

{#each sessions as session } 
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <div class="session" style="--color: #{session.color}" id="s{session.id}" data-id={session.id} on:click={(e) => handleSessionClick(e.target)}>
    <div class="name">
      {session.name}
    </div>
    <div class="details">
      <span class="date">
        {renderDate(session.start)}
      </span>
    </div>
  </div>
{/each}
<div use:inview={{}} on:change={handleChange}></div>

<svelte:component this={popupWindow} {selectedSession} on:close={handleClose}/>