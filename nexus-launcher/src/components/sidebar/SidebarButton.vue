<script setup lang="ts">
const emit = defineEmits(['click'])

const handleClick = () => {
  emit('click')
}
</script>

<template>
  <div class="btn" @click="handleClick">
    <slot class="slot"></slot>
  </div>
</template>

<style lang="stylus" scoped>
.btn {
  display flex;
  justify-content center;
  align-items center;
  aspect-ratio 1;
  border-radius 20px
}

.btn:hover {
  cursor pointer;
  transition all 0.5s ease;
  transform scale(1.05);
  box-shadow: 2px 2px 5px 'rgb(from %s r g b / 0.5)' % var(--gray-900);
}

</style>