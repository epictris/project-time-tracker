<script lang="ts">

  import { extent } from 'd3-array';
  import { scaleLinear } from 'd3-scale';
  import { line, curveLinear } from 'd3-shape';
	import { onMount } from 'svelte';
  // curveMonotoneX
  export let data : any;
  export let color : any;

  const graphHeight = 60
  const graphWidth = 100
  const edgeBuffer = 6;

  let highestPoint : SVGCircleElement;
  let lowestPoint : SVGCircleElement;

  let box;

  const xScale = scaleLinear()
    .domain(extent(data.map(d => d.date))).range([edgeBuffer, graphWidth - edgeBuffer])

  const yScale = scaleLinear()
    .domain(extent(data.map(d => d.duration))).range([graphHeight - edgeBuffer, edgeBuffer])

  const pathLine = line()
    // @ts-ignore
    .x(d => xScale(d.date))
    // @ts-ignore
    .y(d => yScale(d.duration))
    .curve(curveLinear);

  let points = pathLine(data)?.replaceAll("L", " ").slice(1).split(" ")
  let highest : any = null;
  let maxDuration : number;
  let lowest : any = null;
  let minDuration : number;
  for (let point = 0; point < points!.length; point++) {
    let tuple = points![point].split(",")
    let x = Number(tuple[0])
    let y = Number(tuple[1])
    if(y <= edgeBuffer) {
      highest = [x, y]
      maxDuration = data[point].duration
    } else if (y >= graphHeight - edgeBuffer) {
      lowest = [x, y]
      minDuration = data[point].duration
    }
  }

  let highestPointLabel : HTMLElement;
  let lowestPointLabel : HTMLElement;

  onMount(() => {
    if(!highestPointLabel || !lowestPointLabel || !highestPoint || !lowestPoint) return;
    highestPointLabel.style.top = (highestPoint.getBoundingClientRect().y - highestPointLabel.getBoundingClientRect().top - 30).toString() + "px"
    highestPointLabel.style.left = (highestPoint.getBoundingClientRect().x - highestPointLabel.getBoundingClientRect().width / 2 - 2).toString() + "px"
    lowestPointLabel.style.top = (lowestPoint.getBoundingClientRect().y - lowestPointLabel.getBoundingClientRect().top + 25).toString() + "px"
    lowestPointLabel.style.left = (lowestPoint.getBoundingClientRect().x - lowestPointLabel.getBoundingClientRect().width / 2 - 2).toString() + "px"

  })

  function formatDuration(duration : number) {
    if(duration == undefined) {return "";}
    let hours = Math.floor(duration / 3600000)
    let minutes = Math.floor((duration / 60000) % 60)
    let seconds = Math.floor((duration / 1000) % 60)

    if (hours >= 10) return hours + "h";
    if (hours > 0) return hours + "h" + (minutes > 0 ? " " + minutes + "m" : "")
    if (minutes > 0) return minutes + "m" + (seconds > 0 ? " " + seconds + "s" : "")
    return seconds + "s";
  }

</script>

<div bind:this={highestPointLabel} class="label">{formatDuration(maxDuration)}</div>
<div bind:this={lowestPointLabel} class="label">{formatDuration(minDuration)}</div>

<svg viewBox="0 0 {graphWidth} {graphHeight}" bind:this={box}>
  {#if data.length > 0}
    <path style="--color: #{color}" d={pathLine(data)} />
    {#if highest}
      <circle bind:this={highestPoint} class="dot" cx="{highest[0]}" cy="{highest[1]}" r="2"/>
    {/if}
    {#if lowest}
      <circle bind:this={lowestPoint} class="dot" cx="{lowest[0]}" cy="{lowest[1]}" r="2"/>
    {/if}
  {/if}
</svg>

<style>
  path {
    stroke: var(--color);
    stroke-width: 1.1;
    fill: none;
  }

  .dot {
    fill: #fff;
  }

  .label {
    position: absolute;
    color: #fff;
    top: 0;
    left: 0;
    width: 60px;
  }
</style>