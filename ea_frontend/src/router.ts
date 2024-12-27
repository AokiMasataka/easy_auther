import { createRouter, createWebHistory } from 'vue-router';
import CreateGroup from './views/CreateGroup.vue';
import GroupDetail from './views/GroupDetail.vue';
import GroupUsers from './views/GroupUsers.vue';
import Login from './views/Login.vue';

const routes = [
    { path: '/', redirect: '/login'},
    { path: '/login', name: 'Login', component: Login},
    { path: '/create', name: 'Create', component: CreateGroup},
    { path: '/:groupId/detail', name: 'GroupDetail', component: GroupDetail },
    { path: '/:groupId/users', name: 'GroupUsers', component: GroupUsers }
]

const router = createRouter({
    history: createWebHistory(),
    routes,
});

export default router;