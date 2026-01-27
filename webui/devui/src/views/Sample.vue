<template>
    <h1>Login page</h1>
    <button @click="onLoginButtom">login</button>
</template>

<script setup lang="ts">
import { redirectLoginForm } from '../scripts/auth';


async function onLoginButtom() {
    await redirectLoginForm("1111");
}

</script>