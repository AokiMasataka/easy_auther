import { createRouter, createWebHistory } from "vue-router";

import Sample from "./views/Login.vue";
import Callback from "./views/Callback.vue";
import Home from "./views/Home.vue";

export const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: "/", component: Sample },
    { path: "/callback", component: Callback },
    { path: "/home", component: Home },
  ],
});