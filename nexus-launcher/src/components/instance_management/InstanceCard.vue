<!--
TODO: FIX SERIALISATION RENAME OF SPECTA

-->

<script setup lang="ts">

import {NexusInstance, launchInstance, deleteInstance} from "../../scripts/rust/instances.ts";
import {listen} from "@tauri-apps/api/event";
import {computed, ref} from "vue";
import NButton from "../common/NButton.vue";
import NContextMenu from "../common/NContextMenu.vue";
import {ask, message} from "@tauri-apps/api/dialog";

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
  console.log("Click")
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

const btn_hover = ref(false)
const card_hover = ref(false);

const card_base_hover = computed(() => {
  if (progress.value === 0) {
    return false;
  } else if (progress.value >= 100) {
    if (btn_hover.value) {
      return false;
    } else if (card_hover.value) {
      return true;
    }
  }
  return false;
});

</script>

<template>
  <div ref="btn" :class="{
    'root': true,
    'none': progress === 0,
    'installed': progress >= 100,
    'card-base-hover': card_base_hover,
  }" @click.self="$emit('click')" @click.right.prevent="openContextMenu" @mouseenter="card_hover = true" @mouseleave="card_hover = false">
    <div class="btn-content">
      <div class="top-bar">

      </div>
      <div class="instance-name"><p>{{ instance.name }}</p></div>

      <div class="bottom-section">
        <div class="btn-div" @click="launchInstanceClick">
          <NButton v-if="card_base_hover || btn_hover" square class="play-button" :disabled="progress != 100" @mouseenter="btn_hover = true" @mouseleave="btn_hover = false">
            <svg width="20px" height="20px" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000" stroke-width="1"><path d="M6.90588 4.53682C6.50592 4.2998 6 4.58808 6 5.05299V18.947C6 19.4119 6.50592 19.7002 6.90588 19.4632L18.629 12.5162C19.0211 12.2838 19.0211 11.7162 18.629 11.4838L6.90588 4.53682Z" fill="#000000" stroke="#000000" stroke-width="1" stroke-linecap="round" stroke-linejoin="round"></path></svg>
          </NButton>
          <div></div>
        </div>
        <div class="stats-text" style="line-height: 5px">
          <p>{{ instance.modloader }}</p>
          <p>{{ instance.game_version }}</p>
        </div>
      </div>
    </div>
    <NContextMenu ref="contextMenu">
      <ul class="context-menu">
        <li @click="message('That action is still in development')"><p>Manage Instance</p></li>
        <li class="warn" @click="deleteInstanceClick">

          <p>Delete Instance <svg xmlns="http://www.w3.org/2000/svg" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-trash-2"><polyline points="3 6 5 6 21 6"/><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/><line x1="10" y1="11" x2="10" y2="17"/><line x1="14" y1="11" x2="14" y2="17"/></svg></p>
        </li>
      </ul>
    </NContextMenu>
  </div>
</template>

<style lang="stylus" scoped>
  .root {
    flex-flow: column;
    background-color: var(--primary-800);
    aspect-ratio: 1;
    padding: 10px;
    border-radius: 10px;
    width: 200px;
    box-shadow: var(--primary-900) 2px 2px 2px;
    user-select: none;

    & .top-bar {
      display: flex;
      justify-content: flex-end;
      align-items: center;
      margin: 10px;
    }

    & .btn-content {
      display: flex;
      flex-flow: column nowrap;
      height: 100%;
      justify-content: space-between;
      top: 0;
      bottom: 0;
      min-width: auto;

      & .instance-name {
        margin-left: 15px;
        font-size: 20px;
        overflow: hidden;
      }

      & .bottom-section {
        display: flex;
        justify-content: space-between;
        vertical-align: bottom;

        & .btn-div {
          padding: 10px;
          display: flex;
          flex-flow: column;
          justify-content: end;

          & .play-button {
            position: absolute;
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

        & .stats-text {
          overflow: clip;
          padding: 0 10px;
          text-align: right;
          color: var(--gray-400)
        }
      }
    }
  }

  .card-base-hover:active {
    transform: scale(0.99);
    transition-duration: 50ms;
  }


  .none {
    & .PlayButton {
      background-color: var(--secondary-100)!important;
    }
  }

  .installed {
    & .btn-content {
      & .bottom-section {
        & .btn-div {
          & .play-button {
            background-color: var(--secondary-800)!important;
          }
        }

        & .btn-div:hover {
          transform: scale(1.05);
          transition: height .5s;

          & > .play-button {
            transform: scale(1.05);
            transition-duration: 1s;

            background-color: var(--secondary-600)!important;
          }
        }

        & .btn-div:active {
          transform: scale(0.9);
          transition: height .5s;
        }
      }
    }
  }

  .context-menu {
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
  }


</style>