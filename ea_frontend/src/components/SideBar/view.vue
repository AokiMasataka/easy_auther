<template>
    <v-navigation-drawer>
        <v-list line="two">
            <v-list-item
                class="text-center"
                title="Home"
                @click='move("/home")'
            />
            <v-list-item
                class="text-center"
                title="Managers"
                @click='move("/managers")'
            />
            <v-list-item
                class="text-center"
                title="Users"
                @click='move("/users")'
            />
            <v-list-item
                class="text-center"
                title="Settings"
                @click='move("/settings")'
            />
        </v-list>
    </v-navigation-drawer>
</template>

<script setup lang="ts">
import router from '@/router';

function move(url: string) {
    router.push(url)
}
</script>