<template>
  <div class="register-container">
    <h2 class="register-title">Créer un compte</h2>

    <form @submit.prevent="handleRegister" class="register-form">
      <div>
        <label class="form-label">Pseudo</label>
        <input v-model="pseudo" type="text" class="form-input" required />
      </div>

      <div>
        <label class="form-label">Adresse email</label>
        <input v-model="email" type="email" class="form-input" required />
      </div>

      <div>
        <label class="form-label">Mot de passe</label>
        <input v-model="password" type="password" class="form-input" required />
      </div>

      <button type="submit" class="submit-button">S'inscrire</button>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'

const pseudo = ref('')
const email = ref('')
const password = ref('')
const router = useRouter()

const handleRegister = async () => {
  alert('Compte créé (mock)')
  router.push('/login')
}
</script>

<style scoped>
.register-container {
  @apply max-w-md mx-auto mt-20 p-8 bg-white shadow-lg rounded-2xl;
}

.register-title {
  @apply text-3xl font-bold text-center text-violet-700 mb-6;
}

.register-form {
  @apply space-y-5;
}

.form-label {
  @apply block text-sm font-medium text-gray-700;
}

.form-input {
  @apply mt-1 w-full px-4 py-2 border border-gray-300 rounded-[20px] focus:outline-none focus:ring-2 focus:ring-violet-500;
}

.submit-button {
  @apply w-full bg-violet-600 hover:bg-violet-700 text-white font-semibold py-2 rounded-[20px] transition shadow-md;
}
</style>
