<script lang="ts">
  import SimulationCanvas from "@/components/simulation/SimulationCanvas.svelte";
  import type { SimulationHandle, EngineConfig, SimulationConfig } from "@/libs/types"

  let simConfig = $state<SimulationConfig>();
  let engineConfig = $state<EngineConfig>();
  // Normal let so svelte won't track
  let handle: SimulationHandle | null = null;
</script>

<h1>Simulation Root</h1>
<SimulationCanvas
  {engineConfig}
  {simConfig}
  onHandleReady={(h) => (handle = h)}
/>
