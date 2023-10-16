<script setup lang="ts">
const emit = defineEmits(['click'])

const handleClick = () => {
  emit('click')
}
</script>

<template>
  <button @click="handleClick">
    <slot></slot>
  </button>
</template>

<style scoped>
button {
  border: none;
  background: none;
  padding: 5px;
  aspect-ratio: 1;
  border-radius: 20px;
  margin: 0 10px;
}

button:hover {
  cursor: pointer;
  background: rgba(117, 99, 99, 0.3);
  transition: all 0.5s ease;
}

</style>