<script lang="ts">
  import { onMount } from "svelte";
  import Simulation from "@/components/Simulation.svelte";
  import SimulationInput from "@/components/SimulationInput.svelte";
  import { simulationStore } from "@/store/SimulationConfigStore.svelte";

  let { initialAnimalCount = 80, initialFoodCount = 120 } = $props();

  onMount(() => {
    simulationStore.initialize({
      animalCount: initialAnimalCount,
      foodCount: initialFoodCount,
    });
  });
</script>

<div class="space-y-4">
  <SimulationInput />
  <Simulation />
</div>
