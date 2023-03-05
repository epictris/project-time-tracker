<script lang="ts">
	import { scale } from "svelte/transition";
  import { cubicInOut } from "svelte/easing"
  import { fade } from "svelte/transition";
	import Overlay from "./Overlay.svelte";
	import Popup_SetTarget from "./Popup_SetTarget.svelte";
  import { activeProjects } from '../../src/routes/stores';
  import { DB_getActiveProjects } from '../scripts/queries';
  import { DB_createProject } from '../scripts/queries';
	import PopupSetTarget from "./Popup_SetTarget.svelte";
  import { createEventDispatcher } from "svelte";
  
let targetWindow : any = null;
let dispatch = createEventDispatcher();

function openTargetWindow() {
  targetWindow = PopupSetTarget
}

let color : string = "9c6cf0";
$: colorDisplay = `#${color}`

let targetMinutes = 0
$: targetMinutesDisplay = targetMinutes % 60;
$: targetHoursDisplay = Math.floor(targetMinutes / 60);

let projectName : string = "";

async function createNewProject() {
  DB_createProject(projectName, color, targetMinutes).then(() => {
    DB_getActiveProjects().then((projectsList) => {
      $activeProjects = JSON.parse(projectsList);
    });
    dispatch("close");
    }
  )
}

</script>

<style>
  .container {
    width: 100vw;
    height: 100vh;
    display: flex;
    justify-content: center;
    align-items: center;
    pointer-events: none;
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

  #colors {
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

  #submit {
    margin-top: 15px;
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

  #submit:hover {
    background: #1e1e1e;
    color: #fff;
  }
</style>

<div in:fade="{{delay:250, duration: 250, easing: cubicInOut}}" out:fade="{{duration: 250, easing: cubicInOut}}" class="container">
  <div class="window">
    <input id="project-name" type="text" placeholder="Project Name" style="--color: {colorDisplay}" bind:value={projectName}>
    <div id="colors">
      <input on:click={() => color = '9c6cf0'} type="radio" checked name="color" style="--color: #9c6cf0">
      <input on:click={() => color = '84d1e0'} type="radio" name="color" style="--color: #84d1e0">
      <input on:click={() => color = '92d65a'} type="radio" name="color" style="--color: #92d65a">
      <input on:click={() => color = 'f56f49'} type="radio" name="color" style="--color: #f56f49">
      <input on:click={() => color = 'f473a2'} type="radio" name="color" style="--color: #f473a2">
    </div>
    <div id="colors">
      <input on:click={() => color = '558cf2'} type="radio" name="color" style="--color: #558cf2">
      <input on:click={() => color = '93faa5'} type="radio" name="color" style="--color: #93faa5">
      <input on:click={() => color = 'f7a145'} type="radio" name="color" style="--color: #f7a145">
      <input on:click={() => color = 'fc5868'} type="radio" name="color" style="--color: #fc5868">
      <input on:click={() => color = 'd178ee'} type="radio" name="color" style="--color: #d178ee">
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
    <input on:click={createNewProject} type="button" id="submit" value="Create New Project">
  </div>
</div>

<svelte:component this={targetWindow} on:close={() => {targetWindow = null}} bind:targetMinutes={targetMinutes} {targetHoursDisplay} {targetMinutesDisplay} />