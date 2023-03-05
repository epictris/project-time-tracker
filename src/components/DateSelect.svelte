<script lang="ts">
  import { cubicInOut } from "svelte/easing"
  import { fade } from "svelte/transition";
	import Overlay from "./Overlay.svelte";
  import createScrollSnap from "scroll-snap"
	import { onMount } from "svelte";
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher(); 

  export let selectedDate : string;

  let hoursContainer : HTMLElement;
  let minutesContainer : HTMLElement;
  let secondsContainer : HTMLElement;
  let signsContainer : HTMLElement;
  let datesContainer : HTMLElement;
  let monthsContainer: HTMLElement;
  let yearsContainer : HTMLElement;

  function getDaysOfMonth(month : string) {
  switch(month) {
    case "Jan":
      return [...Array(32).keys()].slice(1);
    case "Feb":
      return [...Array(29).keys()].slice(1);
    case "Mar":
      return [...Array(32).keys()].slice(1);
    case "Apr":
      return [...Array(31).keys()].slice(1);
    case "May":
      return [...Array(32).keys()].slice(1);
    case "Jun":
      return [...Array(31).keys()].slice(1);
    case "Jul":
      return [...Array(32).keys()].slice(1);
    case "Aug":
      return [...Array(32).keys()].slice(1);
    case "Sep":
      return [...Array(31).keys()].slice(1);
    case "Oct":
      return [...Array(32).keys()].slice(1);
    case "Nov":
      return [...Array(31).keys()].slice(1);
    case "Dec":
      return [...Array(32).keys()].slice(1);
  }
}

function UTCStringToMillis(UTCString : string | null) {
    if(UTCString) return new Date(UTCString + " UTC").getTime()
    return null;
  }

  function convertToUTCString() {
    
  }

  let hoursElements : any = {}
  let minutesElements : any = {}
  let secondsElements : any = {}
  let signsElements : any = {}
  let datesElements : any = {}
  let monthsElements : any = {}
  let yearsElements : any = {}

  let dateTime = new Date(UTCStringToMillis(selectedDate)!)
  let sign = dateTime.getHours() < 12 ? "AM" : "PM";
  let hour = ("0" + (dateTime.getHours() % 12)).slice(-2);
  if (hour == "00") {hour = "12"}
  let minute = ("0" + (dateTime.getMinutes())).slice(-2);
  let second = ("0" + (dateTime.getSeconds())).slice(-2);

  let month = dateTime.toDateString().slice(4, 7);
  let date = ("0" + dateTime.getDate()).slice(-2);
  let year = dateTime.getFullYear().toString().slice(2, 4);

  // Display arrays
  let hours = [...Array(13).keys()].map(n => ("0" + n).slice(-2))
  hours[0] = hours.pop()!
  let minutes = [...Array(60).keys()].map(n => ("0" + n).slice(-2));
  let seconds = [...Array(60).keys()].map(n => ("0" + n).slice(-2));
  let signs = ["AM", "PM"];
  let dates : any = getDaysOfMonth(month)!.map(n => ("0" + n).slice(-2));
  let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"]
  let years = [...Array(77).keys()].map(n => n+23);

  // let d = document.createEvent("TouchEvent");

  onMount(() => {
    var isScrolling : any;
    monthsContainer.addEventListener('scroll', function ( event ) {
      window.clearTimeout( isScrolling );
      isScrolling = setTimeout(function() {
        dates = getDaysOfMonth(getSelectedElement(monthsContainer).dataset.month!)!.map(n => ("0" + n).slice(-2));
      }, 100);
    }, false);

    // scrollToElement(document.getElementById("h" + targetHoursDisplay.toString()) as HTMLElement)
    // scrollToElement(document.getElementById("m" + targetMinutesDisplay.toString()) as HTMLElement)
    scrollToElement(hoursElements[hour])
    scrollToElement(minutesElements[minute])
    scrollToElement(secondsElements[second])
    scrollToElement(signsElements[sign])
    scrollToElement(datesElements[date])
    scrollToElement(monthsElements[month])
    scrollToElement(yearsElements[year])
  });

  function distance(x1 : number, y1 : number, x2 : number, y2 : number) {
    return Math.sqrt((x2-x1)*(x2-x1)+(y2-y1)*(y2-y1))
  }


  function updateTime() {
    hour = getSelectedElement(hoursContainer).dataset.hour!
    minute = getSelectedElement(minutesContainer).dataset.minute!
    second = getSelectedElement(secondsContainer).dataset.second!
    sign = getSelectedElement(signsContainer).dataset.sign!
    date = getSelectedElement(datesContainer).dataset.date!
    month = getSelectedElement(monthsContainer).dataset.month!
    year = getSelectedElement(yearsContainer).dataset.year!

    if (sign == "AM" && hour == "12") {hour = "00"}
    if (sign == "PM") {
      if (hour != "12") {
        hour = (Number(hour) + 12).toString()
      }
    }
    selectedDate = new Date(date + " " + month + " " + year + " " + hour + ":" + minute + ":" + second + ":" + "000").toISOString().replace("T", " ").replace("Z", "");
    dispatch("close");
  }

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
    // let selectedHours = getSelectedElement(hoursElement).dataset.hour
    // let selectedMinutes = getSelectedElement(minutesElement).dataset.minute
  }
  
