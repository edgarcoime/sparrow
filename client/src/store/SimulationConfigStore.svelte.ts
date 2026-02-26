export type SimulationConfig = {
  animalCount: number;
  foodCount: number;
};

const DEFAULT_CONFIG: SimulationConfig = {
  animalCount: 80,
  foodCount: 120,
};

function normalizeCount(value: number): number {
  if (!Number.isFinite(value)) return 0;
  const parsed = Math.floor(value);
  return parsed < 0 ? 0 : parsed;
}

function normalizeConfig(config: SimulationConfig): SimulationConfig {
  return {
    animalCount: normalizeCount(config.animalCount),
    foodCount: normalizeCount(config.foodCount),
  };
}

function applyConfig(target: SimulationConfig, source: SimulationConfig) {
  target.animalCount = source.animalCount;
  target.foodCount = source.foodCount;
}

const state = $state({
  draftConfig: { ...DEFAULT_CONFIG },
  appliedConfig: { ...DEFAULT_CONFIG },
  submitCount: 0,
});

export const simulationStore = {
  get draftConfig() {
    return state.draftConfig;
  },
  get appliedConfig() {
    return state.appliedConfig;
  },
  get submitCount() {
    return state.submitCount;
  },
  initialize(config: SimulationConfig) {
    const next = normalizeConfig(config);
    applyConfig(state.draftConfig, next);
    applyConfig(state.appliedConfig, next);
    state.submitCount += 1;
  },
  patchDraft(config: Partial<SimulationConfig>) {
    const next = normalizeConfig({
      animalCount: config.animalCount ?? state.draftConfig.animalCount,
      foodCount: config.foodCount ?? state.draftConfig.foodCount,
    });

    applyConfig(state.draftConfig, next);
  },
  submit() {
    applyConfig(state.appliedConfig, state.draftConfig);
    state.submitCount += 1;
  },
  reset() {
    applyConfig(state.draftConfig, DEFAULT_CONFIG);
    applyConfig(state.appliedConfig, DEFAULT_CONFIG);
    state.submitCount += 1;
  },
};
