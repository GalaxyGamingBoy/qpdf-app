<script setup lang="ts">
import { Button, Panel, Toast, useToast } from "primevue";
import Page from "../../components/pdf/Page.vue";
import { onMounted, Ref, ref, shallowRef, ShallowRef } from "vue";
import { open, save } from "@tauri-apps/plugin-dialog";
import { decompressB64 } from "../../lib/decompress";
import { GlobalWorkerOptions, PDFDocumentProxy, getDocument } from "pdfjs-dist";
import { samplePDF } from "../../lib/seed";
import { invoke } from "@tauri-apps/api/core";

const toast = useToast();

let pages: Ref<number[]> = ref([]);
let loadedPDF: ShallowRef<PDFDocumentProxy | null> = shallowRef(null);
let selectedPDF: Ref<string | null> = ref(null);

onMounted(async () => {
  GlobalWorkerOptions.workerSrc = "/pdf.worker.mjs";
});

async function selectPDFDialog(): Promise<string | null> {
  if (import.meta.env.VITE_OUTSIDE_TAURI == "true")
    return "~/Download/PlaceholderFile.pdf";

  let diag = await open({
    multiple: false,
    directory: false,
    filters: [{ name: "PDF File", extensions: ["pdf"] }],
  });

  return diag;
}

async function selectOutPDFDialog(): Promise<string | null> {
  if (import.meta.env.VITE_OUTSIDE_TAURI == "true")
    return "~/Download/OutPlaceholderFile.pdf";

  let diag = await save({
    filters: [
      {
        name: "PDF File",
        extensions: ["pdf"],
      },
    ],
  });

  return diag;
}

async function getPDFData(file: string): Promise<string> {
  if (import.meta.env.VITE_OUTSIDE_TAURI == "true") return samplePDF;

  return await invoke("read_pdf_file", { file });
}

async function loadPDF(file: string) {
  if (loadedPDF.value != null) loadedPDF.value.destroy();
  loadedPDF.value = null;

  const data = await decompressB64(await getPDFData(file));
  const pdf = await getDocument({ data }).promise;

  pages.value = [...Array(pdf.numPages).keys()].map((v) => v + 1);

  loadedPDF.value = pdf;
}

async function onSelectPDF() {
  selectedPDF.value = await selectPDFDialog();
  await loadPDF(selectedPDF.value!);
}

async function onReorderPDF() {
  // @ts-ignore
  let out = selectOutPDFDialog();
  let success = true;

  if (!success)
    return toast.add({
      severity: "error",
      summary: "PDF Reorder",
      detail: "There was an error reordering the PDF",
    });

  toast.add({
    severity: "success",
    summary: "PDF Reorder",
    detail: "Reordered PDF File Saved!",
  });
}

function allow(e: Event) {
  e.preventDefault();
}

function drag(e: DragEventInit, from: number) {
  e.dataTransfer?.setData("from", from.toString());
}

function drop(e: DragEvent, to: number) {
  const from = parseInt(e.dataTransfer?.getData("from")!);

  const fromIdx = pages.value.indexOf(from);
  const toIdx = pages.value.indexOf(to);

  pages.value[fromIdx] = to;
  pages.value[toIdx] = from;
}
</script>

<template>
  <Toast />
  <Panel header="Reorder PDF" class="w-[86vw]">
    <div class="flex">
      <Button
        class="mx-2"
        :fluid="true"
        @click="onSelectPDF"
        :severity="selectedPDF ? 'secondary' : 'info'"
        >Select PDF</Button
      >

      <Button
        class="mx-2"
        :fluid="true"
        :disabled="selectedPDF == null"
        @click="onReorderPDF"
        >Reorder PDF</Button
      >
    </div>

    <div class="my-4" v-if="selectedPDF != null">
      <div class="flex flex-wrap justify-evenly">
        <div
          v-for="p in pages"
          :key="p"
          @dragover="allow"
          @dragstart="(e) => drag(e, p)"
          @drop="(e) => drop(e, p)"
        >
          <Page :page="p" :pdf="loadedPDF" draggable="true"></Page>
        </div>
      </div>
    </div>
  </Panel>
</template>

<style scoped></style>
