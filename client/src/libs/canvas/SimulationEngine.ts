import type { SimulationConfig, SimulationHandle, EngineConfig } from "../types";
import * as PIXI from "pixi.js";

export default class SimulationEngine implements SimulationHandle {
    private app: PIXI.Application;
    private step = 0;
    private paused = false;
    private speed = 1
    private fps = 60;
    private readonly tickerFn = () => this.tick();

    // Private constructor so we can use async
    private constructor(app: PIXI.Application, config: EngineConfig) {
        this.app = app;
        this.fps = config?.fps ?? 60;
    }

    static async create(
        app: PIXI.Application,
        engineConfig: EngineConfig,
        _simConfig: SimulationConfig
    ): Promise<SimulationEngine> {
        // Create simulation based off config
        // Create world
        const engine = new SimulationEngine(app, engineConfig);
        engine.attachTicker()
        return engine;
    }

    // ---- PRIVATE ----
    private attachTicker() {
        this.applyTickerFps();
        this.app.ticker.add(this.tickerFn);
    }

    private applyTickerFps() {
        this.app.ticker.maxFPS = this.fps;
    }

    // Primary game loop
    private tick() {
        if (this.paused) return;

        for (let i = 0; i < this.speed; i++) {
            // Tick world/sim wasm
            this.step++;

            // Check if exceed generation
            if (this.step >= 2500) {
                this.runGeneration();
            }
        }

        // Sync sprites here
        this.syncSprites();
    }

    private runGeneration() {
        // this.world.run_genetic_algorithm();
        console.log("Create new generation")
        this.step = 0;
        this.onGenerationEnd();
    }

    private onGenerationEnd() {
        // e.g. log stats, trigger callbacks, snapshot best genome, etc.
    }

    private syncSprites() {
        // update pixi sprite positions from WASM Float32Array
        console.log("Syncing sprites with WASM data");
    }

    // ---- PUBLIC API ----
    pause() {
        this.paused = true;
    }

    resume() {
        this.paused = false;
    }

    setFps(fps: number) {
        if (!Number.isFinite(fps) || fps <= 0) return;
        this.fps = fps;
        this.applyTickerFps();
    }

    destroy() {
        this.app.ticker.remove(this.tickerFn);
        this.paused = true;
    }
}
