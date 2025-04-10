// composables/useAuth.ts
import { ref, watch } from 'vue';
import gql from 'graphql-tag';
import { useNuxtApp } from '#app';
import type { User } from '~/types/user';

const user = ref<User | null>(null);
const isAuthenticated = ref(false);
const loading = ref(false);
const initialized = ref(false);

if (process.client) {
	console.log('useAuth module loaded');
	watch(user, (newVal) => {
		console.log('Global user state changed:', newVal);
	});
	watch(isAuthenticated, (newVal) => {
		console.log('Global isAuthenticated state changed:', newVal);
	});
}

const LOGIN_MUTATION = gql`
	mutation Login($credential: String!, $password: String!) {
		login(loginRequest: { credential: $credential, password: $password }) {
			userId
			email
			token
		}
	}
`;

const LOGOUT_MUTATION = gql`
	mutation Logout {
		logout {
			message
		}
	}
`;

// Query depuis le service Users pour récupérer le profil complet
const GET_CURRENT_USER = gql`
	query GetCurrentUser {
		getCurrentUser {
			id
			firstName
			lastName
			pseudo
			email
			classIds
		}
	}
`;

// Query depuis le service Auth pour récupérer le role
const GET_CURRENT_AUTH = gql`
	query GetCurrentAuth {
		getCurrentAuth {
			role
		}
	}
`;

function loadFromLocalStorage() {
	if (!process.client) return false;
	try {
		const stored = localStorage.getItem('auth');
		if (stored) {
			const data = JSON.parse(stored);
			if (data.timestamp && Date.now() - data.timestamp < 86400000) {
				console.log('Loading auth from localStorage:', data.user);
				user.value = data.user;
				isAuthenticated.value = true;
				return true;
			} else {
				console.log('Auth data in localStorage too old, removing');
				localStorage.removeItem('auth');
			}
		}
	} catch (e) {
		console.error('Error parsing auth from localStorage:', e);
		localStorage.removeItem('auth');
	}
	return false;
}

if (process.client && !initialized.value) {
	console.log('useAuth - trying to load from localStorage');
	if (loadFromLocalStorage()) {
		initialized.value = true;
		console.log('useAuth - successfully loaded from localStorage');
	}
}

export function useAuth() {
	const nuxtApp = useNuxtApp();

	function hasApolloAuth() {
		if (!nuxtApp || !nuxtApp.$apolloAuth) {
			console.error('Apollo Auth Client is not available yet');
			return false;
		}
		return true;
	}
	function hasApolloUsers() {
		if (!nuxtApp || !nuxtApp.$apolloUsers) {
			console.error('Apollo Users Client is not available yet');
			return false;
		}
		return true;
	}

	async function login(credential: string, password: string): Promise<boolean> {
		loading.value = true;
		if (!hasApolloAuth()) {
			loading.value = false;
			return false;
		}
		try {
			const apolloAuth = nuxtApp.$apolloAuth;
			console.log('Login attempt for:', credential);
			const result = await apolloAuth.mutate({
				mutation: LOGIN_MUTATION,
				variables: { credential, password },
			});
			if (result?.data?.login) {
				console.log('Login successful:', result.data.login);
				user.value = {
					id: result.data.login.userId,
					email: result.data.login.email,
					firstName: '',
					lastName: '',
					pseudo: '',
					classIds: [],
					role: '',
				};
				isAuthenticated.value = true;
				if (process.client) {
					localStorage.setItem(
						'auth',
						JSON.stringify({ user: user.value, timestamp: Date.now() }),
					);
				}
				await fetchCurrentUser(true);
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

	async function logout(): Promise<void> {
		console.log('Logout started');
		if (!hasApolloAuth()) {
			user.value = null;
			isAuthenticated.value = false;
			if (process.client) localStorage.removeItem('auth');
			return;
		}
		try {
			const apolloAuth = nuxtApp.$apolloAuth;
			await apolloAuth.mutate({ mutation: LOGOUT_MUTATION });
			console.log('Logout successful (API)');
		} catch (error) {
			console.error('Logout API error:', error);
		} finally {
			console.log('Clearing auth state');
			user.value = null;
			isAuthenticated.value = false;
			if (process.client) localStorage.removeItem('auth');
		}
	}

	async function fetchCurrentUser(force = false): Promise<User | null> {
		console.log('fetchCurrentUser called, force:', force, 'initialized:', initialized.value);
		if (initialized.value && !force) {
			console.log('Using cached auth state:', user.value);
			return user.value;
		}
		if (process.client && !initialized.value) {
			loadFromLocalStorage();
		}
		if (!hasApolloUsers()) {
			console.log('Apollo Users Client not available, using localStorage state');
			return user.value;
		}
		loading.value = true;
		console.log('Fetching current user from Users API');
		try {
			const apolloUsers = nuxtApp.$apolloUsers as any;
			const result = await apolloUsers.query({
				query: GET_CURRENT_USER,
				fetchPolicy: 'network-only',
			});
			if (result?.data?.getCurrentUser) {
				const fetchedUser = result.data.getCurrentUser;
				user.value = {
					...fetchedUser,
					role: user.value?.role || '', // role sera mis à jour ensuite
				};
				isAuthenticated.value = true;
				if (process.client) {
					localStorage.setItem(
						'auth',
						JSON.stringify({ user: user.value, timestamp: Date.now() }),
					);
				}
			} else {
				console.log('Users API: no user found');
				user.value = null;
				isAuthenticated.value = false;
				if (process.client) localStorage.removeItem('auth');
			}
		} catch (error) {
			console.error('Error fetching current user from Users API:', error);
		}
		// Récupération du role via le service Auth
		if (hasApolloAuth()) {
			try {
				const apolloAuth = nuxtApp.$apolloAuth as any;
				const authResult = await apolloAuth.query({
					query: GET_CURRENT_AUTH,
					fetchPolicy: 'network-only',
				});
				if (authResult?.data?.getCurrentAuth) {
					if (user.value) {
						user.value.role = authResult.data.getCurrentAuth.role;
					}
				}
			} catch (error) {
				console.error('Error fetching current auth data:', error);
			}
		}
		loading.value = false;
		initialized.value = true;
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
