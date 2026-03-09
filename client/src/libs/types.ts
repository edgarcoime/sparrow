export interface SimulationHandle {
    pause: () => void;
    resume: () => void;
    setFps: (fps: number) => void;
    destroy: () => void;
}

export interface EngineConfig {
    fps: number;
}

export interface SimulationConfig {
}
