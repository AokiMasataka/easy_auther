<template>
    
    <div class="flex justify-center">
        <div class="w-2/3 flex items-center justify-between mt-16">
            <h1 class="text-2xl">Users</h1>
            <PemBtn
                :group-id="groupId"
            />

            <CreateBtn
                title="Create New User"
                v-model="createUserProps"
                @on-create="createUser"
            />
        </div>
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
import { useRoute } from 'vue-router'
import { UserInfo, AuthInfo } from '@/scripts/types';
import { userApi } from '@/scripts/api';
import List from '../templates/List.vue';
import CreateBtn from '../templates/CreateBtn.vue';
import PemBtn from '../templates/PemBtn.vue';

const route = useRoute();
const groupId = route.params.groupId as string;

const users = ref<UserInfo[]>([{name: "", id: ""}]);
const createUserProps = ref<AuthInfo>({name: "", pass: ""});

async function fetchUsers() {
    users.value = await userApi.getUsers(groupId);
};

async function createUser() {
    await userApi.createUser(
        groupId,
        createUserProps.value.name,
        createUserProps.value.pass
    );
    fetchUsers();
};

async function deleteUser(userId: string) {
    await userApi.deleteUser(groupId, userId);
    fetchUsers();
};

function uri(userId: string): string  {
    return `http://localhost:3000/${groupId}/${userId}`;
};

onMounted(fetchUsers);
</script>
