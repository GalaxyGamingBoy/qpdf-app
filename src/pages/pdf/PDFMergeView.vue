<script setup lang="ts">
import { open } from "@tauri-apps/plugin-dialog";
import { Button, OrderList, Panel, Toast, useToast } from "primevue";
import { Ref, ref } from "vue";

interface File {
  idx: number;
  file: string;
}

const toast = useToast();

const files: Ref<File[]> = ref([
  {
    idx: 0,
    file: "~/Repos/libqpdf-rs/assets/testpdf1.pdf",
  },
  {
    idx: 1,
    file: "~/Repos/libqpdf-rs/assets/testpdf2.pdf",
  },
  {
    idx: 2,
    file: "~/Repos/libqpdf-rs/assets/testpdf3.pdf",
  },
  {
    idx: 3,
    file: "~/Repos/libqpdf-rs/assets/testpdf4.pdf",
  },
]);

const str2file = (s: string): File => {
  return {
    idx: files.value.length,
    file: s,
  };
};

const getPDFs = async (): Promise<string[]> => {
  if (import.meta.env.VITE_OUTSIDE_TAURI) return ["~/test.pdf"];

  let diag = await open({
    multiple: true,
    directory: false,
    filters: [{ name: "PDF Files", extensions: ["pdf"] }],
  });

  return diag!;
};

const addPDF = async () => {
  let newFiles = await getPDFs();
  let processedFiles: File[] = newFiles.map(str2file);

  files.value = files.value.concat(processedFiles);
};

const removePDF = (file: File) => {
  files.value = files.value.filter((v) => v.idx != file.idx);
};

const merge = async () => {
  console.log(files.value);

  toast.add({
    severity: "success",
    summary: "PDF Merge",
    detail: "Merged PDF File Saved!",
  });
};
</script>

<template>
  <Toast />

  <Panel header="Merge PDFs" class="w-[80vw]">
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

    <Button label="Merge All" :fluid="true" class="mt-2" @click="merge" />
  </Panel>
</template>
