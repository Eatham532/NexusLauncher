<script setup lang="ts">
  import {ref} from "vue";
  import NButton from "../common/NButton.vue";

  const page_number = ref(1);
  const check_view_backBtn = () => {
    switch (page_number.value) {
      case 1:
        return false;
      case 3:
        return false;
      case 4:
        return false;
      default:
        return true;
    }
  };

  const check_view_nextBtn = () => {
    switch (page_number.value) {
      case 2:
        return false;
      case 3:
        return false;
      case 4:
        return false;
      default:
        return true;
    }
  };

  const view_backBtn = ref(false);
  const view_nextBtn = ref(true);

  const reload_btn_states = () => {
    view_backBtn.value = check_view_backBtn();
    view_nextBtn.value = check_view_nextBtn();
  }

  const next_btn_click = () => {
    page_number.value++;
    reload_btn_states();
  }

  const back_btn_click = () => {
    page_number.value--;
    reload_btn_states();
  }


  reload_btn_states();

</script>

<template>
  <div class="rootDiv">
    <div class="content-wrapper">
      <div class="content">
        <div class="page_2" v-if="page_number == 1">
          <h2>Hi! Thanks for downloading Nexus Launcher!</h2>
        </div>
        <div class="page_2" v-if="page_number == 2">
          <h2>Minecraft requires that JRE to be installed on your system</h2>
          <p style="font-size: 0.8rem"><i>JRE is the Java Runtime Environment. Not to be confused with the name of the game.</i></p>
          <p>How would you like to set it up?</p>

          <div class="jre_actions">
            <div>
              <NButton @click="next_btn_click" class="NButton" use_padding>Automatic Setup (Recommended)</NButton>
              <NButton @click="next_btn_click(); next_btn_click();" class="NButton" use_padding>Manual Setup</NButton>
            </div>
          </div>
        </div>
        <div v-if="page_number == 3">
          <h1>Automatic Setup</h1>
          <div>
            <NButton @click="back_btn_click">Back</NButton>
          </div>
        </div>
        <div v-if="page_number == 4">
          <h1>Manual Setup</h1>
          <div style="display: flex; flex-flow: column; margin: 50px 0; gap: 20px">
            <div>
              <label>Java 8 path </label>
              <input type="text" class="NTextbox">
            </div>

            <div>
              <label>Java 17 path </label>
              <input type="text" class="NTextbox">
            </div>
          </div>
          <NButton @click="back_btn_click(); back_btn_click()">Back</NButton>
        </div>
      </div>
    </div>
    <div class="actions">
      <div>
        <NButton v-if="view_backBtn" @click="back_btn_click" use_padding>Back</NButton>
      </div>
      <div>
        <NButton v-if="view_nextBtn" @click="next_btn_click" use_padding>Next</NButton>
      </div>
    </div>
  </div>
</template>

<style scoped>
  .rootDiv {
    background: var(--primary-900);
    top: 0;
    right: 0;
    left: 0;
    bottom: 0;
    position: absolute;
    display: flex;
    flex-flow: column;
    justify-content: space-between;

    & .content-wrapper {
      height: 100%;
      display: flex;
      align-items: center;
      justify-content: center;
      padding-top: 60px;

      & .content {
        border-radius: 0.5em;
        padding: 30px;
        color: black;
        background-color: white;
      }
    }

    & .actions {
      display: flex;
      justify-content: space-between;
      padding: 0 60px 40px;
      gap: 50px;
    }
  }

  .page_2 {
    & .jre_actions {
      margin-top: 50px;
      display: flex;
      justify-content: center;
      align-items: center;

      & .NButton {
        display: flex;
        flex-flow: column;
        width: 100%;
      }

      & div {
        display: flex;
        flex-flow: column;
        gap: 10px;
      }
    }
  }
</style>