<script setup lang="ts">

import {markRaw, ref} from "vue";
import {listen} from "@tauri-apps/api/event";
import {getPfpPath, getUsers} from "../../scripts/rust/user.ts";
import {convertFileSrc} from "@tauri-apps/api/tauri";
import ModalAccount from "../popups/ModalAccount.vue";
import {useModal} from "../../composables/useModal.ts";
import NButton from "../common/NButton.vue";

let emit = defineEmits(['addAccount'])
let focused = ref(false);

const openAcc = () => {
  focused.value = false;
  console.log("addAccount");
  emit('addAccount');
}

let pfpImage = ref("/ProfileIconPlaceholder.png");
async function reload_icon() {
  let users = await getUsers();
  let default_path = "/ProfileIconPlaceholder.png"

  console.log(users.active);
  if (users.active) {
    let path = await getPfpPath(users.active);
    console.log(path);

    if (path == "") {
      pfpImage.value = default_path;
    } else {
      let p = convertFileSrc(path);
      console.log(p);
      pfpImage.value = p;
    }
  }
  else {
    pfpImage.value = default_path;
  }
  console.log(pfpImage.value);
}

const handleFocusOut = () => {
  focused.value = false;
};

type AuthPayload = { stage: AuthStage }
type AuthStage = "Start" | { AuthCode: { code: string; url: string } } | "Cancelled" | "Complete";


listen<AuthPayload>('auth_login', (event) => {
  if (typeof event.payload.stage === 'string') {
    switch (event.payload.stage) {
      case "Complete":
        reload_icon()
        break;
    }
  }
})

reload_icon();
</script>

<template>
  <div class="pfp" ref="pfpBtn" v-bind:style="{ '--pfp-image': 'url(' + pfpImage + ')' }" @focus="focused=true">
    <div class="menu-wrapper" v-if="focused">
      <div class="menu">
        <div class="account">
        </div>
        <NButton use_padding @click="openAcc">Add Account</NButton>
        Yes this is buggy
      </div>
    </div>
  </div>

</template>

<style scoped>
:root {
  position: absolute;
  --pfp-image: "/ProfileIconPlaceholder.png";
  display: grid;
}

.pfp {
  border: black 2px solid;
  background-size: cover;
  background-image: linear-gradient(rgba(0, 0, 0, 0.3), rgba(0, 0, 0, 0.3)), var(--pfp-image);
  aspect-ratio: 1;
  border-radius: 28px;
  padding: 0;
  overflow: visible;
  margin:0;
  cursor: pointer;
}

.pfp:is(:focus, :focus-within) {
  background-image: linear-gradient(rgba(75, 30, 30, 0.3), rgba(91, 134, 164, 0.3)), var(--pfp-image);
  transform: scale(1.05);
  transition: all .5s;
}

.menu-wrapper {
  width: 200px;
  height: 200px;
  padding: 0 0 0 30px;
  position: absolute;
  top: 0;
  left: 80%;

  & .menu {
    color: black;
    background-color: white;
    width: 100%;
    height: 100%;
    border-radius: 0.5rem;
  }
}
</style>