<script lang="ts">
  import { onMount } from "svelte";

  import { FractalPlotter, C } from "../fractals-wasm";
  import RangeInput from "./RangeInput.svelte";
  import Selector from "./Selector.svelte";  
  
  const WIDTH = 700;

  enum Fractal {
    Julia,
    Mandelbrot,
  }

  let canvas: HTMLCanvasElement;
  let selector: Selector;

  let selectedFractal = $state(Fractal.Mandelbrot);
  let real = $state(-1);
  let imaginary = $state(0);

  let plotter: FractalPlotter;

  onMount(() => {
    plotter = new FractalPlotter(canvas);
    plot();
  });

  function plot() {
    let maybeC;
    if (selectedFractal == Fractal.Julia) {
      maybeC = new C(real, imaginary);
    }
    plotter.plot(maybeC, selector.getSelection());
    selector.deselect();
  }

  function goBack() {
    let maybeC;
    if (selectedFractal == Fractal.Julia) {
      maybeC = new C(real, imaginary);
    }
    plotter.go_back(maybeC);
  }
</script>

<div style={`width: ${WIDTH}px`} class="">
  <label for="fractal-select">Fractal:</label>
  <select bind:value={selectedFractal} id="fractal-select" class="p-1 border-2 border-black">
    <option value={Fractal.Mandelbrot}>Mandelbrot</option>
    <option value={Fractal.Julia}>Julia</option>
  </select>
  <div class:hidden={selectedFractal != Fractal.Julia}>
    <RangeInput
      id="real"
      label="Real"
      min="-1"
      max="1"
      step="0.002"
      bind:value={real}
    />
    <RangeInput
      id="imaginary"
      label="Imaginary"
      min="-1.5"
      max="1.5"
      step="0.004"
      bind:value={imaginary}
    />
  </div>
  <div class="flex gap-1.5 mt-2">
    <button class="w-full border-2 border-black py-1" onclick={plot}>
      Plot</button
    >
    <button class="w-full border-2 border-black py-1" onclick={goBack}>
      &larr; Back</button
    >
  </div>  
  <div class="border-2 border-black mt-2 relative">
    <canvas
      bind:this={canvas}
      id="canvas"
      class="w-full aspect-square"
      width={WIDTH - 4}
      height={WIDTH - 4}
    >
    </canvas>
    <Selector bind:this={selector} width={WIDTH - 4} {plot} />    
  </div>
</div>