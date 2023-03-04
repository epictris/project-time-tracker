<script>
	import { onMount } from "svelte";
  import Overlay from '../components/Overlay.svelte'
  import Menu from "../components/Menu.svelte";
	import { slide } from "svelte/transition";
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
</style>

<link href="/src/assets/css/fonts.css" rel="stylesheet">

<div></div>

<slot/>
<Menu/>