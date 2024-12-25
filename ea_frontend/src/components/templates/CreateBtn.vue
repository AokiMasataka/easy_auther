<template>
    <v-btn
        @click="formOpen"
    >{{ props.title }}</v-btn>

    <v-dialog v-model="isAvtiveDialog" max-width="600">
        <CreateForm
            :title="props.title"
            v-model="createData"
            @on-create="createUser"
            @on-close="formClose"
        />
    </v-dialog>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { AuthInfo } from '../../scripts/types';
import CreateForm from '../common/CreateForm.vue';

const isAvtiveDialog = ref<boolean>(false);
const createData = ref<AuthInfo>({name: "", pass: ""});
const props = defineProps<{title: string}>();
const emit = defineEmits<{(e: 'onCreate', name: string, pass: string): void}>();

async function createUser() {
    await emit(
        'onCreate',
        createData.value.name,
        createData.value.pass
    );
    formClose();
}

function formOpen() { isAvtiveDialog.value = true; };
function formClose() { isAvtiveDialog.value = false; };

</script>
