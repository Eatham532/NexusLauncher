<script setup lang="ts">
import {Ref, ref} from "vue";

  const props = defineProps({
    items: {
      type: Object as () => string[],
      default: []
    },
    selected_items: {
      type: Object as () => string[],
      default: [],
    },
    multi_select: {
      type: Boolean,
      default: false,
    },
    disabled: {
      type: Boolean,
      default: false,
    }
  })

  const emit = defineEmits(['update:selected'])

  const selected: Ref<string[]> = ref(props.selected_items);

  function itemClick(item: string) {
    if (props.multi_select) {
      if (selected.value.includes(item)) {
        selected.value.splice(selected.value.indexOf(item), 1);
      }
      else {
        selected.value.push(item);
      }
    }
    else {
      selected.value = [item];
    }
    emit('update:selected', selected.value);
  }
</script>

<template>
  <div class="root">
    <div :class="{
      item: true,
      selected: selected.includes(item),
      disabled: disabled,
    }"

         @click="!disabled ? itemClick(item) : null" v-for="item in items">{{item}}</div>
  </div>
</template>

<style scoped lang="stylus">
  .root {
    display flex;
    flex-flow row;
    background-color var(--gray-800);
    border-radius 5px;
    user-select none;
    overflow clip
  }

  .item {
    display: flex;
    align-items center;
    padding 0.5em 1em;
    margin 0;

    &:is(:hover), &:is(.selected) {
      background-color var(--gray-700);
    }
  }
</style>