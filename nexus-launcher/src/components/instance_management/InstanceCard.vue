<!--
TODO: FIX SERIALISATION RENAME OF SPECTA

-->

<script setup lang="ts">

import {NexusInstance, launchInstance, deleteInstance} from "../../scripts/rust/instances.ts";
import {listen} from "@tauri-apps/api/event";
import {ref} from "vue";
import NButton from "../common/NButton.vue";
import NContextMenu from "../common/NContextMenu.vue";
import {ask, message} from "@tauri-apps/api/dialog";
import NTooltip from "../common/NTooltip.vue";

const props = defineProps({
  instance: {
    type: Object as () => NexusInstance,
    default: () => {
      return {
        name: 'Invalid',
        modloader: '',
        game_version: '',
        install_stage: 'None',
        path: '',
        loader_version: '',
        id: '-1',
      };
    },
  },
});

console.log(props.instance);

let progress = ref(0);

interface ProgressPayload {
  id: string,
  progress: number,
  message: string,
}

if (props.instance != undefined) {
  if (props.instance.install_stage === 'Installed') {
    progress.value = 100;
  }
  else if (props.instance.install_stage === 'Installing') {
    progress.value = 1;
  }
}

listen<ProgressPayload>('game-install-progress', (event) => {
  if (props.instance != undefined) {
    if (event.payload.id === props.instance.id) {
      console.log(props.instance.id);
      console.log(event.payload.progress);
      progress.value = event.payload.progress;
    }
  }
});

function launchInstanceClick() {
  if (props.instance != undefined) {
    if (props.instance.install_stage == "Installed") {
      console.log("Launching Game...")
      launchInstance(props.instance);
    }
  }
}

function deleteInstanceClick() {
  ask(`Are you sure you want to delete the following instance? \n\nName: ${props.instance.name}\nVersion: ${props.instance?.game_version}`, {
    title: "Delete Instance",
    okLabel: "Delete",
    cancelLabel: "Cancel",
    type: "warning",
  }).then((response) => {
    if (response) {
      console.log("Deleting Instance...");
      deleteInstance(props.instance);
      window.location.reload();
    }
  });
}


const contextMenu: any = ref(null);
const openContextMenu = (e: any) => {
  contextMenu.value?.open(e);
};

</script>

<template>
  <div ref="btn" :class="{
    root: true,
    none: progress === 0,
    installed: progress >= 100,
  }" @click.self="$emit('click')" @click.right.prevent="openContextMenu">
    <div class="BtnContent">
      <div class="topbar">
        <NTooltip position="top" :text="instance.modloader">
          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-box"><path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/><polyline points="3.27 6.96 12 12.01 20.73 6.96"/><line x1="12" y1="22.08" x2="12" y2="12"/></svg>
        </NTooltip>
      </div>
      <div class="InstanceName"><p>{{ instance.name }}</p></div>
      <div class="bottom-section">
        <div class="BtnDiv">
          <NButton class="PlayButton" @click.self="launchInstanceClick" :disabled="progress != 100">
            <p v-if="progress >= 100">Play</p>
          </NButton>
        </div>
        <div class="StatsText" style="line-height: 5px">
          <p>{{ instance.modloader }}</p>
          <p>{{ instance.game_version }}</p>
        </div>
      </div>
    </div>
    <NContextMenu ref="contextMenu">
      <ul class="contextMenu">
        <li @click="message('That action is still in development')"><p>Manage Instance</p></li>
        <li class="warn" @click="deleteInstanceClick">

          <p>Delete Instance <svg xmlns="http://www.w3.org/2000/svg" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-trash-2"><polyline points="3 6 5 6 21 6"/><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/><line x1="10" y1="11" x2="10" y2="17"/><line x1="14" y1="11" x2="14" y2="17"/></svg></p>
        </li>
      </ul>
    </NContextMenu>
  </div>
</template>

<style scoped>
  .root {
    flex-flow: column;
    background-color: var(--primary-800);
    aspect-ratio: 1;
    padding: 10px;
    border-radius: 10px;
    width: 200px;
    box-shadow: var(--primary-900) 2px 2px 2px;

    & .topbar {
      position: relative;
      display: flex;
      justify-content: flex-end;
      align-items: center;
      margin: 10px;
    }

    & .BtnContent {
      display: flex;
      flex-flow: column nowrap;
      height: 100%;
      justify-content: space-between;
      top: 0;
      bottom: 0;

      & .InstanceName {
        margin-left: 15px;
        font-size: 20px;
        overflow: hidden;
      }

      & .bottom-section {
        display: flex;
        justify-content: space-between;
        vertical-align: bottom;

        & .BtnDiv {
          padding: 10px;
          display: flex;
          flex-flow: column;
          justify-content: end;

          & .PlayButton {
            width: 35px;
            height: 35px;
            aspect-ratio: 1;


            &:enabled {
              cursor: pointer;
              background-color: var(--secondary-400);
            }

            &:disabled {
              background-color: var(--secondary-200);
            }
          }
        }

        & .StatsText {
          padding: 0 10px;
          text-align: right;
          color: var(--gray-400)
        }
      }
    }

    &:hover {
      background-color: var(--primary-700);
      box-shadow: var(--primary-900) 3px 3px 2px;
    }
  }

  .none {
    & .BtnContent {
      & .bottom-section {
        & .BtnDiv {
          & .PlayButton {
            background-color: var(--secondary-100)!important;
          }
        }
      }
    }
  }

  .installed {
    & .BtnContent {
      & .bottom-section {
        & .BtnDiv {
          & .PlayButton {
            background-color: var(--secondary-800)!important;
          }
        }
      }
    }
  }

  .contextMenu {
    background-color: red;

    & li {
      color: var(--gray-900);
      list-style-type: none;
      padding: 10px;
      background-color: var(--gray-200);
      cursor: pointer;
      transition: all .5s;
      font-size: 0.9rem;
      display: flex;
      gap: 10px;

      & p {
        margin: 0;
      }

      &:hover {
        background-color: var(--gray-500);

        &:is(.warn) {
          background-color: #a82a2a;
        }
      }


    }

    & svg {
      color: black;
    }
  }
</style>