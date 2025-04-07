<template>
  <div class="max-w-md mx-auto mt-20 bg-white card dark:bg-violet-950">

    <Head>
      <title>Connexion</title>
    </Head>

    <h2 class="text-center title-primary dark:text-violet-200">Connexion</h2>

    <form @submit.prevent="handleLogin" class="space-y-5">
      <div>
        <label class="label-primary dark:text-violet-200">Adresse email</label>
        <input v-model="email" type="email"
          class="input-primary dark:bg-violet-900 dark:text-white dark:border-violet-800" required />
      </div>

      <div>
        <label class="label-primary dark:text-violet-200">Mot de passe</label>
        <input v-model="password" type="password"
          class="input-primary dark:bg-violet-900 dark:text-white dark:border-violet-800" required />
      </div>

      <button type="submit" class="w-full btn-primary">
        Se connecter
      </button>

      <p class="mt-4 text-sm text-center">
        <NuxtLink to="/forgot-password" class="link-primary">Mot de passe oublié ?</NuxtLink>
      </p>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useAuth } from '~/composables/useAuth'
import { useRouter } from 'vue-router'

const { login } = useAuth()
const router = useRouter()

const email = ref('')
const password = ref('')

const handleLogin = async () => {
  const success = await login(email.value, password.value)
  if (success) router.push('/')
  else alert('Identifiants incorrects')
}
</script>

<style scoped>
/* Nightmode via Tailwind dark classes */
.card {
  @apply shadow-lg rounded-2xl transition duration-300;
}
</style>
