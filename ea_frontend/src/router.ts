import { createRouter, createWebHistory } from 'vue-router';

import Login from './views/Login/View.vue';
import Register from './views/Register/View.vue';
import Home from './views/Home/View.vue';
import Manager from './views/Manager/View.vue';
import Managers from './views/Managers/View.vue';
import Users from './views/Users/View.vue';

const routes = [
    { path: "/login", name: "Login", component: Login },
    { path: "/register", name: "Register", component: Register },
    { path: "/", redirect: "/home" },
    { path: "/home", name: "Home", component: Home },
    { path: "/managers", name: "Managers", component: Managers },
    { path: "/managers/:id", name: "Manager", component: Manager },
    { path: "/users", name: "Users", component: Users }
]

const router = createRouter({
    history: createWebHistory(),
    routes,
});

export default router;