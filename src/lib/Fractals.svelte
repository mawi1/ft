<script lang="ts">
  import { draw, C } from "../fractals-wasm";
  import Selector from "./Selector.svelte";
  import RangeInput from "./RangeInput.svelte";

  const WIDTH = 700;

  enum Fractal {
    Julia,
    Mandelbrot,
  }

  let canvas: HTMLCanvasElement;
  let selector: Selector;

  let selectedFractal = $state(Fractal.Julia);
  let real = $state(-1);
  let imaginary = $state(0);
</script>

<div style={`width: ${WIDTH}px`} class="">
  <label for="fractal-select">Fractal</label>
  <select bind:value={selectedFractal} id="fractal-select">
    <option value={Fractal.Julia}>Julia</option>
    <option value={Fractal.Mandelbrot}>Mandelbrot</option>
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
  <button
    class="w-full border-2 border-black py-1 mt-2"
    onclick={() => {
      // if (selectedFractal == Fractal.Mandelbrot) {
      //   draw(canvas);
      // } else {
      //   draw(canvas, new C(real, imaginary));
      // }
     console.log(selector!.getSelection());
    }}
  >
    Draw</button
  >

  <div class="border-2 border-black mt-2 relative">
    <canvas
      bind:this={canvas}
      id="canvas"
      class="w-full aspect-square"
      width={WIDTH - 4}
      height={WIDTH - 4}
    >
    </canvas>
    <Selector bind:this={selector} width={WIDTH - 4} />
  </div>
</div>

<style>
</style>
