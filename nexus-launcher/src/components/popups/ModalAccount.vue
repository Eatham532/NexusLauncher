<script setup lang="ts">
  import {listen} from "@tauri-apps/api/event";
  import {ref} from "vue";
  import {cancelAuth, startLogin} from "../../scripts/rust/auth.ts";
  import {writeText} from "@tauri-apps/api/clipboard";
  import NButton from "../common/NButton.vue";
  import { open } from '@tauri-apps/api/shell';
  const emit = defineEmits(['close']);

  const auth_code = ref("********")
  const auth_url = ref("")

  const copied = ref(false)

  type AuthPayload = { stage: AuthStage }
  type AuthStage = "Start" | { AuthCode: { code: string; url: string } } | "Cancelled" | "Complete";

  listen<AuthPayload>('auth_login', (event) => {
    if (typeof event.payload.stage === 'string') {
      switch (event.payload.stage) {
        case "Start":
          console.log("Authentication Started");
          break;
        case "Cancelled":
          console.log("Authentication Cancelled");
          break;
        case "Complete":
          console.log("Authentication Complete");
          emit('close');
          break;
      }
    } else if ('AuthCode' in event.payload.stage) {
      console.log("Logging in...");
      console.log(`Auth Code: ${event.payload.stage.AuthCode.code}`);
      console.log(`URL: ${event.payload.stage.AuthCode.url}`);
      console.log();
      auth_code.value = event.payload.stage.AuthCode.code;
      auth_url.value = event.payload.stage.AuthCode.url;
    }
  });

  const copy = () => {
    copied.value = false;
    setTimeout(function () {
      writeText(auth_code.value);
      copied.value = true;
    }, 100)
  }

  const go_to_url = () => {
    copy();
    open(auth_url.value);
  }

  const close_and_cancel = () => {
    cancelAuth();
    emit('close');
  }
  startLogin();
</script>

<template>
  <div class="rootDiv" @click="$emit('close')">
    <div class="modalPosDiv"> <!--A div to position the modal inside-->
      <div class="modalContent" @click.stop> <!-- A div that contains the content -->
        <div class="titlebar">
          <p>Login with Microsoft</p>
          <NButton class="closeBtn" transparent @click="close_and_cancel">✖</NButton>
        </div>

        <div class="content">
          <div>
            <h2 style="margin: 0 0 20px 0">Copy this code:</h2>
            <div style="display: flex; gap: 10px" @click="copy">
              <div class="auth_code">{{auth_code}}</div>
            </div>
            <!--
                      <NButton use_padding><img src="https://www.htmlcssbuttongenerator.com/iconExample-search-thin.svg"></NButton>
            -->
            <p style="padding: 0; margin: 10px 0 0 0" v-if="copied">Copied code to your clipboard</p>
          </div>
          <div>
            <h2>And enter it at this url</h2>
            <div style="display: flex; gap: 10px">
              <input class="NTextbox" type="text" disabled v-model="auth_url">
              <NButton use_padding :disabled="auth_url == ''" @click="go_to_url">Copy and Open Link</NButton>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style lang="stylus" scoped>
.titlebar {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  align-items: center;
  border-radius: 0.5rem;
  padding: 0 15px;

  background: var(--gray-300);
}

.rootDiv {
  position: absolute;
  top: 0;
  bottom: 0;
  left: 0;
  right: 0;
  overflow-y: auto;
  background: rgba(0, 0, 0, 0.5);

  & .modalPosDiv {
    margin-top: 6rem;
    display: flex;
    align-items: flex-start;
    justify-content: center;


    & .modalContent {
      background: White;

      border-radius: 0.5rem;
      color: #000000;
      box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);

      display: flex;
      flex-direction: column;

      .content {
        padding: 30px;
      }
    }
  }
}

.auth_code {
  font-family: monospace;
  border-radius: 20px;
  letter-spacing: 10px;
  padding: min(max(16px, 3vw), 30px) min(max(16px, 4vw), 40px);
  font-size: 2em;
  color: var(--gray-100);
  background: var(--gray-900);
  text-align: center;
  border: 10px var(--gray-900) solid;
  cursor: pointer;

  &:hover {
    background: var(--gray-600);
  }
}

.NTextbox {
  width: 80%;
}

  .closeBtn {
    padding 10px;
  }
</style>