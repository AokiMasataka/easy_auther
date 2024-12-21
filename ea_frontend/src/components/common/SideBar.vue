<template>
    <v-navigation-drawer>
        <v-list line="two">
            <v-list-item
                title="User list"
                :href='url("users")'
            />
            <v-list-item
                title="Group Detail"
                :href='url("detail")'
            />
        </v-list>
    </v-navigation-drawer>
</template>

<script setup lang="ts">
import { useRoute } from 'vue-router';

const route = useRoute();
const groupId = route.params.groupId as string;

function url(prefix: string) {
    return `http://localhost:3000/${groupId}/${prefix}`;
}
</script>