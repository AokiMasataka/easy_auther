import './style.css'
import { createApp } from 'vue'
import App from './App.vue'
import { router } from "./router.ts";

// createApp(App).mount('#app')
createApp(App).use(router).mount("#app");