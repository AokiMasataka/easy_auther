<template>
    <h1>Callback Page</h1>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { callback } from '../scripts/auth';

async function handleCallback() {
    try {
        const token = await callback();
        sessionStorage.setItem("access_token", token);
        window.location.href = "/";
    } catch (e) {
        alert("code verifier not found");
        return;
    }
}

console.log("setup executed");
onMounted(async () => {
    await handleCallback();
});
</script>