<script setup lang="ts">
import { Menubar } from "primevue";
import { ref, Ref } from "vue";
import type { MenuItem } from "primevue/menuitem";

const items: Ref<MenuItem[]> = ref([
  {
    label: "Home",
    icon: "home",
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
