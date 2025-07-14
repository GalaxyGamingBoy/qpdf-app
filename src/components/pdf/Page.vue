<script setup lang="ts">
import { PageViewport, PDFDocumentProxy, PDFPageProxy } from "pdfjs-dist";
import { onMounted, useTemplateRef, watch } from "vue";

const canvas = useTemplateRef("canvas");

const props = defineProps(["page", "pdf"]);
const pageNum = props.page ?? 1;

function getScaledViewport(page: PDFPageProxy): PageViewport {
  const original = page.getViewport({ scale: 1 });

  const scaleX = canvas.value!.width / original.width;
  const scaleY = canvas.value!.height / original.height;
  const scale = Math.min(scaleX, scaleY);

  return page.getViewport({ scale });
}

async function render(doc: PDFDocumentProxy | null, page: number) {
  const ctx = canvas.value!.getContext("2d")!;

  if (doc == null) return;

  const pdfPage = await doc.getPage(page);

  const context = {
    canvasContext: ctx,
    viewport: getScaledViewport(pdfPage),
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
  <canvas :width="595 / 3" :height="842 / 3" ref="canvas"></canvas>
</template>

<style scoped>
canvas {
  background-color: blue;
  margin: 0.5rem 0.5rem 0.5rem 0.5rem;
}
</style>
