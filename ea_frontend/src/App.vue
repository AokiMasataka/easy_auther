<template>
    <v-app>
        <div v-if="showHeader">
            <Header></Header>
            <SideBar></SideBar>
        </div>
        <v-main>
            <router-view/>
        </v-main>
    </v-app>
</template>
  
<script setup lang="ts">
import { useRoute } from "vue-router";
import Header from './components/common/Header.vue';
import SideBar from './components/common/SideBar.vue';

const route = useRoute();

function showHeader(): boolean {
    const noHeaderRoutes = ["/"]; // ヘッダーを表示しないページのパス
    return !noHeaderRoutes.includes(route.path);
}

</script>
  