</script>

<style>
  * {
    --entry-height: 45px;
    --entries-shown: 5;
    --font-size-value: 14pt;
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
    width: 80%;
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
    justify-content: center;
    align-items: center;
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
    scroll-snap-type: y mandatory;
    color: #fff;
    height: calc(var(--entry-height) * 5);
    overflow: scroll;
    text-align: center;
  }

  #time > div > p {
    font-size: var(--font-size-value);
    line-height: var(--entry-height);
    margin: 0;
    height: var(--entry-height);
    scroll-snap-align: center;
  }
</style>
 <!-- svelte-ignore a11y-click-events-have-key-events -->
<Overlay on:close={() => {; dispatch("close")}}>
  <div class="container" in:fade={{duration: 250}} out:fade={{duration: 250}}>
    <div class="window">
      <div id="time">
        <div id="hours" bind:this={hoursContainer}>
          <p></p>
          <p></p>
          {#each hours as hour}
            <p bind:this={hoursElements[hour]} on:click={(e) => {scrollToElement(e.target, true)}} id="h{hour}" data-hour="{hour}">{hour}</p>
          {/each}
          <p></p>
          <p></p>
        </div>
        <div style="width: 10px;">
          <p></p>
          <p></p>
          <p>:</p>
          <p></p>
          <p></p>
        </div>
        <div id="minutes" bind:this={minutesContainer}>
          <p></p>
          <p></p>
          {#each minutes as minute}
            <p bind:this={minutesElements[minute]} on:click={(e) => {scrollToElement(e.target, true)}} id="m{minute}" data-minute="{minute}">{minute}</p>
          {/each}
          <p></p>
          <p></p>
        </div>
        <div style="width: 10px;">
          <p></p>
          <p></p>
          <p>:</p>
          <p></p>
          <p></p>
        </div>
        <div id="seconds" bind:this={secondsContainer}>
          <p></p>
          <p></p>
          {#each seconds as second}
            <p bind:this={secondsElements[second]} on:click={(e) => {scrollToElement(e.target, true)}} id="s{second}" data-second="{second}">{second}</p>
          {/each}
          <p></p>
          <p></p>
        </div>
        <div style="width: 5px"/>
        <div id="sign" bind:this={signsContainer}>
          <p></p>
          <p></p>
          {#each signs as sign}
            <p bind:this={signsElements[sign]} on:click={(e) => {scrollToElement(e.target, true)}} id="si{sign}" data-sign="{sign}">{sign}</p>
          {/each}
          <p></p>
          <p></p>
        </div>
        <div style="width: 20px">
          <p></p>
          <p></p>
          <p>-</p>
          <p></p>
          <p></p>
        </div>
        <div id="date" bind:this={datesContainer}>
          <p></p>
          <p></p>
          {#each dates as date}
            <p bind:this={datesElements[date]} on:click={(e) => {scrollToElement(e.target, true)}} id="d{date}" data-date="{date}">{date}</p>
          {/each}
          <p></p>
          <p></p>
        </div>
        <div style="width: 5px"/>
        <div id="month" bind:this={monthsContainer}>
          <p></p>
          <p></p>
          {#each months as month}
            <p bind:this={monthsElements[month]} on:click={(e) => {scrollToElement(e.target, true)}} data-month="{month}">{month}</p>
          {/each}
          <p></p>
          <p></p>
        </div>
        <div style="width: 5px"/>
        <div id="year" bind:this={yearsContainer}>
          <p></p>
          <p></p>
          {#each years as year}
            <p bind:this={yearsElements[year]} on:click={(e) => {scrollToElement(e.target, true)}} id="y{year}" data-year="{year}">{year}</p>
          {/each}
          <p></p>
          <p></p>
        </div>
      </div>
      <div id="submit-container">      
        <button id="submit" on:click={updateTime}>Set</button>
      </div>
    </div>
  </div>
</Overlay>