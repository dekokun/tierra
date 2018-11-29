import {Universe} from "tierra";

const pre = document.getElementById("universe");
const universe = Universe.new();

const renderLoop = () => {
    pre.textContent = universe.render();
    universe.tick();

    setTimeout(() => requestAnimationFrame(renderLoop), 1000);
    ;
  };
requestAnimationFrame(renderLoop);