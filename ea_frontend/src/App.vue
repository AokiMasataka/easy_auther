<template>
    <v-app>
        <div v-if="showHeader()">
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
import Header from './components/Header';
import SideBar from './components/SideBar';

const route = useRoute();

function showHeader(): boolean {
    const noHeaderRoutes = ["/login", "/create"]; // ヘッダーを表示しないページのパス
    return !noHeaderRoutes.includes(route.path);
}

</script>
  