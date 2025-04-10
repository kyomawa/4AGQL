<template>
	<div class="min-h-screen py-12 px-4 sm:px-6 lg:px-8 dark:to-violet-900">
		<div class="w-[830px] mx-auto">
			<Head>
				<title>Profil - SchoolInc</title>
			</Head>
			<h2 class="text-center title-primary">Votre profil</h2>

			<div class="card p-6 mx-auto w-full">
				<div class="space-y-5">
					<!-- Prénom -->
					<div class="profile-item">
						<label class="label-primary dark:text-violet-300">Prénom</label>
						<div class="flex items-center">
							<div
								v-if="editingFields.firstName"
								class="flex-1"
							>
								<input
									v-model="draftProfile.firstName"
									type="text"
									class="input-primary dark:bg-violet-900 dark:text-white dark:border-violet-800"
								/>
							</div>
							<div
								v-else
								class="flex-1 text-violet-200"
							>
								{{ userProfile.firstName }}
							</div>
							<div class="ml-2">
								<button
									v-if="editingFields.firstName"
									@click="saveField('firstName')"
									class="btn-primary text-sm"
								>
									Sauvegarder
								</button>
								<button
									v-if="editingFields.firstName"
									@click="cancelField('firstName')"
									class="btn-primary text-sm ml-2 bg-violet-800 hover:bg-violet-700"
								>
									Annuler
								</button>
								<button
									v-else
									@click="editField('firstName')"
									class="btn-primary text-sm"
								>
									Modifier
								</button>
							</div>
						</div>
					</div>

					<!-- Nom -->
					<div class="profile-item">
						<label class="label-primary dark:text-violet-300">Nom</label>
						<div class="flex items-center">
							<div
								v-if="editingFields.lastName"
								class="flex-1"
							>
								<input
									v-model="draftProfile.lastName"
									type="text"
									class="input-primary dark:bg-violet-900 dark:text-white dark:border-violet-800"
								/>
							</div>
							<div
								v-else
								class="flex-1 text-violet-200"
							>
								{{ userProfile.lastName }}
							</div>
							<div class="ml-2">
								<button
									v-if="editingFields.lastName"
									@click="saveField('lastName')"
									class="btn-primary text-sm"
								>
									Sauvegarder
								</button>
								<button
									v-if="editingFields.lastName"
									@click="cancelField('lastName')"
									class="btn-primary text-sm ml-2 bg-violet-800 hover:bg-violet-700"
								>
									Annuler
								</button>
								<button
									v-else
									@click="editField('lastName')"
									class="btn-primary text-sm"
								>
									Modifier
								</button>
							</div>
						</div>
					</div>

					<!-- Pseudo -->
					<div class="profile-item">
						<label class="label-primary dark:text-violet-300">Pseudo</label>
						<div class="flex items-center">
							<div
								v-if="editingFields.pseudo"
								class="flex-1"
							>
								<input
									v-model="draftProfile.pseudo"
									type="text"
									class="input-primary dark:bg-violet-900 dark:text-white dark:border-violet-800"
								/>
							</div>
							<div
								v-else
								class="flex-1 text-violet-200"
							>
								{{ userProfile.pseudo }}
							</div>
							<div class="ml-2">
								<button
									v-if="editingFields.pseudo"
									@click="saveField('pseudo')"
									class="btn-primary text-sm"
								>
									Sauvegarder
								</button>
								<button
									v-if="editingFields.pseudo"
									@click="cancelField('pseudo')"
									class="btn-primary text-sm ml-2 bg-violet-800 hover:bg-violet-700"
								>
									Annuler
								</button>
								<button
									v-else
									@click="editField('pseudo')"
									class="btn-primary text-sm"
								>
									Modifier
								</button>
							</div>
						</div>
					</div>

					<!-- Email -->
					<div class="profile-item">
						<label class="label-primary dark:text-violet-300">Adresse email</label>
						<div class="flex items-center">
							<div
								v-if="editingFields.email"
								class="flex-1"
							>
								<input
									v-model="draftProfile.email"
									type="email"
									class="input-primary dark:bg-violet-900 dark:text-white dark:border-violet-800"
								/>
							</div>
							<div
								v-else
								class="flex-1 text-violet-200"
							>
								{{ userProfile.email }}
							</div>
							<div class="ml-2">
								<button
									v-if="editingFields.email"
									@click="saveField('email')"
									class="btn-primary text-sm"
								>
									Sauvegarder
								</button>
								<button
									v-if="editingFields.email"
									@click="cancelField('email')"
									class="btn-primary text-sm ml-2 bg-violet-800 hover:bg-violet-700"
								>
									Annuler
								</button>
								<button
									v-else
									@click="editField('email')"
									class="btn-primary text-sm"
								>
									Modifier
								</button>
							</div>
						</div>
					</div>

					<!-- Rôle (lecture seule) -->
					<div class="profile-item">
						<label class="label-primary dark:text-violet-300">Rôle</label>
						<div class="flex items-center">
							<div class="flex-1 text-violet-200">
								<!-- Laissé vide pour aligner avec les autres champs -->
							</div>
							<div class="ml-2">
								<span :class="['px-3 py-1 rounded-lg text-sm', roleColorClass]">
									{{ formattedRole }}
								</span>
							</div>
						</div>
					</div>

					<!-- Bouton de suppression du compte -->
					<div class="text-center mt-8">
						<button
							@click="confirmDeleteAccount"
							class="btn-primary bg-red-600 hover:bg-red-700"
						>
							Supprimer le compte
						</button>
					</div>
				</div>
			</div>

			<!-- Modal de confirmation de suppression -->
			<div
				v-if="showDeleteConfirm"
				class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50"
			>
				<div class="w-full max-w-md p-6 card dark:bg-violet-950">
					<h3 class="mb-4 text-xl font-bold text-violet-300">Confirmer la suppression</h3>
					<p class="mb-6 text-violet-200">
						Êtes-vous sûr de vouloir supprimer votre compte ? Cette action est
						irréversible.
					</p>
					<div class="flex gap-4">
						<button
							@click="deleteAccount"
							:disabled="deleting"
							class="btn-primary flex-1 bg-red-600 hover:bg-red-700"
						>
							{{ deleting ? 'Suppression...' : 'Confirmer la suppression' }}
						</button>
						<button
							@click="showDeleteConfirm = false"
							class="btn-primary flex-1 bg-violet-800 hover:bg-violet-700"
						>
							Annuler
						</button>
					</div>
				</div>
			</div>

			<!-- Notification (erreur ou succès) -->
			<div
				v-if="error || notification"
				class="fixed top-[80px] right-4 w-[340px] p-4 rounded-lg shadow-md"
				:class="error ? 'bg-red-600' : 'bg-green-600'"
			>
				<p class="text-white text-center">{{ error || notification }}</p>
				<button
					@click="clearNotification"
					class="absolute top-2 right-2 text-white hover:text-red-100"
				>
					×
				</button>
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import gql from 'graphql-tag';
import { useNuxtApp } from '#app';
import { useAuth } from '~/composables/useAuth';

