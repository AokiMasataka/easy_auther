<template>
</template>

<script setup lang="ts">
import { useRoute } from 'vue-router';

const route = useRoute();
const groupId = route.params.groupId as string;
</script>