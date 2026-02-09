<template>
    <div class="min-h-screen p-8">
        <div class="max-w-5xl mx-auto">
            <div class="flex justify-between items-center mb-6">
                <h1 class="text-2xl font-semibold mb-6">
                    Users
                </h1>
                <v-btn
                    variant="elevated"
                    color="primary"
                    @click="clickCreateUser"
                >
                    Add User
                </v-btn>
            </div>

        <div class="bg-white shadow overflow-hidden">
            <table class="w-full text-left">
            <thead class="bg-neutral-800">
                <tr>
                <th class="table-header-cell">Name</th>
                <th class="table-header-cell">Email</th>
                <th class="table-header-cell w-14 px-2"></th>
                </tr>
            </thead>
            <tbody>
                <ItemCard
                    v-for="manager in managers"
                    :id="manager.id"
                    :name="manager.name"
                    :email="manager.email"
                    @on-click="detailUser(manager.id)"
                    @on-delete="clickDeleteUser(manager.id)"
                />
            </tbody>
            </table>
        </div>
        </div>

        <v-dialog v-model="modal" max-width="600">
            <v-card class="p-8">
                <v-card-title>{{ dialogParams.title }}</v-card-title>
                <v-text-field
                    v-if="dialogParams.createMode"
                    label="User Name"
                    variant="outlined"
                    density="comfortable"
                    v-model="createParams.userName"
                    bg-color="neutral-700"
                    class="mb-6 text-white"
                    hide-details
                />

                <v-text-field
                    v-if="dialogParams.createMode"
                    label="Email"
                    variant="outlined"
                    density="comfortable"
                    v-model="createParams.email"
                    bg-color="neutral-700"
                    class="mb-4 text-white"
                    hide-details
                />

                <v-text-field
                    v-if="dialogParams.createMode"
                    label="Password"
                    variant="outlined"
                    density="comfortable"
                    type="password"
                    v-model="createParams.pass"
                    bg-color="neutral-700"
                    class="mb-6 text-white"
                    hide-details
                />

                <v-card-actions>
                    <v-btn @click="modal = false">Cancel</v-btn>
                    <v-btn
                        :color=dialogParams.confirmColor
                        @click="dialogParams.confirm"
                    >{{ dialogParams.confirmText }}</v-btn>
                </v-card-actions>
            </v-card>
        </v-dialog>
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { UserInfo } from '@/scripts/types';
import { ezAuthClient } from '@/scripts/apis/index';
import ItemCard from '@/components/Item.vue';
import router from '@/router';

const managers = ref<UserInfo[]>([]);
const createParams = ref<{
    userName: string;
    email: string;
    pass: string;
}>({
    userName: '',
    email: '',
    pass: '',
});
const modal = ref(false);
const dialogParams = ref<{
    createMode?: boolean;
    title: string;
    cancel: () => void;
    confirm: () => void;
    confirmText: string;
    confirmColor: string;
}>();


async function fetchUsers() {
    try {
        managers.value = await ezAuthClient.listUser();
    } catch (error) {
        console.log('error fetching managers');
    };
    console.log('managers', managers.value);
}

async function detailUser(id: string) {
    router.push(`/users/${id}`);
}

async function clickCreateUser() {
    dialogParams.value = {
        createMode: true,
        title: 'Create User',
        cancel: () => {
            modal.value = false;
        },
        confirm: async () => {
            // await ezAuthClient.createUser(...);
            modal.value = false;
            await ezAuthClient.createUser(
                createParams.value.userName,
                createParams.value.email,
                createParams.value.pass
            );
            fetchUsers();
        },
        confirmText: 'Create',
        confirmColor: 'primary',
    };
    modal.value = true;
}

async function clickDeleteUser(id: string) {
    dialogParams.value = {
        title: 'Delete User?',
        cancel: () => {
            modal.value = false;
        },
        confirm: async () => {
            await ezAuthClient.deleteUser(id);
            modal.value = false;
            fetchUsers();
        },
        confirmText: 'Delete',
        confirmColor: 'red',
    };
    modal.value = true;
}

onMounted(fetchUsers);
</script>

<style lang="css" scoped>
.table-header-cell {
  padding-left: 1.5rem;   /* px-6 */
  padding-right: 1.5rem;  /* px-6 */
  padding-top: 0.75rem;   /* py-3 */
  padding-bottom: 0.75rem;/* py-3 */
  font-size: 0.875rem;    /* text-sm */
  font-weight: 500;       /* font-medium */
  color: #d1d5db;         /* text-gray-600 */
}
</style>