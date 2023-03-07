<script lang="ts">
	import { updated } from '$app/stores';
  import { extent } from 'd3-array';
  import { scaleLinear } from 'd3-scale';
  import { line, curveLinear } from 'd3-shape';
	import { onMount } from 'svelte';
	import { draw, fade } from 'svelte/transition';
  import { data } from '../routes/stores';
  import tweenSvgPath from "tween-svg-path";  
  import { interpolate } from "flubber"
	import { getHours, getMinutes, getSeconds, showHours, showMinutes, showSeconds } from '../scripts/helpers';
	import { quintOut } from 'svelte/easing';
  export let dateRange : any;
  onMount(() => {
    document.addEventListener('click', (e) => {
      if(!e.target) return
      if((e.target as HTMLElement).id != "container" && (e.target as HTMLElement).classList.contains("type") == false) {
        selectedPoint = null;
      }
    })
  })
  const graphHeight = 60
  const graphWidth = 100
  const edgeBuffer = 4;
  let box;
  let selectedPoint : any = null;
  let scaledX : any = null;
  let scaledY : any = null;
  let toolTip : HTMLElement;
  let toolTipLeft = 0;
  let toolTipTop = 0;
  let dateLineLeft = 0;
  const toolTipOffset = 10;
  let dateLine : HTMLElement;
  let selectedDate : string = "";
  let toolTipData : any = [];
  let container : HTMLElement;
  let cumulative : boolean = false;

  function calculateXExtent(data : any) : [number, number] {
    if(!data) return [0, 0]
    let max = null;
    let min = null;
    for(let project of data) {
      for(let point of project.data) {
        (max == null) ? max = point.date : max = point.date > max ? point.date : max;
        (min == null) ? min = point.date : min = point.date < min ? point.date : min;
      }
    }
    return [min, max]
  }
  function calculateYExtent(data : any) : [number, number] {
    if(!data) return [0, 0]
    let max = null;
    let min = null;
    for(let project of data) {
      for(let point of project.data) {
        let dailyData : any = cumulative ? point.cumulative : point.duration;
        (max == null) ? max = dailyData : max = dailyData > max ? dailyData : max;
        (min == null) ? min = dailyData : min = dailyData < min ? dailyData : min;
      }
    }
    return [min, max]
  }
  let paths : any = {}
  let points : any = {}
  $: updatePaths($data)

  function updatePaths(data : any[]) {
    if(data == null || data == undefined) {data = []}

    const xScale = scaleLinear()
    .domain(calculateXExtent(data)).range([edgeBuffer, graphWidth - edgeBuffer])
    const yScale = scaleLinear()
    .domain(calculateYExtent(data)).range([graphHeight - edgeBuffer, edgeBuffer])
    const pathLine = line()
    // @ts-ignore
    .x(d => xScale(d.date))
    // @ts-ignore
    .y(d => yScale(cumulative ? d.cumulative : d.duration))
    .curve(curveLinear)
    for(let project of data) {
        if(!project.visible) continue;
      // paths[project.id] = pathLine(project.data)
      const newPath = pathLine(project.data)!
      let newPoints = newPath.replaceAll("L", " ").slice(1).split(" ").map((n: string) => {let values = n.split(","); return [Number(values[0]), Number(values[1])]})
      points[project.id] = {locations: newPoints, data: project.data};
      paths[project.id] = newPath;
      }
  }
  // tweenSvgPath(testPath, testPath2, 500).onUpdate((svg) => {
  //   testPath = svg
  // })
    function isCumulative(val : boolean) {
      return val ? "selected" : "";
    }
    function createToolTip(e : TouchEvent) {
      if($data.length == 0) return;
      if((e.target as HTMLElement).id != "container") {return}
      let width = (e.target as HTMLElement).getBoundingClientRect().width;
      let height = (e.target as HTMLElement).getBoundingClientRect().height;
      let left = (e.target as HTMLElement).getBoundingClientRect().left;
      let top = (e.target as HTMLElement).getBoundingClientRect().y;
        const xScale = scaleLinear()
      .domain([0, width]).range([edgeBuffer, graphWidth - edgeBuffer])
      const yScale = scaleLinear()
      .domain([0, height]).range([graphHeight - edgeBuffer, edgeBuffer])
      let touchX = e.touches[0].clientX
      let touchY = e.touches[0].clientY
      scaledX = xScale(touchX);
      scaledY = (graphHeight - (edgeBuffer)) - yScale(touchY - top);
        if(touchX - left > width || touchX - left < left) {return}
      const numberOfPoints = $data[0].data.length;
      let newSelectedPoint = Math.floor((touchX - left) / width * numberOfPoints);
      if(selectedPoint == newSelectedPoint) return;
      selectedPoint = newSelectedPoint;
      let dateLineX = left + 4.5 + (selectedPoint / (numberOfPoints-1)) * (width * 0.92)
      let toolTipWidth = 100
      let toolTipX = dateLineX - toolTipWidth - toolTipOffset;
      if(toolTipX < 0) {
        toolTipX += toolTipWidth + (2 * toolTipOffset);
      }
      toolTipLeft = toolTipX;
      dateLineLeft = dateLineX;
      toolTipTop = 60;
      selectedDate = $data[0].data[selectedPoint].dateString;
      let tempToolTipData = []
      for(let project of $data) {
        tempToolTipData.push({cumulative: project.data[selectedPoint].cumulative, duration: project.data[selectedPoint].duration, color: project.color})
      }
      toolTipData = tempToolTipData;
    }

  function formatDate(date : Date) {
    let segments = date.toDateString().split(" ");
    let day = segments[2]
    let month = segments[1]
    let year = segments[3].slice(-2)
    return day + " " + month + " " + year
  }
