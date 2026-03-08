<!-- SimulationCanvas.svelte -->
<script lang="ts">
  import { onMount } from 'svelte';
  import * as PIXI from 'pixi.js';
  import type { SimulationConfig, EngineConfig, SimulationHandle } from '@/libs/types';
  import SimulationEngine from "@/libs/canvas/SimulationEngine";

  interface Props {
    engineConfig: EngineConfig;
    simConfig: SimulationConfig;
    onHandleReady: (handle: SimulationHandle) => void;
  }

  let { engineConfig, simConfig, onHandleReady }: Props = $props();
  let container: HTMLDivElement;
  let engine: SimulationHandle | null = null;

  onMount(() => {
    const app = new PIXI.Application();

    (async () => {
      console.log("Initializing")
      await app.init({ resizeTo: container, background: 0x0a0a0f });
      container.appendChild(app.canvas);

      engine = await SimulationEngine.create(app, engineConfig, simConfig);
      onHandleReady(engine);
    })();

    // onMount cleanup — mirrors useEffect's return fn
    return () => {
      engine?.destroy?.();
      app.destroy();
    }
  });
</script>

<div bind:this={container} class="canvas-container" />
