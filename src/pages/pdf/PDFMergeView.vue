<script setup lang="ts">
import { open } from "@tauri-apps/plugin-dialog";
import { Button, OrderList, Panel } from "primevue";
import { Ref, ref } from "vue";

interface File {
  idx: number;
  file: string;
}

const files: Ref<File[]> = ref([
  {
    idx: 0,
    file: "~/Repos/libqpdf-rs/assets/testpdf1.pdf",
  },
  {
    idx: 1,
    file: "~/Repos/libqpdf-rs/assets/testpdf1.pdf",
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

const addPDF = async () => {
  await open({
    multiple: true,
    directory: false,
    filters: [{ name: "PDF Files", extensions: ["pdf"] }],
  });
};
</script>

<template>
  <Panel header="Merge PDFs" class="w-[80vw]">
    <OrderList v-model="files" data-key="idx">
      <template #option="{ option }">
        <span class="material-symbols-outlined mr-4"> picture_as_pdf </span>
        {{ option.file }}
      </template>
    </OrderList>

    <Button
      label="Add PDF"
      severity="secondary"
      variant="outlined"
      :fluid="true"
      class="my-2 mt-6"
      @click="addPDF"
    />

    <Button label="Merge All" :fluid="true" class="mt-2" />
  </Panel>
</template>