</script>

<!-- <div bind:this={highestPointLabel} class="label">{formatDuration(maxDuration)}</div>
<div bind:this={lowestPointLabel} class="label">{formatDuration(minDuration)}</div> -->



<!-- svelte-ignore a11y-click-events-have-key-events -->
<div id="container" bind:this={container} on:touchstart={(e) => createToolTip(e)} on:touchmove={(e) => createToolTip(e)}>
  <div id="top">
    <button on:click={() => {cumulative = false;  updatePaths($data)}} id="raw" class="type {isCumulative(!cumulative)}"/>
    <button on:click={() => {cumulative = true;  updatePaths($data)}} id="cumulative" class="type {isCumulative(cumulative)}"/>
  </div>
  {#if selectedPoint != null}
    <div style="left: {dateLineLeft}px" id="dateline" bind:this={dateLine}/>
  {/if}
  <svg viewBox="0 0 {graphWidth} {graphHeight}" bind:this={box}>
    {#if $data != undefined}
      {#each $data as project (project.id)}
        {#if cumulative}
          <defs>
            <linearGradient id="gradient{project.id}" gradientTransform="rotate(90)">
              <stop offset="20%" stop-color="#{project.color}55" />
              <stop offset="100%" stop-color="#{project.color}00" />
            </linearGradient>
          </defs>
          <!-- <path stroke-linecap="round" fill="url(#gradient{project.id})" in:draw="{{duration: 800}}" style="--color: #{project.color}" d="{paths[project.id]}" /> -->
          {/if}
          <path stroke-linecap="round" stroke="none" fill="none" in:draw="{{duration: 800}}" style="--color: #{project.color}" d={paths[project.id]} out:draw|local="{{duration: 400, easing: quintOut}}"/>
          <path stroke-linecap="round" fill="url(#gradient{project.id})" in:fade="{{delay: 600, duration: 200}}" out:fade|local="{{duration: 150}}" d="{paths[project.id]},{graphWidth - edgeBuffer},{graphHeight - edgeBuffer}Z" />
          
          {#if selectedPoint != null && selectedPoint < project.data.length}
            <circle style="transition: 200ms all" cx={points[project.id].locations[selectedPoint][0]} cy={points[project.id].locations[selectedPoint][1]} r="2" fill="#{project.color}"/>
          {/if}
      {/each}
    {/if}
  </svg>
  <div id="bottom">
    <div id="range-start">
      {formatDate(new Date(dateRange.start))}
      <!-- {$data[0].data[0].dateString} -->
    </div>
    <div id="range-end">
      {formatDate(new Date(dateRange.end))}
      <!-- {$data[0].data[$data[0].data.length-1].dateString} -->
    </div>
  </div>
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  {#if selectedPoint != null}
  <div id="tooltip" bind:this={toolTip} style="left: {toolTipLeft}px; top: {toolTipTop}px">
    <div id="tooltip-content">
      <p id="selected-date">{selectedDate}</p>
      {#each toolTipData as data}
        <p class="tooltip-data" style="--color: #{data.color}">
          {#if showHours(cumulative ? data.cumulative : data.duration)}
            <span class="number">{getHours(cumulative ? data.cumulative : data.duration)}</span><span class="unit">h</span>
          {/if}
          {#if showMinutes(cumulative ? data.cumulative : data.duration)}
            <span class="number">{getMinutes(data.duration)}</span><span class="unit">m</span>
          {/if}
          {#if showSeconds(cumulative ? data.cumulative : data.duration) || (cumulative ? data.cumulative : data.duration) < 1000}
            <span class="number">{getSeconds(cumulative ? data.cumulative : data.duration)}</span><span class="unit">s</span>
          {/if}
        </p>
      {/each}</div>
  </div>
  {/if}
</div>

<style>
  #tooltip {
    transition: 200ms all, 100ms opacity;
    width: 100px;
    position: absolute;
    background-color: #00000088;
    pointer-events: none;
    display: flex;
    justify-content: center;
  }
  #tooltip-content {
    padding: 10px 0px;
  }
  #selected-date {
    color: #d3d3d3;
    margin: 0;
    font-family: "poppinsregular";
    font-size: 10pt;
    line-height: 16pt;
  }
  .tooltip-data {
    margin: 0;
    padding: 0;
    line-height: 16pt;
    color: var(--color);
    font-size: 12pt;
  }
  .number {
  font-family: "latoregular";
  line-height: 0px;
}
.unit {
  font-size: 8pt;
  line-height: 0px;
}
  #dateline {
    transition: 200ms all;
    position: absolute;
    pointer-events: none;
    height: 100%;
    width: 2px;
    border: 0;
    border-left: 3px dotted #9a9a9a;
    top: 0;
    left: 0;
  }
  svg {
    position: relative;
    pointer-events: none;
  }
  #container {
    background: #1e1e1e;
    border-radius: 10px;
    position: relative;
  }
  #top {
    pointer-events: none;
    display: flex;
  }
  #raw:after, #cumulative:after {
    transition: 150ms all;
    width: 20px;
    height: 20px;
    content: "";
    position: absolute;
    background-size: cover;
    top: 3px;
    left: 3px;
    filter: invert(0.83);
  }
  #raw::after {
    background-image: url("../assets/images/raw.svg");
  }
  #cumulative:after {
    background-image: url("../assets/images/cumulative.svg");
  }
  #raw.selected::after, #cumulative.selected::after {
    filter: invert(0.12);
  }
  
  .type {
    pointer-events: all;
    transition: 250ms all;
    position: relative;
    display: inline-block;
    width: 26px;
    height: 26px;
    margin-top: 6px;
    margin-left: 6px;
    border-radius: 50px;
    border: none;
    background: none;
  }
  .type.selected {
    background: #d3d3d3;
  }
  #bottom {
    pointer-events: none;
    display: flex;
    justify-content: space-between;
    color: #d3d3d3;
    padding: 15px 15px;
    padding-top: 0px;
    font-family: "poppinsregular";
    font-size: 10pt;
  }
  path {
    transition: 150ms all;
    stroke: var(--color);
    stroke-width: 0.8;
    /* fill: linear-gradient(90deg, rgba(2,0,36,1) 0%, rgba(9,9,121,1) 35%, rgba(0,212,255,1) 100%); */
    /* fill: var(--color) */
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