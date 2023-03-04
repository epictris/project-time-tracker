<script lang="ts">

  import { cubicInOut } from "svelte/easing"
  import { fade } from "svelte/transition";
  import { createEventDispatcher, onDestroy, onMount } from "svelte";

  const dispatch = createEventDispatcher();

  let container : HTMLElement;
  let background : HTMLElement;

  export let customOpen = false;
  export let opacity : String = "0";

  let draggingBackground = false;
  let touchLocation = {x: 0, y: 0};
  const dragBuffer = 10;

  export let closing = false

  onMount(() => {
    document.body.style.overflow = "hidden";
  })

  onDestroy(() => {
    document.body.style.overflowY = "scroll";
    closing = false
  })

  function distance(x1 : number, y1 : number, x2 : number, y2 : number) {
    return Math.sqrt((x2-x1)*(x2-x1)+(y2-y1)*(y2-y1))
  }
</script>
<style>
  #container {
    width: 100vw;
    height: 100vh;
    position:fixed;
    top: 0;
    left: 0;
  }

  #background {
    width: 100%;
    height: 100%;
    background: #00000099;
    position: absolute;
    pointer-events: all;
  }

  #overlay-content {
    position: relative;
    pointer-events: none;
  }
</style>


<!-- svelte-ignore a11y-click-events-have-key-events -->
{#if customOpen}
  <div id="container" out:fade="{{duration: 250, easing: cubicInOut}}" style="opacity: {opacity}" bind:this={container}>
    <div id="background" 
      on:touchstart={(e)=>{
        e.preventDefault(); 
        draggingBackground = false;
        touchLocation = {x: e.touches[0].clientX, y:e.touches[0].clientY}
      }} 
      on:touchmove={(e)=>{
        e.preventDefault(); 
        if(draggingBackground) return;
        let d = distance(e.touches[0].clientX, e.touches[0].clientY, touchLocation.x, touchLocation.y);
        if(d > dragBuffer) {
          draggingBackground = true;
      }}} 
      on:touchend={()=>{
          if(draggingBackground) return;
          closing = true
          dispatch('closeOverlay');
      }} 
      bind:this={background}>
    </div>    
    <div id="overlay-content">
      <slot />
    </div>
  </div>
{:else}
  <div id="container" out:fade="{{duration: 250, easing: cubicInOut}}" in:fade="{{duration: 250, easing: cubicInOut}}" bind:this={container}>
    <div id="background" on:touchmove|preventDefault
      on:touchstart={(e)=>{
        e.preventDefault();
        if(closing) return;
        draggingBackground = false;
        touchLocation = {x: e.touches[0].clientX, y:e.touches[0].clientY}
      }} 
      on:touchmove={(e)=>{
        e.preventDefault(); 
        if(draggingBackground || closing) return;
        let d = distance(e.touches[0].clientX, e.touches[0].clientY, touchLocation.x, touchLocation.y);
        if(d > dragBuffer) {
          draggingBackground = true;
      }}} 
      on:touchend={(e)=>{
          e.preventDefault()
          if(draggingBackground || closing) return;
          closing = true
          dispatch('closeOverlay');
      }} 
      bind:this={background}>
    </div>
    <div id="overlay-content">
      <slot />
    </div>
  </div>
{/if}