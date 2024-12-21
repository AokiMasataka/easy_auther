<template>
    <div class="mt-3 flex justify-center">
        <CreateBtn
            @click="createUser"
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
import { userApi } from '../scripts/api';
import { UserInfo } from '../scripts/types';
import CreateBtn from '../components/templates/CreateBtn.vue';
import List from '../components/templates/List.vue';

const route = useRoute();
const groupId = route.params.groupId as string;

const users = ref<UserInfo[]>([{name: "", id: ""}]);

async function fetchUsers() {
    users.value = await userApi.getUsers(groupId);
    console.log("fetch user");
};

async function createUser(name: string, pass: string) {
    userApi.createUser(groupId, name, pass);
    fetchUsers();
}

async function deleteUser(userId: string) {
    await userApi.deleteUser(groupId, userId);
    fetchUsers();
};

function uri(userId: string): string  {
    return `http://localhost:3000/${groupId}/${userId}`;
};

onMounted(fetchUsers());
</script>