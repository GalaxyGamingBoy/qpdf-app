<script setup lang="ts">
import { Menubar } from "primevue";
import { ref, Ref } from "vue";

import type { MenuItem } from "primevue/menuitem";
import { useRouter } from "vue-router";

const router = useRouter();

const items: Ref<MenuItem[]> = ref([
  {
    label: "Home",
    icon: "home",
    command: () => router.push("/"),
  },
  {
    label: "Merge PDFs",
    icon: "merge_type",
    command: () => router.push("/pdf/merge"),
  },
  {
    label: "Split PDF",
    icon: "call_split",
    command: () => router.push("/pdf/split"),
  },
  {
    label: "Reorder PDF",
    icon: "move_down",
    command: () => router.push("/pdf/reorder"),
  },
]);
</script>

<template>
  <Menubar :model="items" breakpoint="0px">
    <template #itemicon="{ item }">
      <span class="material-symbols-outlined"> {{ item.icon }}</span>
    </template>
  </Menubar>
</template>
