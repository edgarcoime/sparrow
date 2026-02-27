<script lang="ts">
  import SectionHeader from "@/components/ui/SectionHeader.svelte";
  import { simulationStore } from "@/store/SimulationConfigStore.svelte";

  function updateAnimalCount(event: Event) {
    const target = event.currentTarget as HTMLInputElement;
    simulationStore.patchDraft({ animalCount: Number(target.value) });
  }

  function updateFoodCount(event: Event) {
    const target = event.currentTarget as HTMLInputElement;
    simulationStore.patchDraft({ foodCount: Number(target.value) });
  }

  function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    simulationStore.submit();
  }

  function updateStepCount(event: Event) {
    const target = event.currentTarget as HTMLInputElement;
    simulationStore.setDraftStepCount(Number(target.value));
  }

  function stepOnce() {
    simulationStore.step(1);
  }

  function stepMany() {
    simulationStore.step(simulationStore.draftStepCount);
  }
</script>

<div class="flex h-full min-h-0 flex-col font-mono text-slate-200">
  <div class="mb-5 flex items-start justify-between gap-3 border-b border-slate-800 pb-4">
    <div>
      <p class="text-[0.65rem] uppercase tracking-[0.32em] text-cyan-400">Config Rail</p>
      <h2 class="mt-2 text-lg text-slate-100">Simulation Controls</h2>
    </div>
    <div class="rounded border border-slate-700 bg-slate-950 px-2 py-1 text-[0.65rem] uppercase tracking-[0.24em] text-slate-400">
      Draft
    </div>
  </div>

  <form class="flex min-h-0 flex-1 flex-col overflow-hidden" onsubmit={handleSubmit}>
  <!-- submit button -->
    <div class="shrink-0 border-b border-slate-800 pb-4">
      <button
        class="flex w-full items-center justify-center rounded-lg border border-cyan-500/40 bg-cyan-500/10 px-4 py-2.5 text-sm uppercase tracking-[0.24em] text-cyan-300 transition hover:border-cyan-400 hover:bg-cyan-400/10"
        type="submit"
      >
        Apply Config
      </button>
    </div>

    <div class="hide-scrollbar min-h-0 flex-1 space-y-5 overflow-y-auto py-5 pr-1">
    <!-- world generation parameters -->
      <section class="rounded-xl border border-slate-800 bg-slate-950/70 p-4">
        <SectionHeader
          eyebrow="Constructor"
          title="World generation parameters"
          description="These values define the next simulation instance when you apply the draft."
        />

        <div class="space-y-4">
          <label class="grid gap-1.5">
            <span class="text-[0.7rem] uppercase tracking-[0.22em] text-slate-500">Animals</span>
            <input
              class="rounded-lg border border-slate-700 bg-slate-900 px-3 py-2 text-sm text-slate-100 outline-none transition focus:border-cyan-400"
              min="0"
              name="animalCount"
              type="number"
              value={simulationStore.draftConfig.animalCount}
              oninput={updateAnimalCount}
            />
          </label>

          <label class="grid gap-1.5">
            <span class="text-[0.7rem] uppercase tracking-[0.22em] text-slate-500">Food</span>
            <input
              class="rounded-lg border border-slate-700 bg-slate-900 px-3 py-2 text-sm text-slate-100 outline-none transition focus:border-cyan-400"
              min="0"
              name="foodCount"
              type="number"
              value={simulationStore.draftConfig.foodCount}
              oninput={updateFoodCount}
            />
          </label>
        </div>
      </section>

      <!-- placeholder section -->
      <section class="rounded-xl border border-slate-800 bg-slate-950/70 p-4">
        <SectionHeader
          eyebrow="Reserved"
          title="Future constructor parameters"
          description="This block stands in for the longer list of controls that will eventually live in the scrollable region."
        />

        <div class="space-y-3 text-sm text-slate-400">
          <div class="rounded-lg border border-dashed border-slate-700 bg-slate-900/60 px-3 py-3">
            sensor radius
          </div>
          <div class="rounded-lg border border-dashed border-slate-700 bg-slate-900/60 px-3 py-3">
            mutation rate
          </div>
          <div class="rounded-lg border border-dashed border-slate-700 bg-slate-900/60 px-3 py-3">
            reproduction threshold
          </div>
          <div class="rounded-lg border border-dashed border-slate-700 bg-slate-900/60 px-3 py-3">
            environment seed
          </div>
          <div class="rounded-lg border border-dashed border-slate-700 bg-slate-900/60 px-3 py-3">
            environment seed
          </div>
          <div class="rounded-lg border border-dashed border-slate-700 bg-slate-900/60 px-3 py-3">
            environment seed
          </div>
          <div class="rounded-lg border border-dashed border-slate-700 bg-slate-900/60 px-3 py-3">
            environment seed
          </div>
          <div class="rounded-lg border border-dashed border-slate-700 bg-slate-900/60 px-3 py-3">
            environment seed
          </div>
          <div class="rounded-lg border border-dashed border-slate-700 bg-slate-900/60 px-3 py-3">
            environment seed
          </div>
        </div>
      </section>


      <!-- manual advancement controls -->
      <section class="rounded-xl border border-slate-800 bg-slate-950/70 p-4">
        <SectionHeader
          eyebrow="Execution"
          title="Manual advancement controls"
          description="Step actions are wired through the shared store and currently operate as UI placeholders."
        />

        <div class="space-y-4">
          <label class="grid gap-1.5">
            <span class="text-[0.7rem] uppercase tracking-[0.22em] text-slate-500">Step Count</span>
            <input
              class="rounded-lg border border-slate-700 bg-slate-900 px-3 py-2 text-sm text-slate-100 outline-none transition focus:border-amber-400"
              min="1"
              name="stepCount"
              type="number"
              value={simulationStore.draftStepCount}
              oninput={updateStepCount}
            />
          </label>

          <div class="grid grid-cols-1 gap-3 sm:grid-cols-2">
            <button
              class="rounded-lg border border-slate-700 bg-slate-900 px-4 py-2.5 text-sm uppercase tracking-[0.2em] text-slate-200 transition hover:border-amber-400 hover:text-amber-300"
              type="button"
              onclick={stepOnce}
            >
              Step +1
            </button>
            <button
              class="rounded-lg border border-amber-500/40 bg-amber-500/10 px-4 py-2.5 text-sm uppercase tracking-[0.2em] text-amber-300 transition hover:border-amber-400 hover:bg-amber-400/10"
              type="button"
              onclick={stepMany}
            >
              Step +{simulationStore.draftStepCount}
            </button>
          </div>
        </div>
      </section>
    </div>

    <section class="mt-4 shrink-0 rounded-xl border border-slate-800 bg-slate-950/95 px-4 py-3 text-[0.7rem] uppercase tracking-[0.18em] text-slate-500">
      <div class="grid grid-cols-3 gap-3">
        <div>
          <div>animals</div>
          <div class="mt-1 text-sm tracking-normal text-slate-200">
            {simulationStore.appliedConfig.animalCount}
          </div>
        </div>
        <div>
          <div>food</div>
          <div class="mt-1 text-sm tracking-normal text-slate-200">
            {simulationStore.appliedConfig.foodCount}
          </div>
        </div>
        <div>
          <div>action</div>
          <div class="mt-1 text-sm tracking-normal text-cyan-300">{simulationStore.lastAction}</div>
        </div>
      </div>
    </section>
  </form>
</div>
