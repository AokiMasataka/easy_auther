<template>
    <v-card class="m-4" :href="props.href">
        <div class="flex items-center justify-between">
            <v-card-title>{{ props.name }}</v-card-title>
            <!-- <DeleteBtn class="mx-4"/> -->
        </div>
        <v-card-text>{{props.id}}</v-card-text>
        <!-- <v-card-actions></v-card-actions> -->
    </v-card>

</template>

<script setup lang="ts">



const props = defineProps<{
    name: string,
    id: string,
    href: string,
}>();

</script>