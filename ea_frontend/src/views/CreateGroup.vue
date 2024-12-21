<template>
   <div class="h-1/4"></div>

    <div class="flex justify-center">
        <v-card
            width="800"
        >
            <v-card-title class="mx-12 mt-4">Create New Group</v-card-title>
            <div class="mx-16">
                <v-text-field
                    v-model="authData.name"
                    class="mt-4"
                    label="group name"
                ></v-text-field>

                <v-text-field
                    v-model="authData.pass"
                    label="password"
                ></v-text-field>
            </div>
            
            <v-card-actions class="mx-12">
                <v-btn @click="createGroup">Create</v-btn>
            </v-card-actions>
        </v-card>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { groupApi } from '../scripts/api';
import { AuthInfo } from '../scripts/types';

const authData = ref<AuthInfo>({name: "", pass: ""});


async function createGroup() {
    const idAndPem = await groupApi.createGroup(
        authData.value.name,
        authData.value.pass
    );

    window.location.href = `http://localhost:3000/${idAndPem.id}/users`;
}

</script>