<template>
    <v-app>
      <v-main>
        <Header></Header>
        <SideBar></SideBar>
        <router-view/>
      </v-main>
    </v-app>
  </template>
  
  <script setup lang="ts">
  import Header from './components/templates/Header.vue';
  import SideBar from './components/templates/SideBar.vue';
  </script>
  