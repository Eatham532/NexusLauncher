<script setup lang="ts">
const props = defineProps({
  type: {
    type: String,
    default: 'default',
  }
});

</script>

<template>
  <div :class="{
    'chip': true,
    'chip-checkbox': props.type === 'checkbox',
    'chip-radio': props.type === 'radio',
    'chip-filter': props.type === 'filter',
  }">
    <input type="checkbox" name="radioEx" />
    <span id="checkOneLabel"><slot></slot></span>
  </div>
</template>

<style lang="stylus" scoped>
  .chip {
    transition: all 0.3s ease-in-out;
    background-color: #fff;
    border-radius: 30px;
    border: 1px solid #d3d2d2;
    display: inline-flex;
    align-items:center;
    padding: 4px 6px 4px 6px;
    margin-right:6px;
    margin-bottom:6px;
    color: black;
    user-select none;
    cursor pointer;

    & input {
      display: none;
    }
  }

  .chip > * {
    margin-right:2px;
    margin-left:2px;
    color: #757373;
  }

  .chip.active {
    background-color: #d3d2d2;
    border-color:#c3c2c2;
    box-shadow: 0 1px 1px rgba(0,0,0,0.12)
  }

  .chip:hover, .chip:focus {
    background-color: #e3e2e2;
  }
  .chip:active:focus{
    background-color: #d3d2d2;
  }

  .chip.chip-checkbox,
  .chip.chip-toggle,
  .chip.clickable{
    cursor: pointer;
  }

  .chip.chip-checkbox > .chip-add-icon::after {
    content:"add"
  }
  .chip.chip-checkbox.active > .chip-add-icon::after {
    content:"done"
  }

  .chip.chip-checkbox > .chip-add-icon::after {
    content:"add"
  }
  .chip.chip-checkbox.active > .chip-add-icon::after {
    content:"done"
  }
</style>