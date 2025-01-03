<template>
    <div class="mt-3 flex justify-center">
        <CreateBtn
            title="Create New User"
            @on-create="createUser"
        />
    </div>

    <div class="mt-3 flex justify-center">
        <div class="w-2/3 flex items-center">
            <UserTable
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
import CreateBtn from '../components/CreateBtn';
import UserTable from '../components/UserTable';
import { userApi } from '../scripts/apis';
import { ItemInfo } from '../scripts/types';

const route = useRoute();
const groupId = route.params.groupId as string;

const users = ref<ItemInfo[]>([]);

async function fetchUsers() {
    users.value = await userApi.getUsers(groupId);
};

async function createUser(name: string, pass: string) {
    await userApi.createUser(groupId, name, pass);
    fetchUsers();
}

async function deleteUser(userId: string) {
    await userApi.deleteUser(groupId, userId);
    fetchUsers();
};

function uri(userId: string): string  {
    return `http://localhost:3000/${groupId}/${userId}`;
};

onMounted(fetchUsers);
</script>