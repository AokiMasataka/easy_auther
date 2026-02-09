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
                    @click="addManager"
                >
                    Add Manager
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
                    @on-click="detailManager(manager.id)"
                    @on-delete="deleteManager(manager.id)"
                />
            </tbody>
            </table>
        </div>
        </div>

        <v-dialog v-model="modal" max-width="600">
            <v-card>
                <v-card-title>Delete Manager?</v-card-title>
                <v-card-actions>
                    <v-btn @click="modal = false">Cancel</v-btn>
                    <v-btn color="red" @click="modal = false">Delete</v-btn>
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
const modal = ref(false);

async function fetchManagers() {
    try {
        managers.value = await ezAuthClient.listManager();
    } catch (error) {
        console.log('error fetching managers');
    };
    console.log('managers', managers.value);
}

async function detailManager(id: string) {
    router.push(`/managers/${id}`);
}

async function deleteManager(id: string)  {
    console.log('delete manager', id);
    modal.value = true;
}

async function addManager() {
    router.push('/managers/new')
}

onMounted(fetchManagers);
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