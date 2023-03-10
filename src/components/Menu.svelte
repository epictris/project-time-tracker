<script lang="ts">
import { createEventDispatcher } from "svelte";
	import { cubicInOut } from "svelte/easing";
	import { fly, slide } from "svelte/transition";
	import Overlay from "./Overlay.svelte";
	import PopupProjectData from "./Popup_ProjectData.svelte";
  import { page } from "$app/stores";
  import columnChart from "../assets/images/column-chart.svg"
  import pieChart from "../assets/images/pie-chart.svg"
  import hammer from "../assets/images/hammer.svg"
  import time from "../assets/images/time.svg"
  import archive from "../assets/images/archive.svg"
  import plus from "../assets/images/plus-filled.svg"

  let dispatch = createEventDispatcher()
  let creatingProject : boolean;
  $:pageName = $page.url.pathname;

</script>
<Overlay on:close={() => dispatch("close")}>
  {#if !creatingProject}
    <div id="menu" in:fly={{x: -200, opacity: 1, duration: 250, easing: cubicInOut}} out:fly={{x: -200, opacity: 1, duration: 250, easing: cubicInOut}}>
      <a on:click={(e) => dispatch("close")} href="/" class="entry {pageName == "/" ? "selected" : ""}">
        <img src={columnChart} alt="timer icon"/> <p>Timer</p>
      </a>
      <a on:click={(e) => dispatch("close")} href="/stats" class="entry {pageName == "/stats" ? "selected" : ""}">
        <img src={pieChart} alt="stats icon"/> <span>Stats</span>
      </a>
      <a on:click={(e) => dispatch("close")} href="/projects" class="entry {pageName == "/projects" ? "selected" : ""}">
        <img src={hammer} alt="projects icon"/> <span>Edit Projects</span>
      </a>
      <a on:click={(e) => dispatch("close")} href="/sessions" class="entry {pageName == "/sessions" ? "selected" : ""}">
        <img src={time} alt="session icon"/> <span>Edit Sessions</span>
      </a>
      <a on:click={(e) => dispatch("close")} href="/archive" class="entry {pageName == "/archive" ? "selected" : ""}">
        <img src={archive} alt="archive icon"/> <span>Archived Projects</span>
      </a>
      <div class="separator"/>
      <button class="action" on:click={() => {creatingProject = true}}>
        <img src={plus} alt="plus icon"> <span>Create Project</span>
      </button>
    </div>
  {:else}
    <PopupProjectData on:close={() => dispatch("close")}/>
  {/if}
</Overlay>

<style>
  #menu {
    position: relative;
    z-index: 2;
    display: inline-block;
    height: 100vh;
    background-color: #121212;
    pointer-events: all;
  }

  .separator {
    height: 2px;
    background-color: #1e1e1e;
  }

  .entry, .action {
    transition:250ms all;
    display:block;
    text-decoration: none;
    background: none;
    border: none;
    font-size: 12pt;
    color: #fff;
    font-family: "poppinsregular";
    -webkit-tap-highlight-color: transparent; /* for removing the highlight */
    display: flex;
    align-items: center;
    padding: 15px;
    gap: 20px;
  }

  .entry:hover, .action:hover {
    background: #d3d3d3;
    color: #121212
  }

  .entry:hover img, .action:hover img {
    filter: invert(1) brightness(7%);  
  }

  .entry.selected {
    background: #d3d3d3;
    color: #121212
  }

  .entry.selected img {
    filter: invert(1) brightness(7%);  
  }

  #menu:has(.entry:hover) .entry.selected:not(:hover) {
    background: #121212;
    color: #d3d3d3;
  }

  #menu:has(.entry:hover) .entry.selected:not(:hover) img {
    filter: invert(1) brightness(83%);
  }

  .entry img, .action img{
    transition: all 250ms;
    height: 28px;
    filter: invert(1) brightness(83%);
  }

  .entry, .action{
    vertical-align: baseline;
  }

  .entry p {
    padding: 0;
    margin: 0;
  }
</style>