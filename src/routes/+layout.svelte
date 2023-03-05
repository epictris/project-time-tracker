<script lang="ts">
	import { onMount } from "svelte";
  import Overlay from '../components/Overlay.svelte'
  import Menu from "../components/Menu.svelte";
  import { page } from "$app/stores";
  
  $:title = getPageTitle($page.url.pathname)

  function getPageTitle(path : string) {
    switch(path) {
      case "/":
        return "Timer"
      case "/stats":
        return "Stats"
      case "/projects":
        return "Edit Projects"
      case "/sessions":
        return "Edit Sessions"
      case "/archive":
        return "Archived Projects"
    }
  }
  let menu : any = null;
  let duration = 0;
  let hamburger : HTMLElement;

</script>

<style>
  :global(*) {
    box-sizing: border-box;
    -webkit-user-select: none; /* Safari */
    -ms-user-select: none; /* IE 10 and IE 11 */
    user-select: none; /* Standard syntax */
  }

  :global(html) {
    background-color: #121212;
  }

  :global(body) {
    margin: 0;
  }
  
  :global(::-webkit-scrollbar) {
    display: none;
  }

  #header {
    text-align: center;
    padding: 20px 20px;
    display: flex;
    align-items: center;
    flex-flow: row-reverse;
  }

  #title {
    font-family: "poppinsregular";
    width: 100%;
    color: #fff;
    text-align: center;
    font-size: 14pt;
    line-height: 14pt;
  }

  #hamburger {
    position: absolute;
    width: 24px;
    height: 20px;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
  }

  #hamburger > div {
    background: #d3d3d3;
    width: 100%;
    height: 3px;
    border-radius: 5px;
  }

  #page {
    padding: 0 10px;
  }

</style>

<link href="/src/assets/css/fonts.css" rel="stylesheet">

<div id="header">
  <div id="title">{title}</div>
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <div id="hamburger" on:click={() => {menu = Menu}} style="--duration: {duration}ms" bind:this={hamburger} on:click={() => { duration = 250}}>
      <div id="top"/>
      <div id="middle"/>
      <div id="bottom"/>
    </div>
  </div>
<div id="page">
  <slot/>
</div>
<svelte:component this={menu} on:close={() => {menu = null}}/>