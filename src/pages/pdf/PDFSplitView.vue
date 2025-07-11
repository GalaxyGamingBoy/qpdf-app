<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import {
  Button,
  Checkbox,
  FloatLabel,
  InputGroup,
  InputGroupAddon,
  InputText,
  Panel,
  Toast,
  useToast,
} from "primevue";
import { Ref, ref, watch } from "vue";

interface PDFDetails {
  title: string;
  author: string;
}

const toast = useToast();

const selectedFile: Ref<string | null> = ref(null);
const customOutputFormat: Ref<string> = ref("$title - ($i of $n).pdf");
const hasCustomOutputFormat: Ref<boolean> = ref(false);
const selectedFileDetails: Ref<PDFDetails | null> = ref(null);

async function getPDFDetails(file: string | null): Promise<PDFDetails | null> {
  if (file == null) return null;

  if (import.meta.env.VITE_OUTSIDE_TAURI == true)
    return { title: "PDF Title Placeholder", author: "Lorem Ipsum" };

  return await invoke("get_pdf_details", { pdf: file });
}

async function selectPDFDialog(): Promise<string | null> {
  if (import.meta.env.VITE_OUTSIDE_TAURI == true)
    return "~/Download/PlaceholderFile.pdf";

  let diag = await open({
    multiple: false,
    directory: false,
    filters: [{ name: "PDF File", extensions: ["pdf"] }],
  });

  return diag;
}

async function selectOutputFolderDialog(): Promise<string | null> {
  if (import.meta.env.VITE_OUTSIDE_TAURI == true) return "~/Download/";

  let diag = await open({ multiple: false, directory: true });
  return diag;
}

async function onSelectPDF() {
  selectedFile.value = await selectPDFDialog();
}

async function onSplitPDF() {
  const out = await selectOutputFolderDialog();
  if (out == null) return;

  const success: boolean = await invoke("split_pdf_to_pages", {
    pdf: selectedFile.value,
    format: customOutputFormat.value,
    out,
  });

  if (!success)
    return toast.add({
      severity: "error",
      summary: "PDF Split",
      detail: "There was an error splitting the PDF",
    });

  toast.add({
    severity: "success",
    summary: "PDF Merge",
    detail: "Splitted PDF Files Saved!",
  });
}

watch(selectedFile, async (file, _) => {
  selectedFileDetails.value = await getPDFDetails(file);
});
</script>

<template>
  <Toast />
  <Panel header="Split PDF Pages" class="w-[86vw]">
    <Panel toggleable>
      <template #header>
        <div class="flex items-center gap-2">
          <span class="material-symbols-outlined">quick_reference_all</span>
          <span class="font-bold text-xl">PDF Details</span>
        </div>
      </template>

      <InputGroup class="mb-1 h-12">
        <InputGroupAddon>
          <span class="material-symbols-outlined mx-2">article</span>
        </InputGroupAddon>
        <InputText disabled :value="selectedFile" />
      </InputGroup>

      <InputGroup class="my-1 h-12">
        <InputGroupAddon>
          <span class="material-symbols-outlined mx-2">title</span>
        </InputGroupAddon>
        <InputText disabled :value="selectedFileDetails?.title" />
      </InputGroup>

      <InputGroup class="mt-1 h-12">
        <InputGroupAddon>
          <span class="material-symbols-outlined mx-2">person</span>
        </InputGroupAddon>
        <InputText disabled :value="selectedFileDetails?.author" />
      </InputGroup>
    </Panel>

    <InputGroup class="mt-2">
      <InputGroupAddon>
        <Checkbox
          :binary="true"
          v-tooltip="'Custom Output Format'"
          v-model="hasCustomOutputFormat"
        />
      </InputGroupAddon>

      <FloatLabel variant="in">
        <InputText
          placeholder="$title - $i/$n.pdf"
          id="out_format"
          v-model="customOutputFormat"
          :disabled="!hasCustomOutputFormat"
        />
        <label for="out_format">Output Format</label>
      </FloatLabel>

      <InputGroupAddon>
        <span
          class="material-symbols-outlined"
          v-tooltip="
            'The file format that the output files will follow.\n\n$title: PDF Title\n$author: PDF Author\n$i: Current Page\n$n: Total Pages'
          "
          >help</span
        >
      </InputGroupAddon>
    </InputGroup>

    <Button
      label="Select PDF"
      :fluid="true"
      class="mt-2"
      severity="secondary"
      @click="onSelectPDF"
    />

    <Button label="Split PDF" :fluid="true" class="mt-2" @click="onSplitPDF" />
  </Panel>
</template>
