<template>
  <div class="max-w-md mx-auto mt-20 p-8 bg-white shadow-lg rounded-lg">
    <h2 class="text-2xl font-bold text-center text-blue-600 mb-6">Forgot Password</h2>
    <form @submit.prevent="handleReset" class="space-y-4">
      <div>
        <label class="block text-sm font-medium text-gray-700">Email</label>
        <input v-model="email" type="email" class="mt-1 w-full px-4 py-2 border rounded" required />
      </div>
      <button type="submit" class="w-full bg-indigo-600 text-white py-2 rounded hover:bg-indigo-700">
        Send Reset Link
      </button>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";

const email = ref("");

const handleReset = () => {
  alert(`Reset link sent to ${email.value} (mocked)`);
};
</script>
