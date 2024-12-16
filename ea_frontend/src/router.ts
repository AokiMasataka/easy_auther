import { createRouter, createWebHistory } from 'vue-router';
import Group from './components/pages/Group.vue'
import Groups from './components/pages/Groups.vue';

const routes = [
    { path: '/', name: 'Groups', component: Groups },
    { path: '/:groupId', name: 'Group', component: Group }
]

const router = createRouter({
    history: createWebHistory(),
    routes,
});

export default router;