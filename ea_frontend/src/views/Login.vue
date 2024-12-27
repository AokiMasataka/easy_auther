<template>
    <div class="h-1/4"></div>

    <div class="flex justify-center">
        <v-card
            width="800"
        >
            <v-card-title class="mx-12 mt-4">Login</v-card-title>
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
                <v-btn @click="onLogin">Login</v-btn>
                <v-btn class="ml-4" @click="toCreatePage">Create Group</v-btn>
            </v-card-actions>
        </v-card>
    </div>
    
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { groupApi } from '../scripts/apis';
import { setRefreshToken, setToken } from '../scripts/cookie';
import { AuthInfo } from '../scripts/types';

const authData = ref<AuthInfo>({name: "", pass: ""});


function toCreatePage() {
    window.location.href = "http://localhost:3000/create";
}


async function onLogin() {
    const respone = await groupApi.login(
        authData.value.name,
        authData.value.pass
    );

    setToken(respone.token);
    setRefreshToken(respone.refresh_token);
    window.location.href = `http://localhost:3000/${respone.id}/users`;
}

</script>