<script lang="ts">
	import { scale } from "svelte/transition";
  import { cubicInOut } from "svelte/easing"
  import { fade } from "svelte/transition";
	import Overlay from "./Overlay.svelte";
	import Popup_SetTarget from "./Popup_SetTarget.svelte";
  import { DB_saveProject } from '../scripts/queries';
  import { DB_unarchiveProject } from '../scripts/queries';
	import type { Project } from "../scripts/objects";
  import { createEventDispatcher, onDestroy, onMount } from "svelte";
  import DeleteProjectPopup from "./DeleteProjectPopup.svelte";
  import { canOpenMenu } from "../routes/stores";

  const dispatch = createEventDispatcher();

  export let selectedProject! : Project;

  let confirmationPopup : any;

  onMount(() => {
    $canOpenMenu = false
  })

  onDestroy(() => {
    $canOpenMenu = true
  })

  function deleteProject() {
    confirmationPopup = DeleteProjectPopup
  }

  function unarchiveProject() {
    DB_unarchiveProject(selectedProject.id)
    .then(() => {
      dispatch("refreshData");
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
    vertical-align: middle;
    line-height: 40px;
    text-align: center;
    font-size: 14pt;
    border-radius: 8px;
    border: 2px solid var(--color);
    background: none;
    color: var(--color);
    font-family: 'poppinsregular', sans-serif;
  }

  #submission {
    margin-top: 15px;
    display: flex;
    gap: 10px;
    align-items: stretch;
    justify-content: stretch;
  }

  #submission > input {
    height: 100%;
    flex: 1;
    font-family: "poppinsregular";
    width: 100%;
    margin: 0 auto;
    line-height: 40px;
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

<Overlay on:close={() => dispatch("close")}>
  <div in:fade="{{duration: 250, easing: cubicInOut}}" out:fade="{{duration: 250, easing: cubicInOut}}" class="container">
    <div class="window">
      <div id="project-name" style="--color: #{selectedProject.color}">{selectedProject.name}</div>
      <div id="submission">
        <input on:click={deleteProject} type="button" id="archive" value="Delete">
        <input on:click={unarchiveProject} type="button" id="save" value="Unarchive">
      </div>
    </div>
  </div>
</Overlay>

<svelte:component this={confirmationPopup} {selectedProject} on:close={() => confirmationPopup = null} on:deleted={() => dispatch("deleted")}/>