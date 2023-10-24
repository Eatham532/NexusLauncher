<script setup lang="ts">
import {ref, Ref} from "vue";
import {getInstancesToml, NexusInstance} from "../../scripts/rust/instances.ts";
import SkeletonBasic from "../skeleton/SkeletonBasic.vue";
import InstanceCard from "./InstanceCard.vue";

  let instances: Ref<NexusInstance[]> = ref([]);
  getInstancesToml().then((toml) => {
    instances.value = toml.Instance;
    console.log(instances.value);
  });


</script>

<template>
  <Suspense>
    <template #default>
      <div tabindex="0">
        <instance-card class="card" v-for="instance in instances" :instance="instance"/>
        <div v-if="!instances.length" style="width: 100%; height: 100%; display: flex; flex-direction: column; text-align: center">
          <h2>THERE ARE NO INSTANCES INSTALLED!</h2>
          <P>&lt- Please install an instance by clicking the + button</P>
        </div>
      </div>
    </template>
    <template #fallback>
      <div>
        <SkeletonBasic class="skeleton"/>
        <SkeletonBasic class="skeleton"/>
        <SkeletonBasic class="skeleton"/>
      </div>
    </template>
  </Suspense>
</template>

<style scoped>
div {
  overflow-y: auto;
  height: calc(100vh - 80px);

  & .card {
    display: inline-block;
    height: 200px;
    width: 200px;
  }
}
</style>