// composables/useAuth.ts
import { ref, watch } from 'vue';
import gql from 'graphql-tag';
import { useNuxtApp } from '#app';
import type { User } from '~/types/user';

// ÉTAT GLOBAL PARTAGÉ - Crucial pour la persistance
const user = ref<User | null>(null);
const isAuthenticated = ref(false);
const loading = ref(false);
const initialized = ref(false);

// Debug reactivity
if (process.client) {
	console.log('useAuth module loaded - creating watchers');

	watch(user, (newVal) => {
		console.log('Global user state changed:', newVal);
	});

	watch(isAuthenticated, (newVal) => {
		console.log('Global isAuthenticated state changed:', newVal);
	});
}

// Requêtes GraphQL
const LOGIN_MUTATION = gql`
	mutation Login($auth: LoginRequest!) {
		login(auth: $auth) {
			user {
				id
				email
				pseudo
				role
			}
		}
	}
`;

const LOGOUT_MUTATION = gql`
	mutation {
		logout
	}
`;

const CURRENT_USER_QUERY = gql`
	query {
		get_current_user {
			id
			email
			pseudo
			role
		}
	}
`;

// Fonction pour charger depuis localStorage
function loadFromLocalStorage() {
	if (!process.client) return false;

	try {
		const stored = localStorage.getItem('auth');
		if (stored) {
			const data = JSON.parse(stored);

			// Vérifier que les données ne sont pas trop anciennes (24h max)
			if (data.timestamp && Date.now() - data.timestamp < 86400000) {
				console.log('Loading auth from localStorage:', data.user);
				user.value = data.user;
				isAuthenticated.value = true;
				return true;
			} else {
				// Données trop anciennes
				console.log('Auth data in localStorage too old, removing');
				localStorage.removeItem('auth');
			}
		}
	} catch (e) {
		// Erreur lors du parsing, nettoyer localStorage
		console.error('Error parsing auth from localStorage:', e);
		localStorage.removeItem('auth');
	}
	return false;
}

// Initialiser depuis localStorage si possible
if (process.client && !initialized.value) {
	console.log('useAuth - trying to load from localStorage');
	if (loadFromLocalStorage()) {
		initialized.value = true;
		console.log('useAuth - successfully loaded from localStorage');
	}
}

export function useAuth() {
	const nuxtApp = useNuxtApp();

	// Fonction pour vérifier si Apollo est disponible
	function hasApollo() {
		if (!nuxtApp || !nuxtApp.$apollo) {
			console.error('Apollo is not available yet');
			return false;
		}
		return true;
	}

	// Fonction de login
	async function login(email: string, password: string): Promise<boolean> {
		loading.value = true;

		if (!hasApollo()) {
			loading.value = false;
			return false;
		}

		try {
			const apollo = nuxtApp.$apollo;
			console.log('Login attempt for:', email);

			const result = await apollo.mutate({
				mutation: LOGIN_MUTATION,
				variables: { auth: { email, password } },
			});

			if (result?.data?.login?.user) {
				console.log('Login successful:', result.data.login.user);

				// Mettre à jour l'état global
				user.value = result.data.login.user;
				isAuthenticated.value = true;

				// Sauvegarder dans localStorage
				if (process.client) {
					localStorage.setItem(
						'auth',
						JSON.stringify({
							user: user.value,
							timestamp: Date.now(),
						}),
					);
				}

				initialized.value = true;
				return true;
			}

			console.log('Login failed: no user data returned');
			return false;
		} catch (error) {
			console.error('Login error:', error);
			return false;
		} finally {
			loading.value = false;
		}
	}

	// Fonction de logout
	async function logout(): Promise<void> {
		console.log('Logout started');

		if (!hasApollo()) {
			// Même en cas d'erreur Apollo, on nettoie l'état local
			user.value = null;
			isAuthenticated.value = false;

			if (process.client) {
				localStorage.removeItem('auth');
			}
			return;
		}

		try {
			const apollo = nuxtApp.$apollo;
			await apollo.mutate({ mutation: LOGOUT_MUTATION });
			console.log('Logout successful (API)');
		} catch (error) {
			console.error('Logout API error:', error);
		} finally {
			// Toujours nettoyer l'état local, même en cas d'erreur réseau
			console.log('Clearing auth state');
			user.value = null;
			isAuthenticated.value = false;

			if (process.client) {
				localStorage.removeItem('auth');
			}
		}
	}

	// Fonction pour récupérer l'utilisateur courant
	async function fetchCurrentUser(force = false): Promise<User | null> {
		console.log('fetchCurrentUser called, force:', force, 'initialized:', initialized.value);

		// Si déjà initialisé et pas forcé, retourner les données actuelles
		if (initialized.value && !force) {
			console.log('Using cached auth state:', user.value);
			return user.value;
		}

		// Si client et pas initialisé, essayer de charger depuis localStorage
		if (process.client && !initialized.value) {
			loadFromLocalStorage();
		}

		// Vérifier si Apollo est disponible
		if (!hasApollo()) {
			console.log('Apollo not available, using localStorage state');
			return user.value;
		}

		// Charger depuis le serveur
		loading.value = true;
		console.log('Fetching current user from API');

		try {
			const apollo = nuxtApp.$apollo;
			const result = await apollo.query({
				query: CURRENT_USER_QUERY,
				fetchPolicy: 'network-only', // Crucial: ne pas utiliser le cache
			});

			if (result?.data?.get_current_user) {
				console.log('Current user API success:', result.data.get_current_user);

				// Mettre à jour l'état global
				user.value = result.data.get_current_user;
				isAuthenticated.value = true;

				// Sauvegarder dans localStorage
				if (process.client) {
					localStorage.setItem(
						'auth',
						JSON.stringify({
							user: user.value,
							timestamp: Date.now(),
						}),
					);
				}
			} else {
				console.log('Current user API: no user found');

				// Pas d'utilisateur - s'assurer que l'état est réinitialisé
				user.value = null;
				isAuthenticated.value = false;

				if (process.client) {
					localStorage.removeItem('auth');
				}
			}
		} catch (error) {
			console.error('Error fetching current user:', error);
			// NE PAS réinitialiser l'état en cas d'erreur réseau
			// Cela permet de conserver l'authentification en cas de problème API
		} finally {
			loading.value = false;
			initialized.value = true;
		}

		console.log('fetchCurrentUser complete, authenticated:', isAuthenticated.value);
		return user.value;
	}

	return {
		user,
		isAuthenticated,
		loading,
		login,
		logout,
		fetchCurrentUser,
	};
}
