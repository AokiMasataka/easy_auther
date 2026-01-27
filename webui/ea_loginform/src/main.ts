import './style.css'
import { createApp } from 'vue'
import { router } from "./router.ts";
import Login from "./views/Login.vue";

createApp(Login).use(router).mount("#app");