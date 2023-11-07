<script setup lang="ts">
  const props = defineProps({
    selected: {
      type: Boolean,
      default: false,
    }
  });
</script>

<template>
  <div :class="{'selected': props.selected}" @click="$emit('click')">
    <slot></slot>
  </div>
</template>

<style lang="stylus" scoped>
div {
  padding: 20px 10px 20px 10px;
  width: 100%;

  &:hover {
    font-size: 20px;
    border-top: 2px solid var(--gray-100);
    border-bottom: 2px solid var(--gray-100);
  }
}

.selected {
  font-size: 20px;
  border-top: 2px solid var(--gray-100);
  border-bottom: 2px solid var(--gray-100);
  background-color: var(--primary-700);
}
</style>