<script lang="ts">
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
</script>

<form class="mb-4 flex items-end gap-4" onsubmit={handleSubmit}>
  <label class="grid gap-1.5">
    <span class="text-sm font-medium">Animals</span>
    <input
      class="min-w-32 rounded border border-slate-300 px-3 py-2"
      min="0"
      name="animalCount"
      type="number"
      value={simulationStore.draftConfig.animalCount}
      oninput={updateAnimalCount}
    />
  </label>

  <label class="grid gap-1.5">
    <span class="text-sm font-medium">Food</span>
    <input
      class="min-w-32 rounded border border-slate-300 px-3 py-2"
      min="0"
      name="foodCount"
      type="number"
      value={simulationStore.draftConfig.foodCount}
      oninput={updateFoodCount}
    />
  </label>

  <button
    class="rounded bg-slate-900 px-4 py-2 text-white transition hover:bg-slate-700"
    type="submit"
  >
    Run simulation
  </button>
</form>
