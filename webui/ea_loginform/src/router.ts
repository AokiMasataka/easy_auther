import { createRouter, createWebHistory } from "vue-router";

import Login from "./views/Login.vue";

export const router = createRouter({
    history: createWebHistory(),
    routes: [
        { path : "/", redirect: "/authorize" },
        { path: "/authorize", component: Login },
    ],
});