<script lang="ts">
	import { scale } from "svelte/transition";
  import { cubicInOut } from "svelte/easing"
  import { fade } from "svelte/transition";
	import Overlay from "./Overlay.svelte";
	import Popup_SetTarget from "./Popup_SetTarget.svelte";
  import { DB_deleteProject, DB_saveProject } from '../scripts/queries';
  import { DB_unarchiveProject } from '../scripts/queries';
	import type { Project } from "../scripts/objects";
  import { createEventDispatcher, onDestroy, onMount } from "svelte";

  const dispatch = createEventDispatcher();

  export let selectedProject! : Project;

  function deleteProject() {
    DB_deleteProject(selectedProject.id).then(() => {
      dispatch("deleted")
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


  .message {
    color: #fff;
    line-height: 19pt;
    font-size: 12pt;
    text-align: center;
    margin: 0;
  }

  #submission {
    margin-top: 8px;
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
  <div in:fade="{{duration: 250, easing: cubicInOut}}" out:fade="{{duration: 150, easing: cubicInOut}}" class="container">
    <div class="window">
      <p class="message">
        Delete <span style="color: #{selectedProject.color}">{selectedProject.name}</span> ? <br>All historical data will be lost.
      </p>
      <div id="submission">
        <input on:click={deleteProject} type="button" id="archive" value="Yes, Delete">
        <input on:click={() => dispatch("close")} type="button" id="save" value="Wait no">
      </div>
    </div>
  </div>
</Overlay>