<template>
  <div class="min-h-screen flex items-center justify-center bg-neutral-900">

    <v-card
      class="p-8 w-[380px] rounded-2xl shadow-xl"
      color="neutral-800"
      elevation="6"
    >
      <h2 class="text-3xl font-bold text-center text-white mb-8 tracking-wide">
        Register Account
      </h2>

      <v-text-field
        label="User Name"
        variant="outlined"
        density="comfortable"
        v-model="userName"
        bg-color="neutral-700"
        class="mb-6 text-white"
        hide-details
      />

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
        @click="register"
      >
        Register
      </v-btn>

      <v-btn
        variant="outlined"
        block
        class="py-3 text-lg font-semibold border-neutral-500 text-neutral-300 hover:bg-neutral-700"
        @click="goToLogin"
      >
        Login
      </v-btn>

    </v-card>
  </div>
</template>

<script setup lang="ts">
import {ref} from 'vue';
import router from '@/router';
import { ezAuthClient } from '@/scripts/apis/index';

const userName = ref('');
const email = ref('');
const pass = ref('');


async function register() {
    try {
        await ezAuthClient.register(
            userName.value,
            email.value,
            pass.value
        );
        await ezAuthClient.login(email.value, pass.value);
        router.push('/');
    } catch (e) {
        console.error('register error', e);
        return;
    }
}


function goToLogin() {
    router.push('/login');
}
</script>