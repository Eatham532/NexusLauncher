<script setup lang="ts">
const emit = defineEmits(['click'])

const handleClick = () => {
  emit('click')
}
</script>

<template>
  <button @click="handleClick">
    <slot class="slot"></slot>
  </button>
</template>

<style lang="stylus" scoped>
button {
  display: flex;
  justify-content: center;
  justify-self: center;
  border: none;
  background: none;
  aspect-ratio: 1;
  border-radius: 20px;
  margin: 0 10px;
  align-items: center;
}

button:hover {
  cursor: pointer;
  background: rgba(117, 99, 99, 0.3);
  transition: all 0.5s ease;
  transform: scale(1.05);
}

</style>