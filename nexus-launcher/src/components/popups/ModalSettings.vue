<script setup lang="ts">

import SettingsSidebarBtn from "../settings/SettingsSidebarBtn.vue";
import NButton from "../common/NButton.vue";
import {Ref, ref} from "vue";
import {pick_folder} from "../../scripts/fs.ts";
import {animate} from "../../scripts/utils.ts";
import {writeAppConfig, AppConfig, getAppConfig} from "../../scripts/rust/config.ts";
import {ask} from "@tauri-apps/api/dialog";

const emit = defineEmits(['close']);

const config: Ref<AppConfig> = ref({
  metadata_dir: "",
  default_instances_dir: "",
});

let original_conf: AppConfig = {
  metadata_dir: "",
  default_instances_dir: "",
};

const selectedBtn = ref('');
const showNotice = ref(false);

getAppConfig().then((value) => {
  config.value = value;
  original_conf = value;
});

function goTo(id: string) {
  const element = document.getElementById(id);
  if (element) {
    /*
        element.scrollIntoView({behavior: "smooth", block: 'nearest', inline: 'start'});
    */

    let parentNode : HTMLDivElement = element.parentNode as HTMLDivElement;
    let targetTop = element.offsetTop - parentNode.offsetTop;
    let currentTop = parentNode.scrollTop;

    animate(function (v) {
      parentNode.scrollTop = v;
    }, currentTop, targetTop, 200, function (p) {
      return p;
    });
  }

  selectedBtn.value = id;
  console.log(selectedBtn.value);
}

/*function scroll() {
  /!*const element = document.getElementById("settingsOptions");

  console.log(element?.scrollTop);

  let scrollPos : number = element?.scrollTop?? 0;

  switch (true) {
    case (scrollPos < 500):
      selectedBtn.value = "Test1";
      break;

    case (scrollPos < 1000):
      selectedBtn.value = "Test1";
      break;
  }*!/
}*/




async function pickMetadataDir() {
  let dir = await pick_folder();

  if (dir != undefined) {
    config.value.metadata_dir = dir;
  }
}

async function pickInstanceDefaultDir() {
  let dir = await pick_folder();

  if (dir != undefined) {
    config.value.default_instances_dir = dir;
  }
}

function save() {
  writeAppConfig(config.value).then(() => {
    original_conf = config.value;
    showNotice.value = true;
    setTimeout(() => {
      showNotice.value = false;
    }, 3000);
  });
}

async function close() {
  let new_conf: AppConfig = config.value;
  if (original_conf !== new_conf) {
    const response = await ask('Continuing will discard all your changes. Are you sure you want to continue?', { title: 'You forgot to click save!', type: 'warning' });
    console.log("Discarding...");
    console.log(original_conf);
    console.log(new_conf);
    if (response) {
      emit("close");
    }
  }
  else {
    console.log("Discarding...");
    console.log(original_conf);
    console.log(new_conf);
    emit("close");
  }
}

</script>

<template>
  <div class="rootDiv">
    <div class="navigation">
      <div>
        <SettingsSidebarBtn :selected="selectedBtn == 'SettingDirectories'" @click="goTo('SettingDirectories')">Directories</SettingsSidebarBtn>
      </div>
      <div>
        <div :class="{notice: true, notice_fade: !showNotice}">
          <p>Settings Saved!</p>
        </div>
        <div class="actions">
          <NButton use_padding @click="close">Close</NButton>
          <NButton class="save" use_padding @click="save">Save</NButton>
        </div>
      </div>
    </div>
    <div id="settingsOptions">
      <h1 class="SettingDirectories">Directories</h1>
      <div class="option">
        <h2>Metadata Config</h2>
        <div style="display: flex; gap: 20px">
          <input class="NTextbox" type="text" v-model="config.metadata_dir">
          <NButton use_padding @click="pickMetadataDir">
            Pick Folder
          </NButton>
        </div>
      </div>
      <div class="option">
        <h2>Default instance directory</h2>
        <div style="display: flex; gap: 20px">
          <input class="NTextbox" type="text" v-model="config.default_instances_dir">
          <NButton use_padding @click="pickInstanceDefaultDir">
            Pick Folder
          </NButton>
        </div>
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
  width: 100%;
  height: 100%;
  display: flex;
  flex-flow: row;
  overflow: hidden;

  & .navigation {
    display: flex;
    flex-flow: column;
    justify-content: space-between;
    padding-bottom: 20px;
    width: 30vw;
    max-width: 70%;
    min-width: 250px;
    border-right: 3px var(--gray-100) solid;
  }

  & #settingsOptions {
    width: 100%;
    height: 100%;
    overflow-x: auto;
    overflow-y: auto;
    background: var(--primary-900);
  }
}

.actions {
  display: flex;
  justify-content: space-around;
}

.option {
  border-radius: 20px;
  margin: 10px;
  padding: 20px;
  background-color: var(--primary-500);
}

.notice {
  color: var(--gray-900);
  background-color: var(--primary-200);
  padding: 5px 15px;
  margin: 20px 0;
}

.notice_fade {
  transition: opacity 1s;
  opacity: 0;
}

.NTextbox {
  width: 50%;
}
</style>