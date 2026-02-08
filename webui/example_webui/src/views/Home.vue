<template>
    <h1>Home Page</h1>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { refreshApi } from '../scripts/ea_client/mod.ts';


const jwt = ref<string | null>(null);


onMounted(async () => {
    jwt.value = await refreshApi("http://localhost:8000");
});

</script>