<!-- Reference https://jsfiddle.net/BB3JK/47/ -->

<script setup lang="ts">
  import { ref } from "vue";

  const options = [
    { value: "red", text: "Red" },
    { value: "green", text: "Green" },
    { value: "blue", text: "Blue" },
  ];

  const selected = ref(options[0].value);
</script>

<template>
  <select v-model="selected" class="custom-select">
    <option v-for="option in options" :key="option.value" :value="option.value">
      {{ option.text }}
    </option>
  </select>
</template>

<style scoped>
.custom-select {
  --border-color: #ccc;
  --background-color: #fff;
  --hover-color: #eee;
  --focus-color: #ddd;
  --arrow-color: #333;

  border: 1px solid var(--border-color);
  border-radius: 4px;
  padding: 8px;
  background-color: var(--background-color);
  appearance: none;
}

.custom-select:hover {
  border-color: var(--hover-color);
}

.custom-select:focus {
  border-color: var(--focus-color);
}

.custom-select::-ms-expand {
  display: none;
}

.custom-select::after {
  content: "";
  position: absolute;
  right: 12px;
  top: calc(50% - 3px);
  width: 0;
  height: 0;
  border-left: 6px solid transparent;
  border-right: 6px solid transparent;
  border-top: 6px solid var(--arrow-color);
}
</style>