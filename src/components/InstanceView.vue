<script setup lang="ts">
  const props = defineProps(['width, height']);

</script>

<template>
  <button class="InstanceViewBtn">
    Instance
  </button>
</template>

<style scoped>
  .InstanceViewBtn {
    border-radius: 10px;
    margin: 20px;
    aspect-ratio: 1;
    width: 200px;
  }
</style>