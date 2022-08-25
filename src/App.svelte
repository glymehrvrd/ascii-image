<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { open } from "@tauri-apps/api/dialog";
  import { tick } from "svelte";

  let canvas_width = 800;
  let canvas_height = 800;
  let canvas: HTMLCanvasElement;
  let ascii_img = "";
  let select_pic = async () => {
    let selected = (await open({
      filters: [
        {
          name: "Image",
          extensions: ["png", "jpeg", "jpg"],
        },
      ],
    })) as String;
    if (selected) {
      ascii_img = await invoke("to_ascii", { imgPath: selected });
      let lines = ascii_img.split("\n");
      canvas_height = lines.length * 5;
      canvas_width = lines[0].length * 5;
      await tick();
      let ctx = canvas.getContext("2d");
      ctx.font = "8px Courier New";
      let y = 8;
      for (let i = 0; i < lines.length; i++) {
        const line = lines[i];
        console.log(line, 8, y);
        ctx.fillText(line, 8, y);
        y += 8;
      }
    }
  };
</script>

<main>
  <div>将图片转换为ASCII图</div>

  <div class="upload">
    <button on:click={select_pic}>选择图片</button>
  </div>

  <div id="image-viewer">
    <canvas bind:this={canvas} width={canvas_width} height={canvas_height} />
  </div>
</main>

<style>
</style>
