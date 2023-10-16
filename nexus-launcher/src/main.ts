import { createApp } from "vue";
import { createPinia } from 'pinia'
import "./styles.css";
import App from "./App.vue";

document.addEventListener("keydown", function(e) {
    if (e.code == "Space" || e.code == "Enter") {
        let element = document.activeElement;
        if (element instanceof HTMLElement) {
            element.click();
        }
    }});


const app = createApp(App);
const pinia = createPinia();

app.use(pinia);
app.mount("#app");
