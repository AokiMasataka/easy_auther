<template>
  <div class="page">
    <div class="login-card">
      <h1>Sign in</h1>

      <form @submit.prevent="onSubmit">
        <div class="field">
          <label for="email">Email</label>
          <input
            id="email"
            v-model="email"
            placeholder="you@example.com"
          />
        </div>

        <div class="field">
          <label for="password">Password</label>
          <input
            id="password"
            type="password"
            v-model="password"
            placeholder="••••••••"
          />
        </div>

        <button type="submit">Login</button>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { authorizeApi } from "../scripts/authorize";

const params = new URLSearchParams(window.location.search);

const email = ref("");
const password = ref("");

const onSubmit = async () => {
    if (!email.value || !password.value) {
        alert("Please fill in all fields.");
        return;
    }

    const client_id = params.get("client_id");
    const redirect_uri = params.get("redirect_uri");
    const code_challenge = params.get("code_challenge");
    const state = params.get("state");

    if (
            client_id === null ||
            redirect_uri === null ||
            code_challenge === null ||
            state === null
        ) {
        alert("Missing required OAuth parameters.");
        return;
    }

    const authorize_code = await authorizeApi(
        email.value,
        password.value,
        client_id,
        redirect_uri,
        code_challenge,
    );

    console.log("authorize_code:", authorize_code);

    if (authorize_code) {
        const redirectUrl = new URL(redirect_uri);
        redirectUrl.searchParams.append("code", authorize_code);
        redirectUrl.searchParams.append("state", state);
        window.location.href = redirectUrl.toString();
    } else {
        alert("Authorization failed. Please check your credentials.");
    }
};
</script>

<style scoped>
/* 背景 */
.page {
  min-height: 100vh;
  display: grid;
  place-items: center;
  background: #202123;
  color: #e5e7eb;
  font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI",
    sans-serif;
}

/* カード */
.login-card {
  width: 100%;
  max-width: 360px;
  padding: 32px;
  background: #2a2b2e;
  border-radius: 12px;
  border: 1px solid #3a3b3e;
}

/* タイトル */
h1 {
  text-align: center;
  margin-bottom: 28px;
  font-size: 1.6rem;
  font-weight: 600;
}

/* 入力欄 */
.field {
  display: flex;
  flex-direction: column;
  margin-bottom: 18px;
}

label {
  font-size: 0.8rem;
  color: #b5b7bd;
  margin-bottom: 6px;
}

input {
  padding: 10px 12px;
  border-radius: 8px;
  background: #1f2022;
  border: 1px solid #3a3b3e;
  color: #e5e7eb;
  font-size: 0.95rem;
  outline: none;
}

input::placeholder {
  color: #7c7f86;
}

input:focus {
  border-color: #6b7280;
  background: #1c1d1f;
}

/* ボタン */
button {
  margin-top: 12px;
  width: 100%;
  padding: 12px;
  border-radius: 8px;
  border: 1px solid #3a3b3e;
  background: #8934ba;
  color: #e5e7eb;
  font-size: 0.95rem;
  font-weight: 500;
  cursor: pointer;
}

button:active {
  background: #2a2b2e;
}
</style>