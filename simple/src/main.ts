import "./style.css";
import { Simulation } from "lib-simulation-wasm";

const app = document.querySelector<HTMLDivElement>("#app");
if (!app) {
  throw new Error("Missing #app container");
}

const simulation = new Simulation();
const world = simulation.world();

const animalsPreview = world.animals
  .slice(0, 6)
  .map((animal) => `(${animal.x.toFixed(2)}, ${animal.y.toFixed(2)})`)
  .join(" · ");

app.innerHTML = `
  <h1>Sparrow WASM</h1>
  <p>Simulation loaded via wasm-bindgen.</p>
  <p>Total animals: <strong>${world.animals.length}</strong></p>
  <p>Sample positions: ${animalsPreview || "none"}</p>
`;
