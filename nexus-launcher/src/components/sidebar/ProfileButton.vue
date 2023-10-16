<script setup lang="ts">

const props = defineProps({
  pfpImage: {
    type: String,
    default: "/ProfileIconPlaceholder.png",
  },
});

function get_pfp_image() {
  if (props.pfpImage == "") {
    return "/ProfileIconPlaceholder.png";
  } else {
    return props.pfpImage;
  }
}
</script>

<template>
  <div ref="pfpBtn" v-bind:style="{ '--pfp-image': 'url(' + get_pfp_image() + ')' }">

  </div>
</template>

<style scoped>
:root {
  --pfp-image: "/ProfileIconPlaceholder.png";
}

div {
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

div:is(:hover) {
  background-image: linear-gradient(rgba(75, 30, 30, 0.3), rgba(91, 134, 164, 0.3)), var(--pfp-image);
}
</style>