<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog";
import { Button, OrderList, Panel, Toast, useToast } from "primevue";
import { computed, ComputedRef, Ref, ref } from "vue";

interface File {
  idx: number;
  file: string;
}

const toast = useToast();

const files: Ref<File[]> = ref([]);
const isFilesEmpty: ComputedRef<boolean> = computed(
  () => files.value.length == 0
);

const str2file = (s: string): File => {
  return {
    idx: files.value.length,
    file: s,
  };
};

const getPDFs = async (): Promise<string[]> => {
  if (import.meta.env.VITE_OUTSIDE_TAURI == true)
    return ["~/Download/PlaceholderFile.pdf"];

  let diag = await open({
    multiple: true,
    directory: false,
    filters: [{ name: "PDF Files", extensions: ["pdf"] }],
  });

  return diag ?? [];
};

const getOutPDF = async (): Promise<string | null> => {
  if (import.meta.env.VITE_OUTSIDE_TAURI == true)
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
};

const addPDF = async () => {
  let newFiles = await getPDFs();
  newFiles.forEach((file) => files.value.push(str2file(file)));
};

const removePDF = (file: File) => {
  files.value = files.value.filter((v) => v.idx != file.idx);
};

const merge = async () => {
  const out = await getOutPDF();
  if (out == null) return;

  const success: boolean = await invoke("merge_all_pdfs", {
    pdfs: files.value.map((v) => v.file),
    out,
  });

  if (!success)
    return toast.add({
      severity: "error",
      summary: "PDF Merge",
      detail: "There was an error merging the PDF Files",
    });

  toast.add({
    severity: "success",
    summary: "PDF Merge",
    detail: "Merged PDF File Saved!",
  });
};
</script>

<template>
  <Toast />

  <Panel header="Merge PDFs" class="w-[86vw]">
    <OrderList v-model="files" data-key="idx" breakpoint="908">
      <template #option="{ option }">
        <span class="material-symbols-outlined mr-4"> picture_as_pdf </span>
        {{ option.file }}

        <Button severity="danger" class="ml-auto" @click="removePDF(option)">
          <template #icon>
            <span class="material-symbols-outlined">delete</span>
          </template>
        </Button>
      </template>
    </OrderList>

    <Button
      label="Add PDF(s)"
      severity="secondary"
      variant="outlined"
      :fluid="true"
      class="my-2 mt-6"
      @click="addPDF()"
    />

    <Button
      label="Merge All"
      :fluid="true"
      class="mt-2"
      @click="merge"
      :disabled="isFilesEmpty"
    />
  </Panel>
</template>
