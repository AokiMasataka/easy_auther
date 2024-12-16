<template>
    <v-btn
        @click="isActivateForm = true"
    >{{ props.title }}</v-btn>

    <v-dialog v-model="isActivateForm" max-width="600">
        <CreateForm
            :title="props.title"
            v-model="CreateParamsModel"
            @on-close="closeForm"
            @on-create="createItem"
        />
    </v-dialog>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { AuthInfo } from '@/scripts/types';
import CreateForm from '../common/CreateForm.vue';

const props = defineProps<{title: string}>();
const CreateParamsModel = defineModel<AuthInfo>({ required: true });
const emit = defineEmits<{(e: 'onCreate'): void}>();

const isActivateForm = ref<boolean>(false);

function createItem() {
    emit('onCreate');
    isActivateForm.value = false;
}

function closeForm(){
    isActivateForm.value = false;
};
</script>