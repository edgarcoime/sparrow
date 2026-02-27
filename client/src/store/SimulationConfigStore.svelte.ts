export type SimulationConfig = {
  animalCount: number;
  foodCount: number;
};

export type SimulationAction = "initialize" | "submit" | "step";

const DEFAULT_CONFIG: SimulationConfig = {
  animalCount: 80,
  foodCount: 120,
};

const DEFAULT_STEP_COUNT = 1;

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

function normalizeStepCount(value: number): number {
  if (!Number.isFinite(value)) return DEFAULT_STEP_COUNT;
  const parsed = Math.floor(value);
  return parsed < 1 ? DEFAULT_STEP_COUNT : parsed;
}

function applyConfig(target: SimulationConfig, source: SimulationConfig) {
  target.animalCount = source.animalCount;
  target.foodCount = source.foodCount;
}

const state = $state({
  draftConfig: { ...DEFAULT_CONFIG },
  appliedConfig: { ...DEFAULT_CONFIG },
  draftStepCount: DEFAULT_STEP_COUNT,
  lastStepCount: DEFAULT_STEP_COUNT,
  runVersion: 0,
  lastAction: "initialize" as SimulationAction,
});

export const simulationStore = {
  get draftConfig() {
    return state.draftConfig;
  },
  get appliedConfig() {
    return state.appliedConfig;
  },
  get draftStepCount() {
    return state.draftStepCount;
  },
  get lastStepCount() {
    return state.lastStepCount;
  },
  get runVersion() {
    return state.runVersion;
  },
  get lastAction() {
    return state.lastAction;
  },
  initialize(config: SimulationConfig) {
    const next = normalizeConfig(config);
    applyConfig(state.draftConfig, next);
    applyConfig(state.appliedConfig, next);
    state.lastAction = "initialize";
    state.runVersion += 1;
  },
  patchDraft(config: Partial<SimulationConfig>) {
    const next = normalizeConfig({
      animalCount: config.animalCount ?? state.draftConfig.animalCount,
      foodCount: config.foodCount ?? state.draftConfig.foodCount,
    });

    applyConfig(state.draftConfig, next);
  },
  setDraftStepCount(value: number) {
    state.draftStepCount = normalizeStepCount(value);
  },
  submit() {
    applyConfig(state.appliedConfig, state.draftConfig);
    state.lastAction = "submit";
    state.runVersion += 1;
  },
  step(count = state.draftStepCount) {
    state.lastStepCount = normalizeStepCount(count);
    state.lastAction = "step";
    state.runVersion += 1;
  },
  reset() {
    applyConfig(state.draftConfig, DEFAULT_CONFIG);
    applyConfig(state.appliedConfig, DEFAULT_CONFIG);
    state.draftStepCount = DEFAULT_STEP_COUNT;
    state.lastStepCount = DEFAULT_STEP_COUNT;
    state.lastAction = "submit";
    state.runVersion += 1;
  },
};
