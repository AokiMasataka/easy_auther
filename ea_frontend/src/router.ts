import { createRouter, createWebHistory } from 'vue-router';
import CreateGroup from './components/pages/CreateGroup.vue';
import GroupDetail from './components/pages/Group/GroupDetail.vue';
import GroupUsers from './components/pages/Group/GroupUsers.vue';
import Login from './components/pages/Login.vue';

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