<script setup lang="ts">

</script>

<template>
  <input type="range">
</template>

<style scoped>
/*********** Baseline, reset styles ***********/
input[type="range"] {
  -webkit-appearance: none;
  appearance: none;
  background: transparent;
  cursor: pointer;
  width: 100%;
}

/* Removes default focus */
input[type="range"]:focus {
  outline: none;
}

/******** Chrome, Safari, Opera and Edge Chromium styles ********/
/* slider track */
input[type="range"]::-webkit-slider-runnable-track {
  background-color: var(--secondary-200);
  border-radius: 1rem;
  height: 0.7rem;
}

/* slider thumb */
input[type="range"]::-webkit-slider-thumb {
  -webkit-appearance: none; /* Override default look */
  appearance: none;
  margin-top: -6.4px; /* Centers thumb on the track */
  background-color: var(--gray-700);
  border-radius: 0.5rem;
  height: 1.5rem;
  width: 1.5rem;
}

input[type="range"]:focus::-webkit-slider-thumb {
  outline: 3px solid var(--gray-700);
  outline-offset: 0.125rem;
}

/*********** Firefox styles ***********/
/* slider track */
input[type="range"]::-moz-range-track {
  background-color: var(--secondary-200);
  border-radius: 1rem;
  height: 0.7rem;
}

/* slider thumb */
input[type="range"]::-moz-range-thumb {
  background-color: var(--gray-700);
  border: none; /*Removes extra border that FF applies*/
  border-radius: 0.5rem;
  height: 1.5rem;
  width: 1.5rem;
}

input[type="range"]:focus::-moz-range-thumb{
  outline: 3px solid var(--gray-700);
  outline-offset: 0.125rem;
}
</style>