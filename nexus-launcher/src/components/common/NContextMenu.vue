<script setup lang="ts">
  import {computed, ref, nextTick} from "vue";
  const context = ref();

  const top = ref(0);
  const left = ref(0);
  const show = ref(false);

  const style = computed(() => {
    return {
      top: top.value + 'px',
      left: left.value + 'px',
    };
  })

  const close = () => {
    show.value = false;
    left.value = 0;
    top.value = 0;
  }

  const open = (evt: any) => {
    console.log("open");
    // updates position of context menu
    left.value = evt.pageX || evt.clientX;
    top.value = evt.pageY || evt.clientY;
    // make element focused
    // @ts-ignore
    nextTick(() => context.value.focus());
    show.value = true;
  }

  defineExpose({
    open
  })

</script>

<template>
  <div class="context-menu" v-show="show" :style="style" ref="context" tabindex="0" @blur="close">
    <slot></slot>
  </div>
</template>

<style scoped>
.context-menu {
  border-radius: 0.5rem;
  overflow: hidden;
  position: fixed;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.12), 0 1px 2px rgba(0, 0, 0, 0.24);
}
</style>