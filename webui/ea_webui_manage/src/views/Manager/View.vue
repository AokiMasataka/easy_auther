<template>
    <h1>{{ id }}</h1>
</template>
<script setup lang="ts">
import { useRoute } from 'vue-router';
import router from '@/router';
const route = useRoute();

const id = route.path.split('/')[2];

function goBack() {
    router.push('/managers');
}

</script>