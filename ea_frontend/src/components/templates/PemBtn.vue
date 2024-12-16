<template>
    <v-btn @click="isActivatePemForm = true">View Public Key</v-btn>

    <PemForm
        v-model="isActivatePemForm"
        :pem="publicPem.pem"
    />
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { groupApi } from '@/scripts/api';
import PemForm from '../common/PemForm.vue';

const isActivatePemForm = ref<boolean>(false);

const props = defineProps<{groupId: string}>();
let publicPem = {pem: ""};

async function f() {
    publicPem = await groupApi.getPublicPem(props.groupId);
}

onMounted(f)
</script>