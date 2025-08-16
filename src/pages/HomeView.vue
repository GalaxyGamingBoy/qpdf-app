<script setup lang="ts">
import { Button, Panel } from "primevue";
import { useRouter } from "vue-router";

const router = useRouter();

function goPdfMerge() {
  router.push("/pdf/merge")
}

function goPdfSplit() {
  router.push("/pdf/split")
}

function goPdfReorder() {
  router.push("/pdf/reorder")
}

</script>

<template>
  <Panel header="Home" class="w-[86vw]">
    <p class="text-2xl">Hey! Welcome to the QPDF Visual Toolkit</p>
    <p class="text-lg">What do you want to get started with?</p>

    <div class="flex justify-evenly mt-6 mb-2">
      <Button label="Merge PDFs" @click="goPdfMerge" severity="info">
        <template #icon>
          <span class="material-symbols-outlined">merge_type</span>
        </template>
      </Button>

      <Button label="Split PDF" @click="goPdfSplit" severity="info">
        <template #icon>
          <span class="material-symbols-outlined">call_split</span>
        </template>
      </Button>

      <Button label="Reorder PDF" @click="goPdfReorder" severity="info">
        <template #icon>
          <span class="material-symbols-outlined">move_down</span>
        </template>
      </Button>
    </div>
  </Panel>
</template>
