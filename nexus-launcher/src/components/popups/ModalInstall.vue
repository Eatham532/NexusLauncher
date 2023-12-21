<script setup lang="ts">
import {computed, Ref, ref} from "vue";
  import NButton from "../common/NButton.vue";
import {getVersions, installInstance, Modloader, MVersion, Type} from "../../scripts/rust/instances.ts";
  import {pick_folder} from "../../scripts/fs.ts";
import {ask, message} from "@tauri-apps/api/dialog"
  import {v1 as uuidV1} from "uuid";
import {getAppConfig} from "../../scripts/rust/config.ts";
import {fs, path} from "@tauri-apps/api";
import {readDir} from "@tauri-apps/api/fs";
import NSearchSelect from "../common/NSelect.vue";
import NOptionSelect from "../common/NOptionSelect.vue";
import {PistonMetadata} from "../../scripts/rust/instances.ts";
import {usePistonMeta} from "../../store/metadata.ts";


  const emit = defineEmits(['close']);

  const show_advanced = ref(false);

  const install_dir = ref("");

  let version_list = usePistonMeta().versions;

  let selected_modloader: Ref<string[]> = ref(["Vanilla"]);
  let selected_version = ref("--LOADING VERSIONS--");
  let selected_loader_version = ref("--LOADING VERSIONS--");
  const name = ref("");

  let set_name = false;
  let set_path = false;

  function name_changed() {
    if (name.value != selected_modloader.value + " " + selected_version.value) {
      set_name = true;
      get_modified_path().then((p) => install_dir.value = p);
    }
    else {
      set_name = false;
    }
  }

  function path_changed() {
    get_modified_path().then((p) => set_path = install_dir.value != p);
  }

  function selected_version_changed() {
    if (!set_name) {
      name.value = selected_modloader.value + " " + selected_version.value;

      if (!set_path) {
        get_modified_path().then((p) => install_dir.value = p);
      }
    }
  }

  async function get_modified_path(): Promise<string> {
    let config = await getAppConfig();

    if (selected_modloader.value.includes("Vanilla")) {
      return await path.join(config.default_instances_dir, name.value.toLowerCase().replace(" ", "-"));
    }
    else {
      return await path.join(config.default_instances_dir, name.value.toLowerCase().replace(" ", "-") + "-" + selected_loader_version.value);
    }
  }



  async function pickInstallDir() {
    console.log("Pick Install Dir");
    let folder = await pick_folder();

    if (folder != undefined) {
      install_dir.value = folder;
    }
  }

  const checkDisableInstall = computed(() => {
    console.log("UPDATE DISABLE")
    if (computed_version_list.value.includes(selected_version.value)) {
      if (!selected_modloader.value.includes("Vanilla")) {
        if (!computed_loader_version_list.value.includes(selected_loader_version.value)) {
          console.log("Not there?")
          return true;
        }
      }

      console.log("There");
      return false;
    }
    return true;
  })


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

    if (!selected_modloader.value.includes("Vanilla")) {
      if (selected_loader_version.value == "") {
        return
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
      loader_version: selected_modloader.value.includes("Vanilla") ? null : selected_loader_version.value,
      path: install_dir.value,
      name: name.value,
      game_version: selected_version.value,
      modloader: selected_modloader.value[0] as Modloader,
      install_stage: "Installing",
    }).then(() => {
      message("Instance installed");
    }).catch((err) => {
      message("Error installing instance: " + err);
    });
    window.location.reload();
    emit('close');
  }



  let computed_version_list: Ref<any[]> = computed(() => {
    let new_list: any[] = [];

    version_list.forEach((version) => {
      if ("Vanilla" == selected_modloader.value[0] || selected_modloader.value[0].toLowerCase() in version.modloaders) {
        new_list.push(version.id);
      }
    })

    return new_list;
  });

  let computed_loader_version_list: Ref<any[]> = computed(() => {
    let new_list: any[] = [];

    let loader_versions = version_list.find((x) => selected_version.value == x.id)?.modloaders[selected_modloader.value[0].toLowerCase()];

    for (let v in loader_versions) {
      new_list.push(loader_versions[v].id);
    }

    const compareVersions = (a: string, b: string) => {
      const aParts = a.split('.');
      const bParts = b.split('.');
      const len = Math.min(aParts.length, bParts.length);

      for (let i = 0; i < len; i++) {
        const aPart = +aParts[i] || 0;
        const bPart = +bParts[i] || 0;

        if (aPart !== bPart) {
          return aPart > bPart ? 1 : -1;
        }
      }

      return bParts.length - aParts.length;
    };

    return new_list.sort(compareVersions).reverse();
  });



  selected_version.value = "";
  selected_loader_version.value = "";
  selected_version_changed();
</script>

<template>
  <div class="rootDiv" @click="$emit('close')">
    <div class="modalContent" @click.stop> <!-- A div that contains the content -->
      <div class="titlebar">
        <p>Add a new Instance</p>
        <NButton class="closeBtn" transparent @click="emit('close')">✖</NButton>
      </div>

      <div class="options"> <!-- A div that contains the options -->
        <div class="normal group">
          <div class="option">
            <p>Instance Name</p>
            <input class="NTextbox" type="text" placeholder="Some cool instance name" v-model="name" v-on:change="name_changed">
          </div>

          <div class="option">
            <p>Loader Type</p>
            <NOptionSelect :disabled="selected_version == '--LOADING VERSIONS--'" v-model:selected="selected_modloader" v-on:update:selected="selected_version_changed(); selected_loader_version = ''" :selected_items="selected_modloader" :items="['Vanilla', 'Fabric', 'Quilt', 'NeoForge', 'Forge']" style="color: white; font-size: 0.8em"></NOptionSelect>
          </div>

          <div class="option">
            <p>Game Version</p>
            <NSearchSelect class="NSearchSelect" filter search v-on:change="selected_version_changed" v-model:value="selected_version" :options="computed_version_list" placeholder="Search for a version"></NSearchSelect>
          </div>
          <div class="option" v-if="!selected_modloader.includes('Vanilla')">
            <p>Loader Version</p>
            <NSearchSelect class="NSearchSelect" filter search v-on:change="selected_version_changed" v-model:value="selected_loader_version" :options="computed_loader_version_list" placeholder="Search for a loader version"></NSearchSelect>
          </div>
        </div>

        <div style="padding: 20px 0 0 0">
          <label><b>Show advanced options</b></label><input class="showAdvanced" type="checkbox" v-model="show_advanced">
        </div>

        <div class="advanced group" v-if="show_advanced">
          <div class="option">
            <p style="margin: 20px 0 10px 0">Instance Data Path</p>
            <div style="display: flex; gap: 10px">
              <input class="NTextbox" type="text" placeholder="" v-model="install_dir" v-on:change="path_changed">
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
        <NButton class="button" @click="install" :disabled="checkDisableInstall" use_padding expand>Install</NButton>
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
    overflow-y auto;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;

    & .modalContent {
      background: White;
      border-radius: 0.5rem;
      color: #000000;
      box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
      width: 400px;

      & .options {
        padding 0 30px 30px 30px;
        max-height 60vh;
        overflow-y auto;

        & .group {
          display flex;
          flex-flow column;
          gap 20px;
        }

        & .option {
          display inline-block;
          flex-flow column;
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

.closeBtn {
  padding 10px;
}
</style>