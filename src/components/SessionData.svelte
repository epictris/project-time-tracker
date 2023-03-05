<script lang="ts">
	import { scale } from "svelte/transition";
  import { cubicInOut } from "svelte/easing"
  import { fade } from "svelte/transition";
	import Overlay from "./Overlay.svelte";
	import Popup_SetTarget from "./Popup_SetTarget.svelte";
  import { DB_saveProject, DB_saveSession } from '../scripts/queries';
  import { DB_archiveProject } from '../scripts/queries';
	import type { Project, ProjectSession } from "../scripts/objects";
  import { createEventDispatcher, onDestroy, onMount } from "svelte";
  import { canOpenMenu } from "../routes/stores";
  import ConfirmDeletion from "./ConfirmDeletion.svelte";
	import DateSelect from "./DateSelect.svelte";

  const dispatch = createEventDispatcher();

export let selectedSession! : ProjectSession;

let sessionStart = selectedSession.start;
let sessionEnd = selectedSession.end;

console.log(sessionStart);

let popupConfirm : any;
let selectedDate : string | null;
let selectStart : any;
let selectEnd : any;

$: renderedStart = renderDateTime(sessionStart);
$: renderedEnd = renderDateTime(sessionEnd);

onMount(() => {
  $canOpenMenu = false;
})

onDestroy(() => {
  $canOpenMenu = true;
})

function saveSession() {
  DB_saveSession(selectedSession.id, sessionStart, sessionEnd).then(() => {
    dispatch("close");
  })
}

function renderDateTime(date : any) {
  let dateTime = new Date(UTCStringToMillis(date)!)
  let sign = dateTime.getHours() < 12 ? "AM" : "PM";
  let hours = ("0" + (dateTime.getHours() % 12)).slice(-2);
  if (hours == "00") {hours = "12"}
  let minutes = ("0" + (dateTime.getMinutes())).slice(-2);
  let seconds = ("0" + (dateTime.getSeconds())).slice(-2);
  let month = dateTime.toDateString().slice(3, 7);
  let day = ("0" + dateTime.getDate()).slice(-2);
  let year = dateTime.getFullYear().toString().slice(2, 4);

  return hours + ":" + minutes + ":" + seconds + " " + sign + " - " + day + " " + month + " " + year;
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

  .container {
    width: 100vw;
    height: 100vh;
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .container:has(input[type="text"]:focus) {
    pointer-events: all;
  }

  .window {
    width: 80%;
    background: #1e1e1e;
    border-radius: 10px;
    pointer-events: all;
    padding: 10px;
    font-family: 'poppinsregular', sans-serif;

  }

.session {
  flex: 1;
  line-height: 40px;
  padding: 0 10px;
  font-family: "poppinsregular";
  font-size: 12pt;
  border-radius: 8px;
  color: var(--color);
  border: 2px solid var(--color);
  background: none;
  transition: all 200ms ease;
  display: flex;
  justify-content: space-between;
}

.name {
  text-align: left;
}

.date {
  text-align: right;
}

.time {
  display: flex;
  color: #fff;
  margin-top: 15px;
}

.time > p {
  flex: 1;
  margin: 0;
  line-height: 40px;
  padding: 0 10px;
}

.time > button {
  background: #1e1e1e;
  color: #fff;
  font-family: "poppinsregular";
  text-align: center;
  flex: 5;
  border: 2px solid #fff;
  border-radius: 8px;
  font-size: 12pt;
  line-height: 40px;
  padding: 0 10px;
}

#submission {
  margin-top: 15px;
  display: flex;
  gap: 10px;
}

#submission > input {
    font-family: "poppinsregular";
    width: 100%;
    line-height: 40px;
    padding: 0 10px;
    font-size: 14pt;
    border: 2px solid;
    border-radius: 8px;
    display: block;
    color: #1e1e1e;
  }

  #delete {
    background-color: #fc5868;
  }
</style>

<Overlay on:close={() => dispatch("close")}>
  <div in:fade="{{duration: 250, easing: cubicInOut}}" out:fade="{{duration: 250, easing: cubicInOut}}" class="container">
    <div class="window">
      <div class="session" style="--color: #{selectedSession.color}" id="s{selectedSession.id}" data-id={selectedSession.id}>
        <div class="name">
          {selectedSession.name}
        </div>
        <div class="details">
          <span class="date">
            {renderDate(sessionStart)}
          </span>
        </div>
      </div>
      <div class="time">
        <p>Start</p> 
        <button on:click={() => {selectedDate = sessionStart; selectStart = DateSelect}}>{renderedStart}</button>
      </div>
      <div class="time">
        <p>End</p>
        <button on:click={() => {selectedDate = sessionEnd; selectEnd = DateSelect}}>{renderedEnd}</button>
      </div>
      <div id="submission">
        <input type="button" id="delete" value="Delete" on:click={() => popupConfirm = ConfirmDeletion}>
        <input type="button" id="update" value="Update" on:click={saveSession}>
      </div>
    </div>
  </div>
</Overlay>

<svelte:component this={selectStart} bind:selectedDate={sessionStart} on:close={() => {selectStart = null}}/>
<svelte:component this={selectEnd} bind:selectedDate={sessionEnd} on:close={() => {selectEnd = null}}/>

<svelte:component this={popupConfirm} {selectedSession} on:close={() => {popupConfirm = null}} on:delete={() => {dispatch("close")}} />