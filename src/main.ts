import { createApp } from "vue";
import App from "./App.vue";

import PrimeVue from 'primevue/config';
import Aura from '@primeuix/themes/aura';

import routes from "./routes";
import { createMemoryHistory, createRouter } from "vue-router";

const app = createApp(App);

const router = createRouter({
    history: createMemoryHistory(),
    routes
})

app.use(router)
app.use(PrimeVue, {
    theme: {
        preset: Aura,
        options: {
            darkModeSelector: false
        }
    }
})

app.mount("#app");

import "primeicons/primeicons.css";
import "./assets/css/main.css";