interface UserProfile {
	id: string;
	firstName: string;
	lastName: string;
	pseudo: string;
	email: string;
	role: string;
	classIds: string[];
}

// Type pour limiter les champs éditables
type EditableField = 'firstName' | 'lastName' | 'pseudo' | 'email';

const router = useRouter();
const { user, logout } = useAuth();
const nuxtApp = useNuxtApp();

const loading = ref(false);
const error = ref<string | null>(null);
const notification = ref<string | null>(null);
const deleting = ref(false);
const showDeleteConfirm = ref(false);

const userProfile = ref<UserProfile>({
	id: '',
	firstName: '',
	lastName: '',
	pseudo: '',
	email: '',
	role: '',
	classIds: [],
});

// Copie pour l'édition inline
const draftProfile = ref<UserProfile>({ ...userProfile.value });

// Objet réactif pour suivre quels champs sont en mode édition
const editingFields = ref<Record<EditableField, boolean>>({
	firstName: false,
	lastName: false,
	pseudo: false,
	email: false,
});

// Query pour récupérer l'utilisateur courant depuis le service Users
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

// Query pour récupérer le rôle depuis le service Auth
const GET_CURRENT_AUTH = gql`
	query GetCurrentAuth {
		getCurrentAuth {
			role
		}
	}
`;

// Mutation pour mettre à jour l'utilisateur courant (UpdateUserRequest)
const UPDATE_CURRENT_USER = gql`
	mutation UpdateCurrentUser($user: UpdateUserRequest!) {
		updateCurrentUser(user: $user) {
			id
			firstName
			lastName
			pseudo
			email
			classIds
		}
	}
`;

// Mutation pour supprimer l'utilisateur courant
const DELETE_CURRENT_USER = gql`
	mutation DeleteCurrentUser {
		deleteCurrentUser {
			id
		}
	}
`;

