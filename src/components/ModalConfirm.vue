<script setup lang="ts">

</script>

<template>
  <div
      class="root">
    <div>
      <div
          class="modalContent"
          role="dialog"
          aria-modal="true">
        <h3>Do you confirm?</h3>
        <div>
          <!-- We will handle these emits later -->
          <button @click="$emit('close')">No</button>
          <button @click="$emit('close')">Yes</button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
  .root {
    position: absolute;
    top: 0;
    bottom: 0;
    left: 0;
    right: 0;
    overflow-y: auto;
    background: rgba(0,0,0,0.5);

    & div {
      margin-top: 6rem;
      display: flex;
      align-items: flex-start;
      justify-content: center;

      & .modalContent {
        background: White;
        padding: 1.5rem;
        border-radius: 0.5rem;
        text-align: center;
        color: #000000;
        box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);

        & div {
          padding-top: 1rem;
          padding-bottom: 1rem;
          justify-content: center;

          & button {
            margin: 0 20px;
            border-radius: 30px;
            padding: 10px 20px;
            border-color: #000000;
            color: #000000;
            background-color: #ffffff;
          }
        }
      }
    }
  }
</style>