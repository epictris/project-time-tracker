<script lang="ts">
	import { scale } from "svelte/transition";
  import { cubicInOut } from "svelte/easing"
  import { fade } from "svelte/transition";
	import Overlay from "./Overlay.svelte";
	import Popup_SetTarget from "./Popup_SetTarget.svelte";
  import { activeProjects } from '../routes/stores';
  import { DB_getActiveProjects } from '../scripts/queries';
  import { DB_saveProject } from '../scripts/queries';
  import { DB_archiveProject } from '../scripts/queries';
	import type { Project } from "../scripts/objects";
  import { createEventDispatcher, onDestroy, onMount } from "svelte";
  import { canOpenMenu } from "../routes/stores";

  const dispatch = createEventDispatcher();

let targetWindow : any = null;

export let selectedProject! : Project;

function openTargetWindow() {
  targetWindow = Popup_SetTarget
}

onMount(() => {
  $canOpenMenu = false;
  for(let color of document.getElementsByClassName("color")) {
  if((color as HTMLElement).dataset.color === selectedProject.color) {
    (color as HTMLInputElement).checked = true;
  }
}
})

onDestroy(() => {
  $canOpenMenu = true;
})

let color : string = selectedProject.color;
$: colorDisplay = `#${color}`

let targetMinutes = selectedProject.target;
$: targetMinutesDisplay = targetMinutes % 60;
$: targetHoursDisplay = Math.floor(targetMinutes / 60);

let projectName : string = selectedProject.name;

function saveProject() {
  DB_saveProject(selectedProject.id, projectName, color, targetMinutes)
    .then(() => {
      dispatch("showActive");
      dispatch("close");
    })
}

function archiveProject() {
  DB_archiveProject(selectedProject.id)
    .then(() => {
      dispatch("showActive");
      dispatch("close");
    })
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

  #project-name {
    width: 100%;
    line-height: 40px;
    text-align: center;
    font-size: 14pt;
    border-radius: 8px;
    border: 2px solid var(--color);
    background: none;
    color: var(--color);
    font-family: 'poppinsregular', sans-serif;
  }

  #project-name:focus-visible {
    outline: none;
  }

  .colors {
    margin: 15px 0px;
    width: 100%;
    display: flex;
    justify-content: center;
    gap: 5%;
  }

  input[name="color"] {
    appearance: none;
    margin: 0;
    background: #1e1e1e;
    border-radius: 50%;
    width: 38px;
    height: 38px;
    border: 2px solid var(--color);
  }

  input[name="color"]:checked {
    background: var(--color);
  }
  #daily-target {
    --color: #909090;
    display: flex;
    justify-content: space-evenly;
    align-items: center;
    color: var(--color);
  }

  p {
    font-size: 14pt;
    margin: 0;
  }

  #time {
    width: 110px;
    line-height: 40px;
    font-size: 14pt;
    padding: 0;
    background: #1e1e1e;
    border: 2px solid var(--color);
    border-radius: 8px;
    color: var(--color);
    font-family: 'Poppins', sans-serif;
  }

  .unit {
    font-size: 11pt;
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

  #archive {
    background-color: #fc5868;
  }
</style>

<Overlay on:close={() => {dispatch("close")}}>
  <div in:fade="{{duration: 250, easing: cubicInOut}}" out:fade="{{duration: 150, easing: cubicInOut}}" class="container">
    <div class="window">
      <input id="project-name" type="text" placeholder="Project Name" style="--color: {colorDisplay}" bind:value={projectName}>
      <div class="colors">
        <input class="color" data-color="9c6cf0" on:click={() => color = '9c6cf0'} type="radio" name="color" style="--color: #9c6cf0">
        <input class="color" data-color="84d1e0" on:click={() => color = '84d1e0'} type="radio" name="color" style="--color: #84d1e0">
        <input class="color" data-color="92d65a" on:click={() => color = '92d65a'} type="radio" name="color" style="--color: #92d65a">
        <input class="color" data-color="f56f49" on:click={() => color = 'f56f49'} type="radio" name="color" style="--color: #f56f49">
        <input class="color" data-color="f473a2" on:click={() => color = 'f473a2'} type="radio" name="color" style="--color: #f473a2">
      </div>
      <div class="colors">
        <input class="color" data-color="558cf2" on:click={() => color = '558cf2'} type="radio" name="color" style="--color: #558cf2">
        <input class="color" data-color="93faa5" on:click={() => color = '93faa5'} type="radio" name="color" style="--color: #93faa5">
        <input class="color" data-color="f7a145" on:click={() => color = 'f7a145'} type="radio" name="color" style="--color: #f7a145">
        <input class="color" data-color="fc5868" on:click={() => color = 'fc5868'} type="radio" name="color" style="--color: #fc5868">
        <input class="color" data-color="d178ee" on:click={() => color = 'd178ee'} type="radio" name="color" style="--color: #d178ee">
      </div>

      {#if targetMinutes > 0}
      <div id="daily-target" style="--color: #EECB43">
        <p>Daily Target</p>
        <button id="time" on:click={openTargetWindow}>
          {#if targetHoursDisplay > 0 && targetMinutesDisplay > 0}
            {targetHoursDisplay}<span class="unit">h </span>
            {targetMinutesDisplay}<span class="unit">m</span>
          {:else}
            {#if targetHoursDisplay > 0}
              {targetHoursDisplay}<span class="unit">h</span>
            {/if}
            {#if targetMinutesDisplay > 0}
              {targetMinutesDisplay}<span class="unit">m</span>
            {/if}
          {/if}
        </button>
      </div>
      {:else}
        <div id="daily-target">
          <p>Daily Target</p>
          <button id="time" on:click={openTargetWindow}>
            None
          </button>
        </div>
      {/if}
      <div id="submission">
        <input on:click={archiveProject} type="button" id="archive" value="Archive">
        <input on:click={saveProject} type="button" id="save" value="Save">
      </div>
    </div>
  </div>

  <svelte:component this={targetWindow} on:close={() => {targetWindow = null}} bind:targetMinutes={targetMinutes} {targetHoursDisplay} {targetMinutesDisplay} />
</Overlay>