// Fonction pour charger les données utilisateur
const loadUserData = async () => {
	loading.value = true;
	error.value = null;
	try {
		// Récupérer le profil depuis le service Users
		const result = await (nuxtApp.$apolloUsers as any).query({
			query: GET_CURRENT_USER,
			fetchPolicy: 'network-only',
		});
		if (result.data?.getCurrentUser) {
			userProfile.value = { ...result.data.getCurrentUser } as UserProfile;
			draftProfile.value = { ...result.data.getCurrentUser } as UserProfile;
		} else {
			error.value = 'Impossible de récupérer vos informations de profil.';
		}
		// Récupérer le rôle depuis le service Auth
		const authResult = await (nuxtApp.$apolloAuth as any).query({
			query: GET_CURRENT_AUTH,
			fetchPolicy: 'network-only',
		});
		if (authResult.data?.getCurrentAuth) {
			userProfile.value.role = authResult.data.getCurrentAuth.role;
			draftProfile.value.role = authResult.data.getCurrentAuth.role;
		}
	} catch (err: any) {
		console.error('Error loading user profile:', err);
		error.value = 'Impossible de charger votre profil. Veuillez actualiser la page.';
	} finally {
		loading.value = false;
	}
};

onMounted(() => {
	loadUserData();
});

// Fonctions d'édition inline
const editField = (field: EditableField) => {
	editingFields.value[field] = true;
};

const cancelField = (field: EditableField) => {
	draftProfile.value[field] = userProfile.value[field];
	editingFields.value[field] = false;
};

const saveField = async (field: EditableField) => {
	loading.value = true;
	try {
		const updatedUser = {
			firstName: draftProfile.value.firstName,
			lastName: draftProfile.value.lastName,
			pseudo: draftProfile.value.pseudo,
			email: draftProfile.value.email,
		};
		const result = await (nuxtApp.$apolloUsers as any).mutate({
			mutation: UPDATE_CURRENT_USER,
			variables: { user: updatedUser },
		});
		if (result.data?.updateCurrentUser) {
			const currentRole = userProfile.value.role;
			userProfile.value = {
				...result.data.updateCurrentUser,
				role: currentRole,
			} as UserProfile;
			draftProfile.value = {
				...result.data.updateCurrentUser,
				role: currentRole,
			} as UserProfile;
			editingFields.value[field] = false;
			notification.value = 'Profil mis à jour avec succès.';
			setTimeout(() => {
				notification.value = null;
			}, 3000);
		} else {
			error.value = 'Erreur lors de la mise à jour du profil.';
		}
	} catch (err: any) {
		console.error('Error updating profile:', err);
		error.value = 'Impossible de mettre à jour votre profil. Veuillez réessayer.';
	} finally {
		loading.value = false;
	}
};

const confirmDeleteAccount = () => {
	showDeleteConfirm.value = true;
};

const deleteAccount = async () => {
	deleting.value = true;
	try {
		await (nuxtApp.$apolloUsers as any).mutate({
			mutation: DELETE_CURRENT_USER,
		});
		await logout();
		router.push({ path: '/', query: { deleted: 'Votre compte a été supprimé avec succès.' } });
	} catch (err: any) {
		console.error('Error deleting account:', err);
		error.value = 'Impossible de supprimer votre compte. Veuillez réessayer.';
		showDeleteConfirm.value = false;
	} finally {
		deleting.value = false;
	}
};

const formattedRole = computed(() => {
	const role = userProfile.value.role;
	if (role === 'ADMIN') return 'Administrateur';
	if (role === 'PROFESSOR') return 'Professeur';
	if (role === 'USER') return 'Étudiant';
	return role;
});

const roleColorClass = computed(() => {
	const role = userProfile.value.role;
	if (role === 'ADMIN') return 'bg-purple-600 text-white';
	if (role === 'PROFESSOR') return 'bg-blue-600 text-white';
	if (role === 'USER') return 'bg-green-600 text-white';
	return 'bg-gray-600 text-white';
});

const clearNotification = () => {
	error.value = null;
	notification.value = null;
};
</script>

<style scoped>
.card {
	@apply bg-white shadow-lg rounded-2xl transition duration-300 dark:bg-violet-950;
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
	@apply py-2 px-4 font-medium text-white transition-colors bg-violet-600 rounded-lg hover:bg-violet-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-violet-500 disabled:opacity-50 disabled:cursor-not-allowed;
}

.link-primary {
	@apply text-violet-600 hover:text-violet-800 dark:text-violet-400 dark:hover:text-violet-300;
}

/* Styles spécifiques à profile.vue */
.profile-item {
	@apply border-b border-violet-800 pb-4;
}
</style>
