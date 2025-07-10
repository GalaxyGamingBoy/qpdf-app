import { createApp } from "vue";
import App from "./App.vue";

import PrimeVue from 'primevue/config';
import ToastService from 'primevue/toastservice';
import Aura from '@primeuix/themes/aura';
import Tooltip from 'primevue/tooltip';

import routes from "./routes";

import { createRouter, createWebHistory } from "vue-router";

const app = createApp(App);

const router = createRouter({
    history: createWebHistory(),
    routes
})

app.use(router)


app.directive('tooltip', Tooltip);
app.use(ToastService)
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