<script lang="ts">
  import { onMount } from "svelte";
  import { Simulation } from "lib-simulation-wasm";

  let totalAnimals = 0;
  let sample = "";
  let status = "Loading simulation...";
  let error: string | null = null;

  onMount(() => {
    try {
      const simulation = new Simulation();
      const world = simulation.world();

      totalAnimals = world.animals.length;
      sample = world.animals
        .slice(0, 6)
        .map((animal) => `(${animal.x.toFixed(2)}, ${animal.y.toFixed(2)})`)
        .join(" · ");
      status = "Simulation loaded via wasm-bindgen.";
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
