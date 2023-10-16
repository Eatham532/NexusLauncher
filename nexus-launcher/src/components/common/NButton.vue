<script setup lang="ts">
  const props = defineProps({
    disabled: {
      type: Boolean,
      default: false,
    },
    expand: {
      type: Boolean,
      default: false,
    },
    use_padding: {
      type: Boolean,
      default: false,
    },
    outline: {
      type: Boolean,
      default: false,
    }
  })
</script>

<template>
  <button :class="{
    'expand': props.expand,
    'use_padding': props.use_padding,
    'outline': props.outline,
  }
"
  :disabled="disabled">
    <slot></slot>
  </button>
</template>

<style scoped>
  button {
    color: #fff;
    background-color: #0066CC;
    box-shadow: var(--gray-900) 0px 0px 0px 0px;
    border-radius: 10px;
    transition : 300ms;
    transform: translateY(0);
    display: flex;
    flex-direction: row;
    align-items: center;
    cursor: pointer;
    border: transparent;
    user-select: none;

    &:hover {
      background-color: #fff;
      color: #0066cc;
    }
  }

  .expand:hover {
    transition : 500ms;
    transform : translateY(-0px);
    padding: 10px 10px;

    &:is(.use_padding) {
      padding: 10px 30px;
    }
  }

  .use_padding {
    padding: 10px 20px;
  }

  .outline {
    border: 2px solid #0066cc;

    &:hover {
      border: solid 2px #0066cc;
    }
  }

  :slotted(svg) {
    & path {
      fill: #fff;
    }
  }

  button:disabled,
  button[disabled] {
    background-color: rgb(128, 128, 128)!important;
    color: white!important;
    cursor: not-allowed;
    transition: none!important;
    transform: none!important;
    padding: 10px!important;

    &:is(.use_padding) {
      padding: 10px 20px!important;
    }
  }
</style>