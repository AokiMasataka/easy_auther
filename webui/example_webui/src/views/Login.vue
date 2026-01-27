<template>
    <h1>Login page</h1>
    <button @click="onLoginButtom">login</button>
</template>

<script setup lang="ts">
import { redirectLoginForm } from '../scripts/ea_client/mod.ts';

async function onLoginButtom() {
    const eaLoginFormUrl = "http://localhost:3000/authorize";
    const client_id = "1111";
    await redirectLoginForm(eaLoginFormUrl, client_id);
}

</script>