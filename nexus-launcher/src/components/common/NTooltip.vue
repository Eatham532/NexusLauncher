<template>
  <div class="tooltip-wrapper">
    <slot />
    <!-- <div :class="classes"> -->
    <span :class="tooltipClasses" role="tooltip" :id="tooltipId" inert>{{ text }}</span>

    <!-- </div> -->
  </div>
</template>
<script setup>
import { computed } from 'vue';

const props = defineProps({
  text: {
    type: String,
    required: true
  },
  position: {
    type: String,
    default: 'bottom'
  },
  tooltipId: {
    type: String,
    default: 'tooltip__text'
  }
})

const tooltipClasses = computed(() => ({
  'tooltip__text': true,
  [`tooltip--${props.position}`]: true
}))
</script>
<style scoped>
.tooltip-wrapper {
  position: relative;
  display: inline-block;
}

.tooltip__text {
  margin: 10px;
  visibility: hidden;
  opacity: 0;
  transition: opacity 0.5s;

  color: #ffffff;
  text-align: center;
  padding: 5px;
  border-radius: 2px;
  min-width: 120px;
  background: #5e5d5d;
  box-shadow: 1px 1px 5px var(--gray-500);

  position: absolute;
  z-index: 1;
}

.tooltip-wrapper:hover .tooltip__text{
  visibility: visible;
  opacity: 1;
}

:slotted(*):focus + .tooltip__text {
  visibility: visible;
  opacity: 1;
}

.tooltip--top {
  inset-block-end: 120%;
  inset-inline-start: 50%;
  margin-inline-start: -70px;
}

.tooltip--bottom {
  inset-block-start: 120%;
  inset-inline-start: 50%;
  margin-inline-start: -70px;
}

.tooltip--left {
  inset-block-end: 0%;
  inset-inline-end: 110%;
}

.tooltip--right {
  inset-block-end: 0%;
  inset-inline-start: 110%;
}

.tooltip__text::after {
  content: " ";
  position: absolute;
  border-width: 5px;
  border-style: solid;
  border-color: #5e5d5d transparent transparent transparent;
}

.tooltip--left::after {
  inset-block-start: 50%;
  inset-inline-start: 100%;
  border-color: transparent transparent transparent #5e5d5d;
}

.tooltip--right::after {
  inset-block-start: 50%;
  inset-inline-end: 100%;
  border-color: transparent #5e5d5d transparent transparent;
}

.tooltip--top::after {
  inset-block-start: 100%;
  inset-inline-start: 50%;
  border-color: #5e5d5d transparent transparent  transparent;
}

.tooltip--bottom::after {
  inset-block-end: 100%;
  inset-inline-start: 50%;
  border-color: transparent transparent #5e5d5d transparent;
}

</style>