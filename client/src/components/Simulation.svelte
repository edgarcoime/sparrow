<script lang="ts">
  import { Simulation } from "lib-simulation-wasm";
  import {
    type SimulationAction,
    simulationStore,
    type SimulationConfig,
  } from "@/store/SimulationConfigStore.svelte";

  let totalAnimals = $state(0);
  let sample = $state("");
  let status = $state("Loading simulation...");
  let error = $state<string | null>(null);
  let stepCounter = $state(0);
  let placeholderStepMode = $state(false);
  let activeConfig = $state<SimulationConfig>({ animalCount: 0, foodCount: 0 });

  function snapshotWorld(config: SimulationConfig) {
    const { animalCount, foodCount } = config;

    const simulation = Simulation.withWorldSizes(animalCount, foodCount);
    const world = simulation.world();

    totalAnimals = world.animals.length;
    sample = world.animals
      .slice(0, 6)
      .map((animal) => `(${animal.x.toFixed(2)}, ${animal.y.toFixed(2)})`)
      .join(" · ");
    activeConfig = { animalCount, foodCount };
    placeholderStepMode = false;
    stepCounter = 0;
    status = `World initialized (animals=${animalCount}, foods=${foodCount}).`;
  }

  function handleSimulationAction(action: SimulationAction) {
    if (action === "step") {
      stepCounter += simulationStore.lastStepCount;
      placeholderStepMode = true;
      status = `Step placeholder advanced by ${simulationStore.lastStepCount}. World stepping is not implemented in WASM yet.`;
      return;
    }

    snapshotWorld(simulationStore.appliedConfig);
  }

  $effect(() => {
    simulationStore.runVersion;

    try {
      error = null;
      handleSimulationAction(simulationStore.lastAction);
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
      status = "Failed to load simulation.";
    }
  });
</script>

<div class="flex h-full flex-col font-mono text-slate-200">
  <div class="flex items-start justify-between gap-4 border-b border-slate-800 px-5 py-4">
    <div>
      <p class="text-[0.65rem] uppercase tracking-[0.32em] text-emerald-400">Simulation Window</p>
      <h2 class="mt-2 text-lg text-slate-100">Runtime Diagnostics</h2>
    </div>
    <div class="rounded border border-slate-700 bg-slate-950 px-3 py-1 text-[0.65rem] uppercase tracking-[0.22em] text-slate-400">
      {simulationStore.lastAction}
    </div>
  </div>

  <div class="grid min-h-0 flex-1 gap-4 p-5 xl:grid-cols-[minmax(0,1fr)_18rem]">
    <section class="flex min-h-0 flex-col rounded-xl border border-slate-800 bg-slate-950/80 p-4">
      <div class="mb-4 flex items-center justify-between gap-4">
        <p class="text-sm uppercase tracking-[0.24em] text-slate-500">Viewport</p>
        <div class="flex items-center gap-2 text-[0.65rem] uppercase tracking-[0.2em]">
          <span class="rounded border border-slate-700 px-2 py-1 text-slate-400">step {stepCounter}</span>
          {#if placeholderStepMode}
            <span class="rounded border border-amber-500/30 bg-amber-500/10 px-2 py-1 text-amber-300">
              placeholder
            </span>
          {/if}
        </div>
      </div>

      <div class="grid min-h-[50vh] flex-1 place-items-center rounded-lg border border-dashed border-slate-700 bg-[radial-gradient(circle_at_top,_rgba(56,189,248,0.08),_transparent_40%),linear-gradient(180deg,rgba(15,23,42,0.94),rgba(2,6,23,0.98))] xl:min-h-[24rem]">
        <div class="max-w-xl space-y-4 px-6 text-center">
          <div class="text-[0.65rem] uppercase tracking-[0.38em] text-slate-500">
            Future render surface
          </div>
          <div class="text-2xl text-slate-100">Canvas or component renderer can live here</div>
          <p class="text-sm leading-7 text-slate-400">
            For now this pane shows the active simulation state and action history while the
            rendering strategy is still undecided.
          </p>
        </div>
      </div>
    </section>

    <aside class="hidden xl:flex xl:h-full xl:flex-col xl:space-y-4">
      <section class="rounded-xl border border-slate-800 bg-slate-950/80 p-4">
        <p class="text-[0.65rem] uppercase tracking-[0.28em] text-slate-500">Status</p>
        <p class="mt-3 text-sm leading-7 text-slate-300">{status}</p>
        {#if error}
          <p class="mt-3 rounded-lg border border-rose-500/30 bg-rose-500/10 px-3 py-2 text-sm text-rose-300">
            {error}
          </p>
        {/if}
      </section>

      <section class="rounded-xl border border-slate-800 bg-slate-950/80 p-4 text-sm">
        <p class="text-[0.65rem] uppercase tracking-[0.28em] text-slate-500">Active Config</p>
        <dl class="mt-4 space-y-3 text-slate-300">
          <div class="flex items-center justify-between gap-4">
            <dt class="text-slate-500">animals</dt>
            <dd>{activeConfig.animalCount}</dd>
          </div>
          <div class="flex items-center justify-between gap-4">
            <dt class="text-slate-500">food</dt>
            <dd>{activeConfig.foodCount}</dd>
          </div>
          <div class="flex items-center justify-between gap-4">
            <dt class="text-slate-500">steps requested</dt>
            <dd>{stepCounter}</dd>
          </div>
        </dl>
      </section>

      <section class="rounded-xl border border-slate-800 bg-slate-950/80 p-4 text-sm">
        <p class="text-[0.65rem] uppercase tracking-[0.28em] text-slate-500">Sample Positions</p>
        <p class="mt-4 leading-7 text-slate-300">{sample || "No animal sample available."}</p>
        <div class="mt-4 border-t border-slate-800 pt-4 text-slate-400">
          <span class="text-slate-500">total animals</span>
          <span class="ml-3 text-emerald-300">{totalAnimals}</span>
        </div>
      </section>
    </aside>
  </div>
</div>
