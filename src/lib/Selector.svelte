<script lang="ts">
  import { onMount } from "svelte";

  import { Selection } from "../fractals-wasm";

  type Props = {
    width: number;
    plot: () => void;
  };

  let { width: canvasWidth, plot }: Props = $props();

  let selection:
    | undefined
    | {
        x: number;
        y: number;
        dimensions: undefined | { width: number; height: number };
      };
  let canvas: HTMLCanvasElement;
  let context: CanvasRenderingContext2D;
  
  onMount(() => {
    context = canvas.getContext("2d")!;
    context.strokeStyle = "yellow";
  });

  export function deselect() {
    selection = undefined;
    context.clearRect(0, 0, canvas.width, canvas.height);
  }

  export function getSelection(): undefined | Selection {
    if (selection == undefined || selection.dimensions == undefined) {
      return undefined;
    } else {
      const w = selection.dimensions.width;
      const h = selection.dimensions.height;

      let x, y;
      if (w < 0 && h < 0) {
        x = selection.x + selection.dimensions.width;
        y = selection.y + selection.dimensions.height;
      } else if (w > 0 && h < 0) {
        x = selection.x;
        y = selection.y + selection.dimensions.height;
      } else if (w < 0 && h > 0) {
        x = selection.x + selection.dimensions.width;
        y = selection.y;
      } else {
        //( w > 0 && h > 0 )
        x = selection.x;
        y = selection.y;
      }
      return new Selection(x, y, Math.abs(selection.dimensions.width));
    }
  }

  function canvasPointerPosition(e: PointerEvent) {
    const left = canvas.getBoundingClientRect().left + window.scrollX;
    const top = canvas.getBoundingClientRect().top + window.scrollY;
    return {
      x: e.pageX - left,
      y: e.pageY - top,
    };
  }

  function handlePointerDown(e: PointerEvent) {
    const pointerPosition = canvasPointerPosition(e);
    if (selection == undefined) {
      selection = {
        x: pointerPosition.x,
        y: pointerPosition.y,
        dimensions: undefined,
      };
      canvas.setPointerCapture(e.pointerId);
      canvas.addEventListener("pointercancel", handleCancel);
      canvas.addEventListener("pointermove", handleMove);
      canvas.addEventListener("pointerup", handleUp);
    } else {
      if (selection.dimensions != undefined) {
        let inX;
        if (selection.dimensions.width >= 0) {
          inX =
            pointerPosition.x >= selection.x &&
            pointerPosition.x <= selection.x + selection.dimensions.width;
        } else {
          inX =
            pointerPosition.x <= selection.x &&
            pointerPosition.x >= selection.x + selection.dimensions.width;
        }

        let inY;
        if (selection.dimensions.height >= 0) {
          inY =
            pointerPosition.y >= selection.y &&
            pointerPosition.y <= selection.y + selection.dimensions.height;
        } else {
          inY =
            pointerPosition.y <= selection.y &&
            pointerPosition.y >= selection.y + selection.dimensions.height;
        }

        if (inX && inY) {
          plot();
        } else {
          deselect(); 
        }
      }      
    }
  }

  function handleCancel(e: PointerEvent) {
    deselect();
    canvas.removeEventListener("pointercancel", handleCancel);
    canvas.removeEventListener("pointermove", handleMove);
    canvas.removeEventListener("pointerup", handleUp);
    canvas.releasePointerCapture(e.pointerId);
  }

  function handleMove(e: PointerEvent) {
    if (selection != undefined) {
      const pointerPos = canvasPointerPosition(e);

      let xDistance = pointerPos.x - selection.x;
      let yDistance = pointerPos.y - selection.y;
      let len = Math.max(Math.abs(xDistance), Math.abs(yDistance));

      if (len > 0) {
        let width = Math.sign(xDistance) * len;
        let height = Math.sign(yDistance) * len;

        let cornerX = selection.x + width;
        let cornerY = selection.y + height;

        if (
          cornerX > 0 &&
          cornerY > 0 &&
          cornerX < canvas.width &&
          cornerY < canvas.width
        ) {
          context.clearRect(0, 0, canvas.width, canvas.height);
          context.beginPath();
          context.strokeRect(selection.x, selection.y, width, height);
          selection.dimensions = { width, height };
        }
      }
    }
  }

  function handleUp(e: PointerEvent) {
    if (
      selection != undefined &&
      selection.dimensions != undefined &&
      Math.abs(selection.dimensions.width) > 0
    ) {
      context.clearRect(0, 0, canvas.width, canvas.height);
      context.beginPath();
      context.fillStyle = "rgba(20, 20, 20, 0.7)";
      context.fillRect(0, 0, canvas.width, canvas.height);
      
      context.clearRect(
        selection.x,
        selection.y,
        selection.dimensions.width,
        selection.dimensions.height
      );
      context.strokeRect(
        selection.x,
        selection.y,
        selection.dimensions.width,
        selection.dimensions.height
      );
    } else {
      selection = undefined;
    }
    canvas.removeEventListener("pointercancel", handleCancel);
    canvas.removeEventListener("pointermove", handleMove);
    canvas.removeEventListener("pointerup", handleUp);
    canvas.releasePointerCapture(e.pointerId);
  }
</script>

<canvas
  bind:this={canvas}
  class="w-full aspect-square left-0 top-0 absolute cursor-crosshair"
  width={canvasWidth}
  height={canvasWidth}
  onpointerdown={handlePointerDown}
></canvas>
