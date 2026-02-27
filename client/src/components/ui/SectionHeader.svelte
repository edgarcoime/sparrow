<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    eyebrow: string;
    title: string;
    description?: string;
    children?: Snippet;
  }

  let { eyebrow, title, description, children }: Props = $props();
</script>

<div class="mb-4 flex items-start justify-between gap-3">
  <div>
    <p class="text-[0.65rem] uppercase tracking-[0.28em] text-slate-500">{eyebrow}</p>
    <p class="mt-1 text-sm text-slate-300">{title}</p>
    {#if description}
      <p class="mt-1 text-xs leading-6 text-slate-500">{description}</p>
    {/if}
  </div>

  {#if children}
    <div class="shrink-0">
      {@render children()}
    </div>
  {/if}
</div>
