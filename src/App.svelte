<script lang="ts">
  import "./app.css";
  import init from "./fractals-wasm";
  import Fractals from "./lib/Fractals.svelte";

 let wasmInit = $state(init());
</script>

<main class="flex justify-center mt-6">
  {#await wasmInit}
    <p>...loading</p>
  {:then _}
    <Fractals/>
  {:catch err}
    <p style="color: red">Error could not load wasm.</p>
  {/await}
</main>
