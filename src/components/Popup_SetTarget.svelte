<script lang="ts">
  import { cubicInOut } from "svelte/easing"
  import { fade } from "svelte/transition";
	import Overlay from "./Overlay.svelte";
  import createScrollSnap from "scroll-snap"
	import { onMount } from "svelte";
  import { createEventDispatcher } from "svelte";

  let dispatch = createEventDispatcher();

  export let targetMinutes;

  export let targetHoursDisplay : number;
  export let targetMinutesDisplay : number;

  let timeElement: HTMLElement;
  let hoursElement: HTMLElement;
  let minutesElement: HTMLElement;


  let hours = [...Array(13).keys()];
  let minutes = [...Array(12).keys()].map(n => n * 5);

  // let d = document.createEvent("TouchEvent");

  onMount(() => {
    scrollToElement(document.getElementById("h" + targetHoursDisplay.toString()) as HTMLElement)
    scrollToElement(document.getElementById("m" + targetMinutesDisplay.toString()) as HTMLElement)
  });

  function distance(x1 : number, y1 : number, x2 : number, y2 : number) {
    return Math.sqrt((x2-x1)*(x2-x1)+(y2-y1)*(y2-y1))
  }
    // Update when scrolling

    // var isScrolling : any;
    // hoursElement.addEventListener('scroll', function ( event ) {
    //   window.clearTimeout( isScrolling );
    //   isScrolling = setTimeout(function() {
    //     if(getSelectedElement(hoursElement).dataset.hour == "24") {
    //       minutes = [0];
    //     }
    //     else {
    //       minutes= [...Array(60).keys()]
    //     }
    //   }, 100);
    // }, false);
  // })

  function scrollToElement(element : HTMLElement | EventTarget | null, smoothScroll: Boolean = false) {
    if(!element) return;

    (element as HTMLElement).scrollIntoView({
      behavior: smoothScroll ? "smooth" : "auto",
      block: "center",
      inline: "center"
    })
  }

  function getSelectedElement(parent : HTMLElement) : HTMLElement {
      let rect = parent.getBoundingClientRect();
      return document.elementFromPoint(rect.x + rect.width/2, rect.y + rect.height/2) as HTMLElement
    }

  function setTargetTime() {
    let selectedHours = getSelectedElement(hoursElement).dataset.hour
    let selectedMinutes = getSelectedElement(minutesElement).dataset.minute

    targetMinutes = Number(selectedHours) * 60 + Number(selectedMinutes);
    console.log("TEST");
    dispatch("close");
  }
  
</script>

<style>
  * {
    --entry-height: 45px;
    --entries-shown: 5;
    --font-size-value: 18pt;
    --font-size-unit: 13pt;
  }

  .container {
    width: 100vw;
    height: 100vh;
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .window {
    width: 60%;
    background: #1e1e1e;
    border-radius: 10px;
    pointer-events: all;
    font-family: 'poppinsregular', sans-serif;
  }

  #submit-container {
    padding: 10px;
  }

  #submit {
    font-family: 'Poppins', sans-serif;
    display: block;
    width: 100%;
    padding: 5px;
    line-height: 20pt;
    background: #fff;
    color: #1e1e1e;
    font-size: 14pt;
    border: 2px solid #fff;
    border-radius: 8px;
  }

  #submit:hover {
    background: #1e1e1e;
    color: #fff;
  }

  #time {
    display: flex;
    justify-content: space-evenly;
    position: relative;
  }

  #time::before,
  #time::after {
    content: "";
    width: 100%;
    height: calc(var(--entry-height)*2);
    position: absolute;
    pointer-events: none;
  }

  #time::before {
    border-radius: 10px;
    top: -2px;
    background-image: linear-gradient(
       #1e1e1eFF,
       #1e1e1eFF,
       #1e1e1eEE,
       #1e1e1eBB,
       #1e1e1eBB,
       #1e1e1e99,
       #1e1e1e00
       );
  }

  #time::after {
    bottom: -2px;
    background-image: linear-gradient(
    #1e1e1e00, 
    #1e1e1e99,
    #1e1e1eBB, 
    #1e1e1eBB, 
    #1e1e1eEE, 
    #1e1e1eFF, 
    #1e1e1eFF
    );
  }

  #time > div {
    /* pointer-events: none; */
    width: 50%;
    scroll-snap-type: y mandatory;
    color: #EECB43;
    height: calc(var(--entry-height) * 5);
  }

  #time > div > p {
    font-size: var(--font-size-value);
    line-height: var(--entry-height);
    margin: 0;
    height: var(--entry-height);
    scroll-snap-align: center;
  }

  #time span {
    pointer-events: none;
    font-size: var(--font-size-unit);
  }

  #hours {
    text-align: center;
    height: 40px;
    overflow: scroll;
  }

  #minutes {
    text-align: center;
    height: 40px;
    overflow: scroll;
  }
</style>
 <!-- svelte-ignore a11y-click-events-have-key-events -->
<Overlay on:close={() => dispatch("close")}>
  <div class="container" in:fade={{duration: 250}} out:fade={{duration: 250}}>
    <div class="window">
      <div id="time" bind:this={timeElement}>
        <div id="hours" bind:this={hoursElement}>
            <p></p>
            <p></p>
          {#each hours as hour}
            <p on:click={(e) => {scrollToElement(e.target, true)}} id="h{hour}" data-hour="{hour}">{hour}<span>h</span></p>
          {/each}
          <p></p>
          <p></p>
        </div>
        <div id="minutes" bind:this={minutesElement}>
          <p></p>
          <p></p>
          {#each minutes as minute}
            <p on:click={(e) => {scrollToElement(e.target, true)}} id="m{minute}" data-minute="{minute}">{minute}<span>m</span></p>
          {/each}
          <p></p>
          <p></p>
        </div>
      </div>
      <div id="submit-container">      
        <button id="submit" on:click={setTargetTime}>Set</button>
      </div>
    </div>
  </div>
</Overlay>