/*
Thanks to Alexander Gekov for the tutorial on modals using Vue Teleport
https://dev.to/alexandergekov/creating-better-modals-using-vue-teleport-3cd4

 */

import { ref } from 'vue'

// keep track of component to render
const component = ref();
// keep track of whether to show modal
const show = ref(false);

export function useModal() {
    return {
        component,
        show,
        // methods to show/hide modal
        showModal: () => show.value = true,
        hideModal: () => show.value = false,
    }
}