<script setup lang="ts">

import {Ref, ref} from "vue";
import {listen} from "@tauri-apps/api/event";
import {changeActiveUser, getPfpPath, getUsers, logoutUser, UsersJson} from "../../scripts/rust/user.ts";
import {convertFileSrc} from "@tauri-apps/api/tauri";
import NButton from "../common/NButton.vue";

let emit = defineEmits(['addAccount'])

const menuRef: Ref<HTMLDivElement | null> = ref(null);
const usersRef: Ref<UsersJson | null> = ref(null)

const openAcc = () => {
  menu_open.value = false;
  console.log("addAccount");
  emit('addAccount');
}


let pfpImage: Ref<Map<string | null, string>> = ref(new Map<string | null, string>());
async function reload_component() {
  let users = await getUsers();
  usersRef.value = users;

  pfpImage.value.set(null, "/ProfileIconPlaceholder.png")

  for (let i in users.users) {
    if (!pfpImage.value.has(users.users[i].uuid)) {
      pfpImage.value.set(users.users[i].uuid, await get_icon_path(users.users[i].uuid, false));
    }
  }

  console.log(pfpImage.value);
}

async function get_icon_path(uuid: string, redownload: boolean): Promise<string> {
  let default_path = "/ProfileIconPlaceholder.png"

  let path = await getPfpPath(uuid, redownload);

  if (path == default_path) {
    return default_path;
  } else {
    let p = convertFileSrc(path);
    console.log(p);
    return p;
  }
}

type AuthPayload = { stage: AuthStage }
type AuthStage = "Start" | { AuthCode: { code: string; url: string } } | "Cancelled" | "Complete";


listen<AuthPayload>('auth_login', (event) => {
  if (typeof event.payload.stage === 'string') {
    switch (event.payload.stage) {
      case "Complete":
        reload_component(true)
        break;
    }
  }
})

const menu_open = ref(false);
const handlePfpClick = () => {
  menu_open.value = !menu_open.value;
}

const handlePfpBlur = (event: any) => {
  if (!event.currentTarget.contains(event.relatedTarget)) {
    // Perform blur actions
    menu_open.value = false;
  }
}

const change_acc = (uuid: string) => {
  changeActiveUser(uuid);
  if (usersRef.value) {
    usersRef.value.active = uuid;
  }
}

async function logout(uuid: string) {
  await logoutUser(uuid);
  let users = await getUsers();
  usersRef.value = users;

  // TODO: Fancy active user switching
  // TODO: Switch active user on logout

  await reload_component();

  if (users.users[0]) {
    change_acc(users.users[0].uuid);
  }

}

reload_component();
</script>

<template>
  <div :class="{
    'pfp': true,
    'pfp-active': menu_open,
  }" ref="pfpBtn" v-bind:style="{ '--pfp-image': 'url(' + (pfpImage.get(usersRef?.active ?? null)?? '/ProfileIconPlaceholder.png') + ')' }" @click.self="handlePfpClick" @blur="handlePfpBlur">
    <div class="menu-wrapper" v-if="menu_open" @click.self="menu_open=false">
      <div ref="menuRef" class="menu">
        <div class="account-select-wrapper">
          <div v-for="user in usersRef?.users" :class="{account:true, account_active: usersRef?.active === user.uuid}" @click="change_acc(user.uuid)">
            <div class="pfp-mini" style="width: 30px" v-bind:style="{ '--pfp-image': 'url(' + pfpImage.get(user.uuid) + ')' }">

            </div>
            <div style="display: flex; justify-content: space-between; width: 100%;">
              {{ user.username }}
              <NButton @click="logout(user.uuid)">
                <svg xmlns="http://www.w3.org/2000/svg" width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-log-out">
                  <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"/>
                  <polyline points="16 17 21 12 16 7"/>
                  <line x1="21" y1="12" x2="9" y2="12"/>
                </svg>
              </NButton>
            </div>
          </div>
        </div>
        <NButton use_padding @click="openAcc">Add Account</NButton>
      </div>
    </div>
  </div>

</template>

<style scoped>
:root {
  position: absolute;
  --pfp-image: "/ProfileIconPlaceholder.png";
  display: grid;
}

.pfp {
  border: black 2px solid;
  background-size: cover;
  background-image: linear-gradient(rgba(0, 0, 0, 0.3), rgba(0, 0, 0, 0.3)), var(--pfp-image);
  aspect-ratio: 1;
  border-radius: 28px;
  padding: 0;
  overflow: visible;
  margin:0;
  cursor: pointer;
}

.pfp:hover {
  background-image: linear-gradient(rgba(203, 169, 169, 0.11), rgba(40, 54, 65, 0.42)), var(--pfp-image);
}

.pfp-active {
  background-image: linear-gradient(rgba(203, 169, 169, 0.11), rgba(40, 54, 65, 0.42)), var(--pfp-image);
  transform: scale(1.05);
  transition: all .5s;
}

.menu-wrapper {
  width: 200px;
  height: 200px;
  padding: 0 0 0 30px;
  position: absolute;
  top: 0;
  left: 80%;

  & .menu {
    color: black;
    background-color: var(--gray-200);
    box-shadow: rgba(54, 50, 50, 0.64) 2px 2px 2px;
    width: 130%;
    border-radius: 0.5rem;
    padding: 10px;
    display: flex;
    flex-direction: column;
    gap: 10px;

    & .account-select-wrapper {
      max-height: 20vh;
      overflow-y: auto;
      border-radius: 0.5rem;
    }
  }
}

.account {
  display: flex;
  gap: 10px;
  padding: 10px;

  &:hover {
    background-color: var(--gray-400);
    box-shadow: var(--gray-500) 2px 2px 2px;
  }

  & .pfp-mini {
    background-size: cover;
    border: black 2px solid;
    margin: 0;
    background-image: linear-gradient(rgba(203, 169, 169, 0.11), rgba(40, 54, 65, 0.42)), var(--pfp-image);
    aspect-ratio: 1;
    border-radius: 10px;
  }
}

.account_active {
  background-color: var(--gray-500)!important;
  box-shadow: var(--gray-500) 2px 2px 2px;
}
</style>