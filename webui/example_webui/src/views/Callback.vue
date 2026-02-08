<template>
    <h1>Callback Page</h1>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { callback } from '../scripts/ea_client/mod.ts';

async function handleCallback() {
    try {
        const token = await callback("http://localhost:8000");
        window.location.href = "/home";
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