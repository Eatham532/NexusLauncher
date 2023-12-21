import {defineStore} from "pinia";
import {getVersions} from "../scripts/rust/instances";

let x = await getVersions();

export const usePistonMeta = defineStore('piston-meta', {
    state: () => {
        return x;
    }
})