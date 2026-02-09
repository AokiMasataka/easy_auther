<template>
  <div class="min-h-screen flex items-center justify-center bg-neutral-900">

    <v-card
      class="p-8 w-[380px] rounded-2xl shadow-xl"
      color="neutral-800"
      elevation="6"
    >
      <h2 class="text-3xl font-bold text-center text-white mb-8 tracking-wide">
        Welcome
      </h2>

      <!-- Email -->
      <v-text-field
        label="Email"
        variant="outlined"
        density="comfortable"
        v-model="email"
        bg-color="neutral-700"
        class="mb-4 text-white"
        hide-details
      />

      <!-- Password -->
      <v-text-field
        label="Password"
        variant="outlined"
        density="comfortable"
        type="password"
        v-model="pass"
        bg-color="neutral-700"
        class="mb-6 text-white"
        hide-details
      />

      <!-- Login Button -->
      <v-btn
        color="primary"
        block
        class="py-3 text-lg font-semibold mb-4"
        @click="login"
      >
        Login
      </v-btn>

      <!-- Create Account Button -->
      <v-btn
        variant="outlined"
        block
        class="py-3 text-lg font-semibold border-neutral-500 text-neutral-300 hover:bg-neutral-700"
        @click="goToRegister"
      >
        Register Account
      </v-btn>

    </v-card>
  </div>
</template>

<script setup lang="ts">
import {ref} from 'vue';
import router from '@/router';
import { ezAuthClient } from '@/scripts/apis/index';

const email = ref('');
const pass = ref('');


function login() {
    console.log('login', email.value, pass.value);   
    // const response = loginManager(email.value, pass.value);
    ezAuthClient.login(email.value, pass.value).then(() => {
        console.log('login success');
        router.push('/managers');
    }).catch((error) => {
        console.error('login failed', error);
    });
}

function goToRegister() {
    router.push('/register');
}
</script>

<style scoped>
</style>