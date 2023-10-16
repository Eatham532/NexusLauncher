<script setup lang="ts">
import {Ref, ref} from "vue";
  import NButton from "../common/NButton.vue";
  import {getVersions, installInstance, Version, VersionManifestRoot} from "../../bindings.ts";
  import {pick_folder} from "../../scripts/fs.ts";
  import {message} from "@tauri-apps/api/dialog"
  import {v1 as uuidV1} from "uuid";
import {getAppConfig} from "../../config.ts";
import {path} from "@tauri-apps/api";

  const emit = defineEmits(['close']);

  const show_advanced = ref(false);


  const install_dir = ref("");

  let version_list: Ref<Version[]> = ref([{
    id:"--LOADING VERSIONS--",
    sha1:"0",
    time:"0",
    url:"",
    type:"0",
    releaseTime:"0",
    complianceLevel:-1,
  }]);

  let selected_version = ref("--LOADING VERSIONS--")

  getVersions().then((versions) => {
    version_list.value = versions.versions;
    selected_version.value = versions.latest.release;
    selected_version_changed();
  })

  let selected_loader = ref("Vanilla");

  const name = ref("");
  let set_name = false;
  function name_changed() {
    if (name.value != selected_loader.value + " " + selected_version.value) {
      set_name = false;
      getAppConfig().then((config) => {
        path.join(config.default_instances_dir, name.value).then((path) => {
          install_dir.value = path;
          console.log(install_dir.value);
        })
      })
    }
    else {
      set_name = true;
    }
  }

  function selected_version_changed() {
    if (!set_name) {
      name.value = selected_loader.value + " " + selected_version.value;
      getAppConfig().then((config) => {
        path.join(config.default_instances_dir, name.value).then((path) => {
          install_dir.value = path;
          console.log(install_dir.value);
        })
      })
    }
  }





  async function pickInstallDir() {
    console.log("Pick Install Dir");
    let folder = await pick_folder();

    if (folder != undefined) {
      install_dir.value = folder;
    }
  }

  async function install() {
    console.log("Installing...");
    let options = [
      name.value,
      selected_version.value,
      install_dir.value
    ];

    for (let x in options) {
      if (options[x] == "") {
        await message("Please fill out all fields");
        return;
      }
    }

    /// TODO: Fix installation progress
    installInstance({
      id: uuidV1(),
      loader_version: null,
      path: install_dir.value,
      name: name.value,
      game_version: selected_version.value,
      modloader: "Vanilla",
      install_stage: "Installing"
    }).then(() => {
      message("Instance installed");
    }).catch((err) => {
      message("Error installing instance: " + err);
    });
    window.location.reload();
    emit('close');
  }

</script>

<template>
  <div class="rootDiv" @click="$emit('close')">
    <div class="modalPosDiv"> <!--A div to position the modal inside-->
      <div class="modalContent" @click.stop> <!-- A div that contains the content -->
        <div class="options"> <!-- A div that contains the options -->
          <div class="normal">
            <div class="option">
              <p style="margin: 20px 0 10px 0">Instance Name</p>
              <input class="NTextbox" type="text" placeholder="Some cool instance name" v-model="name" v-on:change="name_changed">
            </div>

            <div class="option">
              <p style="margin: 20px 0 10px 0">Game Version</p>
              <select v-model="selected_version" v-on:change="selected_version_changed">
                <option v-for="version in version_list" >{{ version.id }}</option>
              </select>
            </div>
          </div>

          <label>Show advanced options</label><input class="showAdvanced" type="checkbox" v-model="show_advanced">

          <div class="advanced" v-if="show_advanced">
            <div class="option">
              <p style="margin: 20px 0 10px 0">Install Location</p>
              <div style="display: flex; gap: 10px">
                <input class="NTextbox" type="text" placeholder="" v-model="install_dir">
                <NButton @click="pickInstallDir">Select Folder</NButton>
              </div>
            </div>
          </div>
        </div>
        <div class="actions"> <!-- A div that contains the buttons-->
          <NButton class="button" @click="$emit('close')" use_padding>Cancel</NButton>
          <NButton class="button" @click="install" :disabled="selected_version == '--LOADING VERSIONS--'" use_padding expand>Install</NButton>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
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
        padding: 0 30px;
        border-radius: 0.5rem;
        color: #000000;
        box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
        width: 300px;

        & .options {
          & .option {
            margin-bottom: 30px;
          }


          & .normal {

          }

          & .showAdvanced {

          }

          & .advanced {

          }
        }

        & .actions {
          display: flex;
          justify-content: space-between;
          padding: 20px;
        }
      }
    }
  }
</style>