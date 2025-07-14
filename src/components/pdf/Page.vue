<script setup lang="ts">
import { PageViewport, PDFDocumentProxy, PDFPageProxy } from "pdfjs-dist";
import { Button, Drawer, SpeedDial } from "primevue";
import { onMounted, ref, useTemplateRef, watch } from "vue";

const canvas = useTemplateRef("canvas");
const enlargedCanvas = useTemplateRef("enlarged-canvas");

const props = defineProps(["page", "pdf"]);
const pageNum = props.page ?? 1;

const enlargedVisible = ref(false);
const enlargedPanelHeader = `PDF Page #${pageNum} Preview`;

function getScaledViewport(
  page: PDFPageProxy,
  c: HTMLCanvasElement
): PageViewport {
  const original = page.getViewport({ scale: 1 });

  const scaleX = c.width / original.width;
  const scaleY = c.height / original.height;
  const scale = Math.min(scaleX, scaleY);

  return page.getViewport({ scale });
}

function onEnlargedPressed() {
  enlargedVisible.value = true;
}

async function renderEnlarged() {
  const ctx = enlargedCanvas.value!.getContext("2d")!;
  ctx.scale(1, 1);

  if (props.pdf == null) return;
  const pdfPage = await props.pdf.getPage(pageNum);

  const context = {
    canvasContext: ctx,
    viewport: getScaledViewport(pdfPage, enlargedCanvas.value!),
  };

  await pdfPage.render(context).promise;
}

async function render(doc: PDFDocumentProxy | null, page: number) {
  const ctx = canvas.value!.getContext("2d")!;

  if (doc == null) return;

  const pdfPage = await doc.getPage(page);

  const context = {
    canvasContext: ctx,
    viewport: getScaledViewport(pdfPage, canvas.value!),
  };

  await pdfPage.render(context).promise;
}

watch(
  () => props.pdf,
  async (pdf, _) => {
    await render(pdf, pageNum);
  }
);

onMounted(async () => await render(props.pdf, pageNum));
</script>

<template>
  <div class="relative left-40 top-14">
    <!-- <SpeedDial type="linear" direction="down" :radius="60" :model="dial"> -->
    <!-- <template #itemicon="{ item }"> -->
    <!-- <span class="material-symbols-outlined">{{ item.icon }}</span> -->
    <!-- </template> -->
    <!-- </SpeedDial> -->

    <Button rounded @click="onEnlargedPressed">
      <template #icon>
        <span class="material-symbols-outlined">arrows_output</span>
      </template>
    </Button>
  </div>

  <canvas
    :width="595 / 3"
    :height="842 / 3"
    ref="canvas"
    class="border-1"
    draggable="true"
  ></canvas>

  <Drawer
    v-model:visible="enlargedVisible"
    :header="enlargedPanelHeader"
    @after-show="renderEnlarged"
    class="!w-92"
  >
    <div class="flex justify-center">
      <canvas :width="595 / 1.75" :height="842 / 1.75" ref="enlarged-canvas" />
    </div>
  </Drawer>
</template>

<style scoped>
canvas {
  background-color: #fcfcff;
  margin: 0.5rem 0.5rem 0.5rem 0.5rem;
}
</style>
