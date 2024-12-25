<template>
    <v-icon
        size="small"
        @click="isActivateForm = true"
    >mdi-delete</v-icon>

    <v-dialog v-model="isActivateForm" max-width="600">
        <DeleteForm
            :title="props.title"
            :name="props.name"
            :id="props.id"
            @on-close="onClose"
            @on-delete="onDelete"
        />
    </v-dialog>

</template>

<script setup lang="ts">
import { ref } from 'vue';
import DeleteForm from '../common/DeleteForm.vue';

const props = defineProps<{
    title: string,
    name: string,
    id: string
}>()

const emit = defineEmits<{
    (e: 'onDelete'): void
}>();

const isActivateForm = ref<boolean>(false);

function onDelete() {
    emit('onDelete');
    isActivateForm.value = false;
}

function onClose() {
    isActivateForm.value = false;
};

</script>