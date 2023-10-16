<script setup lang="ts">

import {appWindow} from "@tauri-apps/api/window";
import {ref} from "vue";
const showBtns = ref(true);

</script>

<template>
  <div class="titlebar-button" id="titlebar-minimize" @click="appWindow.minimize()">
    <img
        src="https://api.iconify.design/mdi:window-minimize.svg"
        alt="minimize"
    />
  </div>
  <div class="titlebar-button" id="titlebar-maximize" @click="appWindow.toggleMaximize()">
    <img
        src="https://api.iconify.design/mdi:window-maximize.svg"
        alt="maximize"
    />
  </div>
  <div class="titlebar-button" id="titlebar-close" @click="appWindow.close()">
    <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
  </div>
</template>

<style scoped>
.titlebar-button {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 30px;
  height: 30px;

  visibility: v-bind(showBtns);
}
.titlebar-button:hover {
  background: #5bbec3;
}

#titlebar-close {
  border-radius: 0 5px 0 0;
}
</style>