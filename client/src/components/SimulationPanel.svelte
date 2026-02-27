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

<div class="grid h-[calc(100vh-32px)] grid-rows-[auto_minmax(0,1fr)] overflow-hidden rounded-[1.75rem] border border-slate-800 bg-slate-950 text-slate-100 shadow-[0_30px_80px_rgba(2,6,23,0.6)]">
  <div class="flex items-center justify-between gap-4 border-b border-slate-800 px-6 py-4 font-mono">
    <div>
      <p class="text-[0.65rem] uppercase tracking-[0.38em] text-slate-500">Edgar Coime</p>
      <h1 class="mt-2 text-xl text-slate-100">Simulator de espécies animais</h1>
    </div>

  </div>

  <div class="grid min-h-0 gap-0 xl:grid-cols-[22rem_minmax(0,1fr)]">
    <aside class="h-full min-h-0 border-b border-slate-800 bg-slate-900/80 p-4 xl:border-b-0 xl:border-r xl:border-slate-800">
      <SimulationInput />
    </aside>

    <section class="h-full min-h-0 bg-slate-950/80">
      <Simulation />
    </section>
  </div>
</div>
