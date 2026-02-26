<script lang="ts">
  import { Simulation } from "lib-simulation-wasm";
  import {
    simulationStore,
    type SimulationConfig,
  } from "@/store/SimulationConfigStore.svelte";

  let totalAnimals = $state(0);
  let sample = $state("");
  let status = $state("Loading simulation...");
  let error = $state<string | null>(null);

  function runSimulation(config: SimulationConfig) {
    const { animalCount, foodCount } = config;

    const simulation = Simulation.withWorldSizes(animalCount, foodCount);
    const world = simulation.world();

    totalAnimals = world.animals.length;
    sample = world.animals
      .slice(0, 6)
      .map((animal) => `(${animal.x.toFixed(2)}, ${animal.y.toFixed(2)})`)
      .join(" · ");
    status = `Simulation loaded via wasm-bindgen (animals=${animalCount}, foods=${foodCount}).`;
  }

  $effect(() => {
    simulationStore.submitCount;

    try {
      error = null;
      runSimulation(simulationStore.appliedConfig);
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
      status = "Failed to load simulation.";
    }
  });
</script>

<h1>Sparrow WASM</h1>
<p>{status}</p>
{#if error}
  <p>Error: {error}</p>
{:else}
  <p>Total animals: <strong>{totalAnimals}</strong></p>
  <p>Sample positions: {sample || "none"}</p>
{/if}
