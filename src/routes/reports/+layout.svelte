<script lang="ts">
	import { rangeStart } from "../stores";

  let now = new Date(Date.now() - 86400000 * 6);
  now.setHours(0, 0, 0, 0);

  $rangeStart = new Date(now.toISOString()).getTime()

  let rangeContainer : HTMLElement;

  function selectRange(days : number | null, target: EventTarget | null) {
    if(!target) return;
    for (let rangeElement of rangeContainer.children) {
      rangeElement.classList.remove("active")
    }
    (target as HTMLElement).classList.add("active");
    
    if(days) {
      let now = new Date(Date.now() - 86400000 * days);
      now.setHours(0, 0, 0, 0);

      $rangeStart = new Date(now.toISOString()).getTime();
      return;
    };

    $rangeStart = 0;
  }

  console.log(rangeStart);

</script>

<style>

  #range {
    display: flex;
    justify-content: space-evenly;
    gap: 5px;
  }

  #range > button {
    flex: 1;
    border-radius: 50px;
    font-family: "poppinssemibold";
    background-color: #1e1e1e;
    border: none;
    color: #fff;
    padding: 15px 0;
    font-size: 14pt;
  }

  #range > button.active {
    color: #1e1e1e;
    background-color: #fff;
  }
</style>

<div id="range" bind:this={rangeContainer}>
  <button on:click={(e) => selectRange(6, e.target)} class="active">Week</button>
  <button on:click={(e) => selectRange(29, e.target)} >Month</button>
  <button on:click={(e) => selectRange(365, e.target)} >Year</button>
  <button on:click={(e) => selectRange(null, e.target)}>All</button>
</div>

<slot/>