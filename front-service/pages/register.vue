<template>
	<div
		class="w-[830px] mx-auto mt-20 dark:to-violet-900"
		style="margin-top: 30px"
	>
		<div class="max-w-md mx-auto bg-white card dark:bg-violet-950">
			<Head>
				<title>Créer un compte - SchoolInc</title>
			</Head>

			<h2 class="text-center title-primary dark:text-violet-300">Créer un compte</h2>

			<form
				@submit.prevent="handleRegister"
				class="space-y-5"
			>
				<!-- Grille à 2 colonnes pour 6 champs -->
				<div class="grid grid-cols-2 gap-4">
					<!-- Prénom -->
					<div>
						<label class="label-primary dark:text-violet-300">Prénom</label>
						<input
							v-model="firstName"
							type="text"
							class="input-primary dark:bg-violet-900 dark:text-white dark:border-violet-800"
							required
						/>
					</div>
					<!-- Nom -->
					<div>
						<label class="label-primary dark:text-violet-300">Nom</label>
						<input
							v-model="lastName"
							type="text"
							class="input-primary dark:bg-violet-900 dark:text-white dark:border-violet-800"
							required
						/>
					</div>
					<!-- Pseudo -->
					<div>
						<label class="label-primary dark:text-violet-300">Pseudo</label>
						<input
							v-model="pseudo"
							type="text"
							class="input-primary dark:bg-violet-900 dark:text-white dark:border-violet-800"
							required
						/>
					</div>
					<!-- Adresse email -->
					<div>
						<label class="label-primary dark:text-violet-300">Adresse email</label>
						<input
							v-model="email"
							type="email"
							class="input-primary dark:bg-violet-900 dark:text-white dark:border-violet-800"
							required
						/>
					</div>
					<!-- Mot de passe -->
					<div>
						<label class="label-primary dark:text-violet-300">Mot de passe</label>
						<input
							v-model="password"
							type="password"
							class="input-primary dark:bg-violet-900 dark:text-white dark:border-violet-800"
							required
						/>
					</div>
					<!-- Confirmer le mot de passe -->
					<div>
						<label class="label-primary dark:text-violet-300"
							>Confirmer le mot de passe</label
						>
						<input
							v-model="confirmPassword"
							type="password"
							class="input-primary dark:bg-violet-900 dark:text-white dark:border-violet-800"
							required
						/>
					</div>
				</div>

				<!-- Message d'erreur -->
				<div
					v-if="error"
					class="p-3 text-sm text-center text-red-600 bg-red-100 rounded-md dark:bg-red-900 dark:text-red-200"
				>
					{{ error }}
				</div>

				<!-- Bouton d'inscription centré -->
				<div class="text-center">
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
							Création en cours...
						</span>
						<span v-else>S'inscrire</span>
					</button>
				</div>

				<!-- Lien vers la page de connexion -->
				<p class="mt-4 text-sm text-center text-gray-600 dark:text-gray-400">
					Vous avez déjà un compte ?
					<NuxtLink
						to="/login"
						class="link-primary"
						>Se connecter</NuxtLink
					>
				</p>
			</form>
		</div>
	</div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useRouter, useNuxtApp } from '#app';
import gql from 'graphql-tag';

const firstName = ref('');
const lastName = ref('');
const pseudo = ref('');
const email = ref('');
const password = ref('');
const confirmPassword = ref('');
const loading = ref(false);
const error = ref('');
const router = useRouter();
const nuxtApp = useNuxtApp();

const REGISTER_MUTATION = gql`
	mutation Register(
		$firstName: String!
		$lastName: String!
		$email: String!
		$pseudo: String!
		$password: String!
		$confirmPassword: String!
	) {
		register(
			user: {
				firstName: $firstName
				lastName: $lastName
				email: $email
				pseudo: $pseudo
				password: $password
				confirmPassword: $confirmPassword
			}
		) {
			userId
			email
			token
		}
	}
`;

const handleRegister = async () => {
	error.value = '';

	if (password.value !== confirmPassword.value) {
		error.value = 'Les mots de passe ne correspondent pas';
		return;
	}

	if (!nuxtApp.$apolloAuth) {
		error.value = "Le service d'authentification n'est pas disponible";
		return;
	}

	loading.value = true;

	try {
		await nuxtApp.$apolloAuth.mutate({
			mutation: REGISTER_MUTATION,
			variables: {
				firstName: firstName.value,
				lastName: lastName.value,
				email: email.value,
				pseudo: pseudo.value,
				password: password.value,
				confirmPassword: confirmPassword.value,
			},
		});

		// Succès : rediriger vers la page de connexion
		router.push('/login');
	} catch (e: any) {
		console.error("Erreur lors de l'inscription:", e);
		error.value = e.message || "Une erreur est survenue lors de l'inscription";
	} finally {
		loading.value = false;
	}
};
</script>

<style scoped>
.card {
	@apply p-6 shadow-lg rounded-2xl transition duration-300;
}

.title-primary {
	@apply mb-6 text-2xl font-bold text-violet-700 dark:text-violet-300;
}

.label-primary {
	@apply block mb-1 text-sm font-medium dark:text-violet-300;
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
