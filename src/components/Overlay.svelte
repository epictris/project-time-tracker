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

  #content-container {
    position: relative;
    pointer-events: none;
  }
</style>

<div id="container">
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <div id="background" in:fade="{{duration: 250, easing: cubicInOut}}"  out:fade="{{duration: 250, easing: cubicInOut}}" on:click={() => {dispatch("close")}}/>
  <div id="content-container">
    <slot/>
  </div>
</div>