<script setup lang="ts">
import Navbar from "./components/Navbar.vue";
import { RouterView } from "vue-router";
</script>

<template>
  <nav class="m-4"><Navbar /></nav>

  <main class="flex flex-col items-center h-[80vh]">
    <RouterView></RouterView>
  </main>
</template>
