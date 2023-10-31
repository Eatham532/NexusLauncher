<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup

import {appWindow} from "@tauri-apps/api/window"
import ProfileButton from "./components/sidebar/ProfileButton.vue";
import SidebarButton from "./components/sidebar/SidebarButton.vue";
import {markRaw} from "vue";
import ModalInstall from "./components/popups/ModalInstall.vue";
import ModalSettings from "./components/popups/ModalSettings.vue";
import {useModal} from "./composables/useModal.ts";
import InstanceViewer from "./components/instance_management/InstanceViewer.vue";
import NButton from "./components/common/NButton.vue";
import NotificationMenu from "./components/NotificationMenu.vue";
import ModalAccount from "./components/popups/ModalAccount.vue";

appWindow.theme().then((theme) => {
  if (theme != null) {
    document.documentElement.setAttribute('data-theme', theme)
    console.log(document.documentElement.getAttribute('data-theme'));
  }
})



const modal = useModal();

const openInstall = () => {
  console.log("Open");
  modal.component.value = markRaw(ModalInstall);
  modal.showModal();
};

const openSettings = () => {
  console.log("Open");
  modal.component.value = markRaw(ModalSettings);
  modal.showModal();
};

const openAddAcc = () => {
  console.log("Open");
  modal.component.value = markRaw(ModalAccount);
  modal.showModal();
}


appWindow.show();
appWindow.setFocus();
</script>


<template>
  <Teleport to="#modal">
    <Transition>
      <component :is="modal.component.value" v-if="modal.show.value" @close="modal.hideModal"/>
    </Transition>
  </Teleport>
  <div class="container">
    <div id="sidebar">
      <div id="sidebar-base">
        <div id="sidebar-top">
          <ProfileButton id="AccountView" tabindex="0" @addAccount="openAddAcc"/>
          <div id="sidebar-actions">
            <!-- Add New Instance-->
            <SidebarButton class="sidebarBtn" @click="openInstall" tabindex="0">
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                <path fill-rule="evenodd" d="M 11 2 L 11 11 L 2 11 L 2 13 L 11 13 L 11 22 L 13 22 L 13 13 L 22 13 L 22 11 L 13 11 L 13 2 Z" fill="#000000" />
              </svg>
            </SidebarButton>

            <!-- Search -->
            <sidebar-button class="sidebarBtn" tabindex="0">
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 72 72">
                <path d="M 31 11 C 19.973 11 11 19.973 11 31 C 11 42.027 19.973 51 31 51 C 34.974166 51 38.672385 49.821569 41.789062 47.814453 L 54.726562 60.751953 C 56.390563 62.415953 59.088953 62.415953 60.751953 60.751953 C 62.415953 59.087953 62.415953 56.390563 60.751953 54.726562 L 47.814453 41.789062 C 49.821569 38.672385 51 34.974166 51 31 C 51 19.973 42.027 11 31 11 z M 31 19 C 37.616 19 43 24.384 43 31 C 43 37.616 37.616 43 31 43 C 24.384 43 19 37.616 19 31 C 19 24.384 24.384 19 31 19 z" fill="#000000" />
              </svg>
            </sidebar-button>
          </div>
        </div>
        <div id="sidebar-bottom">
          <!-- Settings -->
          <SidebarButton class="sidebarBtn" @click="openSettings" tabindex="0">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 50 50">
              <path d="M47.16,21.221l-5.91-0.966c-0.346-1.186-0.819-2.326-1.411-3.405l3.45-4.917c0.279-0.397,0.231-0.938-0.112-1.282 l-3.889-3.887c-0.347-0.346-0.893-0.391-1.291-0.104l-4.843,3.481c-1.089-0.602-2.239-1.08-3.432-1.427l-1.031-5.886 C28.607,2.35,28.192,2,27.706,2h-5.5c-0.49,0-0.908,0.355-0.987,0.839l-0.956,5.854c-1.2,0.345-2.352,0.818-3.437,1.412l-4.83-3.45 c-0.399-0.285-0.942-0.239-1.289,0.106L6.82,10.648c-0.343,0.343-0.391,0.883-0.112,1.28l3.399,4.863 c-0.605,1.095-1.087,2.254-1.438,3.46l-5.831,0.971c-0.482,0.08-0.836,0.498-0.836,0.986v5.5c0,0.485,0.348,0.9,0.825,0.985 l5.831,1.034c0.349,1.203,0.831,2.362,1.438,3.46l-3.441,4.813c-0.284,0.397-0.239,0.942,0.106,1.289l3.888,3.891 c0.343,0.343,0.884,0.391,1.281,0.112l4.87-3.411c1.093,0.601,2.248,1.078,3.445,1.424l0.976,5.861C21.3,47.647,21.717,48,22.206,48 h5.5c0.485,0,0.9-0.348,0.984-0.825l1.045-5.89c1.199-0.353,2.348-0.833,3.43-1.435l4.905,3.441 c0.398,0.281,0.938,0.232,1.282-0.111l3.888-3.891c0.346-0.347,0.391-0.894,0.104-1.292l-3.498-4.857 c0.593-1.08,1.064-2.222,1.407-3.408l5.918-1.039c0.479-0.084,0.827-0.5,0.827-0.985v-5.5C47.999,21.718,47.644,21.3,47.16,21.221z M25,32c-3.866,0-7-3.134-7-7c0-3.866,3.134-7,7-7s7,3.134,7,7C32,28.866,28.866,32,25,32z" fill="#000000" />
            </svg>
          </SidebarButton>
        </div>
      </div>
    </div>

    <div id="instance-manager">
      <div class="action-bar">
        <NButton class="action-btn"><img src="https://www.htmlcssbuttongenerator.com/iconExample-search-thin.svg"></NButton>
        <NButton class="action-btn" >Delete</NButton>
<!--        <NButton class="action-btn" expand use_padding>Delete</NButton>
        <NChip>Hello</NChip>
        <NChip>Hello</NChip>
        <NChip>Hello</NChip>-->
      </div>
      <InstanceViewer></InstanceViewer>
    </div>
  </div>

  <NotificationMenu></NotificationMenu>
</template>

<style scoped>
:root {
  color: var(--gray-100);
}

.container {
  margin: 0;
  display: flex;
  flex-direction: row;
  justify-content: left;
  text-align: left;
  height: 100vh;

  #sidebar {
    min-width: 60px;
    text-align: center;
    width: 85px;
    margin: 10px;

    & #AccountView {

    }

    & #sidebar-actions {
      display: flex;
      flex-direction: column;
      justify-content: space-between;
      height: 100%;
      padding: 20px 0;

      & .sidebarBtn {
        padding: 15%;
      }
    }

    & #sidebar-base {
      box-shadow: var(--gray-800) 6px 6px 4px;
      border-radius: 30px;
      background: var(--gray-400);

      & #sidebar-top {
        border-radius: 30px;
        background: var(--primary-500);
      }

      & #sidebar-bottom {
        padding-bottom: 10px;
        margin-top: 10px;

        & .sidebarBtn {
          min-width: 50px;
          padding: 16%;
        }
      }
    }
  }

  #instance-manager {
    display: block;
    width: 100%;

    & .action-bar {
      height: 40px;
      display: flex;
      padding: 20px;
      flex-direction: row-reverse;
      gap: 20px;
    }
  }
}
</style>
