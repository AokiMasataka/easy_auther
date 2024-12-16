<template>
    <div class="flex justify-center">
        <div class="w-2/3 flex items-center justify-between mt-16">
            <h1 class="text-2xl">Groups</h1>
            <CreateBtn
                v-model="createGroupProps"
                title="Create New Group"
                @on-create="createGroup"
            />
        </div>
    </div>

    <div class="mt-3 flex justify-center">
        <div class="w-2/3 flex items-center">
            <List
                :items="groups"
                :uri="uri"
                @on-delete="deleteGroup"
            />
        </div>
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { useRouter } from 'vue-router'
import { GroupInfo, AuthInfo } from '@/scripts/types';
import { groupApi } from '@/scripts/api';
import CreateBtn from '../templates/CreateBtn.vue';
import List from '../templates/List.vue';

const router = useRouter();
const groups = ref<GroupInfo[]>([{name: "", id: ""}]);
const createGroupProps = ref<AuthInfo>({name: "", pass: ""});

async function fetchGroups() {
    groups.value = await groupApi.getGroups();
};

async function createGroup() {
    const res = await groupApi.createGroup(
        createGroupProps.value.name,
        createGroupProps.value.pass
    );
    router.push(`${res.id}`);
}

async function deleteGroup(groupId: string) {
    await groupApi.deleteGroup(groupId);
    fetchGroups();
}

function uri(id: string): string  {
    return `http://localhost:3000/${id}`;
}

onMounted(fetchGroups);
</script>
