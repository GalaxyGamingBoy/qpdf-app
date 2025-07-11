<script setup lang="ts">
import { onMounted, useTemplateRef } from "vue";

const canvas = useTemplateRef("canvas");

const props = defineProps(["page"]);

onMounted(() => {
  const ctx = canvas.value?.getContext("2d");

  const page = props.page ?? 0;
  ctx?.fillText(`#${page}`, 6, 12);
});
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
