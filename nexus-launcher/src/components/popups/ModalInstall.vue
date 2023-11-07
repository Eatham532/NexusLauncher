<script setup lang="ts">
import {Ref, ref} from "vue";
  import NButton from "../common/NButton.vue";
  import {getVersions, installInstance, Version} from "../../scripts/rust/instances.ts";
  import {pick_folder} from "../../scripts/fs.ts";
import {ask, message} from "@tauri-apps/api/dialog"
  import {v1 as uuidV1} from "uuid";
import {getAppConfig} from "../../scripts/rust/config.ts";
import {fs, path} from "@tauri-apps/api";
import {readDir} from "@tauri-apps/api/fs";

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
        path.join(config.default_instances_dir, name.value).then((path: any) => {
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
    let options = {
      name: name.value,
      version: selected_version.value,
      install_dir: install_dir.value,
    };

    for (let x in options) {
      if (x == "") {
        await message("Please fill out all fields");
        return;
      }
    }

    if (await fs.exists(install_dir.value)) {
      try {
        const files = await readDir(install_dir.value);
        if (await fs.exists(await path.join(install_dir.value, 'instance_conf.json'))) {
          await message("An instance is already installed at that directory", {type: "error"});
          return;
        }
        if (files.length > 0) {
          if (!await ask("Warning install directory is not empty. Are you sure that you want to install here?", {type: "warning"})) {
            return;
          }
        }
      } catch (error) {
        console.error('Error reading directory:', error);
        return;
      }
    }

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

          <label><b>Show advanced options</b></label><input class="showAdvanced" type="checkbox" v-model="show_advanced">

          <div class="advanced" v-if="show_advanced">
            <div class="option">
              <p style="margin: 20px 0 10px 0">Install Location</p>
              <div style="display: flex; gap: 10px">
                <input class="NTextbox" type="text" placeholder="" v-model="install_dir">
                <NButton @click="pickInstallDir">
                  <svg style="fill: black" xmlns="http://www.w3.org/2000/svg" x="0px" y="0px" height="1.5em" viewBox="0 0 48 48">
                    <path  d="M 8.5 8 C 6.0324991 8 4 10.032499 4 12.5 L 4 36.40625 L 4.015625 36.40625 C 3.9865145 38.276932 5.5083888 40 7.5019531 40 L 36.546875 40 C 38.416875 40 40.10278 38.832994 40.759766 37.082031 L 45.767578 23.75 L 45.767578 23.746094 C 46.62354 21.501657 44.900814 19 42.498047 19 L 42 19 L 42 17.5 C 42 15.032499 39.967501 13 37.5 13 L 24.042969 13 L 19.574219 9.2753906 A 1.50015 1.50015 0 0 0 19.572266 9.2753906 C 18.584102 8.4521105 17.339162 8 16.052734 8 L 8.5 8 z M 8.5 11 L 16.052734 11 C 16.638307 11 17.202555 11.205358 17.652344 11.580078 L 22.539062 15.652344 A 1.50015 1.50015 0 0 0 23.5 16 L 37.5 16 C 38.346499 16 39 16.653501 39 17.5 L 39 19 L 13.453125 19 C 11.583125 19 9.8972196 20.167006 9.2402344 21.917969 L 7 27.882812 L 7 12.5 C 7 11.653501 7.6535009 11 8.5 11 z M 13.453125 22 L 42.498047 22 C 42.897907 22 43.107444 22.305152 42.964844 22.677734 A 1.50015 1.50015 0 0 0 42.960938 22.6875 L 37.951172 36.027344 C 37.730157 36.616381 37.176875 37 36.546875 37 L 7.5019531 37 C 7.1042373 37 6.8935735 36.697099 7.0332031 36.326172 A 1.50015 1.50015 0 0 0 7.0351562 36.324219 L 12.048828 22.972656 C 12.269843 22.383619 12.823125 22 13.453125 22 z"></path>
                  </svg>
                </NButton>
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

<style lang="stylus" scoped>
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

  .NTextbox {
    width: calc(100% - 70px);
  }
</style>