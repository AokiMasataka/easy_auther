<template>
    <v-btn
        @click="formOpne"
    >Create New User</v-btn>

    <v-dialog v-model="isAvtiveDialog" max-width="600">
        <v-card
            title="Create Form"
        >
            <div class="mx-8">
                <v-text-field
                    v-model="createData.name"
                    class="mt-4"
                    label="name"
                ></v-text-field>

                <v-text-field
                    v-model="createData.pass"
                    label="password"
                ></v-text-field>
            </div>
            <v-card-actions>
                <v-btn
                    color="promary"
                    @click="onCreate"
                >Create</v-btn>
                <v-btn
                    @click="formClose"
                >Cansel</v-btn>
            </v-card-actions>
        </v-card>
    </v-dialog>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { AuthInfo } from '../../scripts/types';

const isAvtiveDialog = ref<boolean>(false);
const createData = ref<AuthInfo>({name: "", pass: ""});


const emit = defineEmits<{
    (e: 'onCreate', name: string, pass: string): void,
}>();

function onCreate() {
    console.log(createData.value.name);
    emit(
        'onCreate',
        createData.value.name,
        createData.value.pass
    );

    formClose();
}

function formOpne() {
    isAvtiveDialog.value = true;
}

function formClose() {
    isAvtiveDialog.value = false;
}

</script>