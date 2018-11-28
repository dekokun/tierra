import {Universe} from "tierra";

const pre = document.getElementById("universe");
const universe = Universe.new();

const renderLoop = () => {
    pre.textContent = universe.render();

    requestAnimationFrame(renderLoop);
  };
requestAnimationFrame(renderLoop);