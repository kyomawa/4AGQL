<template>
	<div
		class="w-[830px] mx-auto mt-20 dark:to-violet-900"
		style="margin-top: 30px"
	>
		<div class="max-w-md mx-auto bg-white card dark:bg-violet-950">
			<Head>
				<title>Connexion</title>
			</Head>

			<h2 class="text-center title-primary dark:text-violet-200">Connexion</h2>

			<form
				@submit.prevent="handleLogin"
				class="space-y-5"
			>
				<div>
					<label class="label-primary dark:text-violet-200">Adresse email</label>
					<input
						v-model="email"
						type="email"
						class="input-primary dark:bg-violet-900 dark:text-white dark:border-violet-800"
						:disabled="loading"
						required
					/>
				</div>

				<div>
					<label class="label-primary dark:text-violet-200">Mot de passe</label>
					<input
						v-model="password"
						type="password"
						class="input-primary dark:bg-violet-900 dark:text-white dark:border-violet-800"
						:disabled="loading"
						required
					/>
				</div>

				<!-- Message d'erreur -->
				<div
					v-if="error"
					class="p-3 text-sm text-center text-red-600 bg-red-100 rounded-md dark:bg-red-900 dark:text-red-200"
				>
					{{ error }}
				</div>

				<button
					type="submit"
					class="w-full btn-primary"
					:disabled="loading"
				>
					<span v-if="loading">
						<svg
							class="inline w-5 h-5 mr-2 -ml-1 text-white animate-spin"
							xmlns="http://www.w3.org/2000/svg"
							fill="none"
							viewBox="0 0 24 24"
						>
							<circle
								class="opacity-25"
								cx="12"
								cy="12"
								r="10"
								stroke="currentColor"
								stroke-width="4"
							></circle>
							<path
								class="opacity-75"
								fill="currentColor"
								d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
							></path>
						</svg>
						Connexion en cours...
					</span>
					<span v-else>Se connecter</span>
				</button>

				<p class="mt-4 text-sm text-center text-gray-600 dark:text-gray-400">
					Mot de passe oublié ? Veuillez contacter la modération.
				</p>

				<p class="text-sm text-center text-gray-600 dark:text-gray-400">
					Pas encore de compte ?
					<NuxtLink
						to="/register"
						class="link-primary"
						>S'inscrire</NuxtLink
					>
				</p>
			</form>
		</div>
	</div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useAuth } from '~/composables/useAuth';
import { useRouter } from 'vue-router';

const { login, loading } = useAuth();
const router = useRouter();

const email = ref('');
const password = ref('');
const error = ref('');

const handleLogin = async () => {
	error.value = ''; // Réinitialiser l'erreur à chaque tentative

	try {
		const success = await login(email.value, password.value);
		if (success) {
			console.log('Login successful, redirecting to home');
			router.push('/');
		} else {
			error.value = 'Identifiants incorrects';
		}
	} catch (e: any) {
		console.error('Login error:', e);
		error.value = e.message || 'Une erreur est survenue lors de la connexion';
	}
};
</script>

<style scoped>
/* Nightmode via Tailwind dark classes */
.card {
	@apply p-6 shadow-lg rounded-2xl transition duration-300;
}

.title-primary {
	@apply mb-6 text-2xl font-bold text-violet-700 dark:text-violet-200;
}

.label-primary {
	@apply block mb-1 text-sm font-medium dark:text-violet-200;
}

.input-primary {
	@apply w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-violet-500 focus:border-transparent;
}

.btn-primary {
	@apply py-2 font-medium text-white transition-colors bg-violet-600 rounded-lg hover:bg-violet-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-violet-500 disabled:opacity-50 disabled:cursor-not-allowed;
}

.link-primary {
	@apply text-violet-600 hover:text-violet-800 dark:text-violet-400 dark:hover:text-violet-300;
}
</style>
