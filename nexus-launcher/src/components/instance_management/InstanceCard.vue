<!--
TODO: FIX SERIALISATION RENAME OF SPECTA

-->

<script setup lang="ts">

import {NexusInstance, launchInstance} from "../../scripts/rust/instances.ts";
import {listen} from "@tauri-apps/api/event";
import {ref} from "vue";
import NButton from "../common/NButton.vue";
import NContextMenu from "../common/NContextMenu.vue";
import {message} from "@tauri-apps/api/dialog";

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
    <div class="spacer"></div>
    <div class="BtnContent">
      <div class="InstanceName"><p>{{ instance.name }}</p></div>
      <div class="bottom-section">
        <div class="BtnDiv">
          <NButton class="PlayButton" @click.self="launchInstanceClick" :disabled="progress != 100">
            Play
          </NButton>
        </div>
        <div class="StatsText">
          <p>{{ instance.modloader }}</p>
          <p>{{ instance.game_version }}</p>
        </div>
      </div>
      <div></div>
    </div>
    <NContextMenu ref="contextMenu">
      <ul class="contextMenu">
        <li @click="message('That action is still in development')">Manage Instance</li>
        <li class="warn" @click="message('That action is still in development')">
          Delete Instance
          <svg xmlns="http://www.w3.org/2000/svg" height="1.2rem" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-trash-2"><polyline points="3 6 5 6 21 6"/><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/><line x1="10" y1="11" x2="10" y2="17"/><line x1="14" y1="11" x2="14" y2="17"/></svg>
        </li>
      </ul>
    </NContextMenu>
  </div>
</template>

<style scoped>
  .root {
    background-color: var(--primary-800);
    aspect-ratio: 1;
    padding: 0;
    border-radius: 10px;
    margin: 20px;
    width: 200px;
    vertical-align: top;
    overflow: hidden;
    box-shadow: var(--primary-900) 2px 2px 2px;

    & .spacer {
      height: 40px
    }

    & .BtnContent {
      display: flex;
      flex-flow: column nowrap;
      justify-content: space-evenly;
      height: 160px;
      top: 0;
      bottom: 0;
      padding: 0 10px 0 10px;

      & .InstanceName {
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
          text-align: right;
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