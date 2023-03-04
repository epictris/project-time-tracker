<script lang=ts>
  import Overlay from "./Overlay.svelte";
  import { page } from "$app/stores";
	import Popup_ProjectData from "./Popup_ProjectData.svelte";
	import { scale, slide } from "svelte/transition";
  import { cubicInOut } from "svelte/easing";
  import { backOut } from "svelte/easing";
  import plus_filled from '../assets/images/plus_filled.svg'
  import plus_empty from '../assets/images/plus_empty.svg'
  import { canOpenMenu } from "../routes/stores";

  let openDistance = 150;
  let openBuffer = 20;

  let startDragY : number | null = null;

  let overlayOpacity : String;

  let closing = false;

  export const MenuState = {
    CLOSED: 0,
    BUFFER: 1,
    OPENING: 2,
    OPEN: 3
  }

  const MenuPage = {
    MAIN: 0,
    CREATE_PROJECT: 1,
    NAVIGATION: 2
  }

  let menuState = MenuState.CLOSED;

  function isClosed() : Boolean {
    return menuState == MenuState.CLOSED;
  }
  function isBuffer() : Boolean {
    return menuState == MenuState.BUFFER;
  }
  function isOpening() : Boolean {
    return menuState == MenuState.OPENING;
  }
  function isOpen() : Boolean {
    return menuState == MenuState.OPEN;
  }

  export let menuPage = MenuPage.MAIN;

  window.addEventListener('touchstart', (e) => {
    if($canOpenMenu && !closing && window.scrollY == 0 && (isClosed() || isBuffer()) && startDragY == null) {
      startDragY = e.touches[0].pageY;
    }
  });

  window.addEventListener('touchmove', (e) => {
    if(window.scrollY != 0) {startDragY = null;}

    if (!$canOpenMenu || closing || startDragY == null || window.scrollY != 0 || e.touches.length != 1 || isOpen()) return; // If multiple fingers are touching
      let dragDistance = e.touches[0].pageY - startDragY
    if(dragDistance < openBuffer) {
      menuState = MenuState.BUFFER;
      return;
    }

    if(isBuffer()){
      menuState = MenuState.OPENING;
    }
    applyOverlayOpenFade(dragDistance);
  });

  window.addEventListener('touchend', (e) => {
    if (!$canOpenMenu || closing || startDragY == null || isOpen() || e.touches.length > 1) return; // If multiple fingers are touching
    
    if(e.changedTouches[0].pageY - startDragY > openDistance) {
      menuPage = MenuPage.MAIN;
      menuState = MenuState.OPEN;
    }
    else {
      menuState = MenuState.CLOSED;
    }
    startDragY = null;
  });

  function applyOverlayOpenFade(dragDistance : number) {
    if(dragDistance < openBuffer) {
      overlayOpacity = "0";
      return;
    }
    overlayOpacity = dragDistance > openDistance? "1" : ((dragDistance - openBuffer) * (100 / (openDistance - openBuffer))/100).toString()
  }

  function handleClose() {
    menuState = MenuState.CLOSED;
    menuPage = MenuPage.MAIN;
  }
</script>

<style>
  #plus {
    filter: invert(100%);
    border: none;
    width: 70px;
    height: 70px;
    position: relative;
    pointer-events: all;
    margin-top: 80vh;
  }

  #plus img {
    width: 100%;
    height: 100%;
    display: inline-block;
    position: absolute;
  }

  #filled {
    opacity: 1;
  }

  #unfilled {
    border-radius: 50%;
    opacity: 0;
  }

  #plus:hover #filled{
    opacity: 0;
    transition: opacity 150ms ease-in-out;
  }

  #plus:hover #unfilled {
    opacity: 1;
    transition: opacity 150ms ease-in-out;
  }

  #header {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: flex-end;
    height: 60px;
    padding: 0 20px 0 20px;
  }

  .nav {
    filter: invert(100%);
    border: none;
    width: 35px;
    height: 40px;
    pointer-events: all;
  }

  #actions {
    width: 100vw;
    height: 100vh;
    position: absolute;
    top: 0;
    display: flex;
    justify-content: center;
  }

  .menu-container {
    display: flex;
    align-items: center;
    background-color: #1e1e1e;
    padding: 10px;
    border-radius: 0 0 8px 8px;
  }

  #menu {
    width: 100%;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }

  .menu-entry {
    color: #fff;
    font-family: "poppinsregular";
    font-size: 16pt;
    padding: 15px;
    border-radius: 50%;
    pointer-events: all;
    text-decoration: none;
    -webkit-tap-highlight-color: transparent; /* for removing the highlight */
  }
</style>

<!-- svelte-ignore a11y-click-events-have-key-events -->
{#if menuState == MenuState.OPENING || menuState == MenuState.OPEN}
  <Overlay customOpen={true} bind:opacity={overlayOpacity} on:closeOverlay={handleClose} bind:closing={closing}>
    {#if menuState == MenuState.OPEN}

      {#if menuPage == MenuPage.MAIN}
        <div class="menu-container" in:slide="{{duration: 150, easing: backOut}}"  out:slide="{{duration: 150, easing: cubicInOut}}">
          <div id="menu">
            <a class="menu-entry" href="/" on:click={handleClose}>Timer</a>
            <a class="menu-entry" href="/reports" on:click={handleClose}>Reports</a>
            <a class="menu-entry" href="/projects" on:click={handleClose}>Projects</a>
            <a class="menu-entry" href="/sessions" on:click={handleClose}>Sessions</a>
          </div>
        </div>
        <div id="actions">
          {#if $page.url.pathname == "/"}
            <div id="plus" in:scale="{{duration: 150, easing: backOut, start: 0}}" out:scale="{{duration: 150, easing: cubicInOut, start: 0}}" on:click={() => menuPage = MenuPage.CREATE_PROJECT}>
              <img src="{plus_empty}" alt="unfilled" id="unfilled">
              <img src="{plus_filled}" alt="filled" id="filled">
            </div>
          {/if}
        </div>
      {/if}

      {#if menuPage == MenuPage.CREATE_PROJECT}
        <Popup_ProjectData bind:menuState={menuState}/>
      {/if}

      {#if menuPage == MenuPage.NAVIGATION}
        <div id="header">
          <button class="nav open" on:click={() => menuPage = MenuPage.MAIN} in:scale="{{duration: 150, easing: backOut, start: 0}}" out:scale="{{duration: 150, easing: cubicInOut, start: 0}}"></button>
        </div>
      {/if}
    {/if}
  </Overlay>
{/if}


