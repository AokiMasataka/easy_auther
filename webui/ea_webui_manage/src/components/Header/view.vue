<template>
    <v-app-bar color="primary" app>
        <v-app-bar-title>Ez Auth</v-app-bar-title>
    </v-app-bar>
</template>

<script setup lang="ts">
import router from '@/router';
import { clearTokens } from '@/scripts/cookie';

function logout() {
    clearTokens();
    router.push("/login");
}
</script>