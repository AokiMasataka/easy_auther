<template>
    <div class="mt-3 flex justify-center">
        <CreateBtn
            title="Create New User"
            @on-create="createUser"
        />
    </div>

    <div class="mt-3 flex justify-center">
        <div class="w-2/3 flex items-center">
            <List
                :items="users"
                :uri="uri"
                @on-delete="deleteUser"
            />
        </div>
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { useRoute } from 'vue-router';
import CreateBtn from '../components/templates/CreateBtn.vue';
import List from '../components/templates/List.vue';
import * as api from '../scripts/apis/userApi';
import { ItemInfo } from '../scripts/types';

const route = useRoute();
const groupId = route.params.groupId as string;

const users = ref<ItemInfo[]>([]);

async function fetchUsers() {
    users.value = await api.getUsers(groupId);
};

async function createUser(name: string, pass: string) {
    await api.createUser(groupId, name, pass);
    fetchUsers();
}

async function deleteUser(userId: string) {
    await api.deleteUser(groupId, userId);
    fetchUsers();
};

function uri(userId: string): string  {
    return `http://localhost:3000/${groupId}/${userId}`;
};

onMounted(fetchUsers);
</script>