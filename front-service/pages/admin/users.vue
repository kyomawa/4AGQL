<template>
	<div class="min-h-screen py-12 px-4 sm:px-6 lg:px-8 dark:to-violet-900">
		<div class="w-[830px] mx-auto">
			<Head>
				<title>Gestion des utilisateurs - SchoolInc</title>
			</Head>
			<h2 class="text-center title-primary">Gestion des utilisateurs</h2>

			<!-- Contrôles de recherche -->
			<div class="card p-6 mx-auto w-full mb-6">
				<div class="flex flex-wrap gap-3">
					<div class="flex-1">
						<label class="label-primary dark:text-violet-300">
							Rechercher des utilisateurs
						</label>
						<input
							type="text"
							v-model="searchQuery"
							placeholder="Rechercher par prénom ou nom"
							class="input-primary dark:bg-violet-900 dark:text-white dark:border-violet-800"
							@keyup.enter="refetchUsers"
						/>
					</div>
				</div>
			</div>

			<!-- Aucun utilisateur trouvé -->
			<div
				v-if="filteredUsers.length === 0"
				class="card p-6 mx-auto w-full text-center"
			>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					class="w-16 h-16 mx-auto text-violet-400"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z"
					/>
				</svg>
				<h3 class="mt-4 text-lg font-medium text-violet-300">Aucun utilisateur trouvé</h3>
				<p class="mt-1 text-violet-300">Essayez de modifier vos critères de recherche.</p>
			</div>

			<!-- Tableau des utilisateurs -->
			<div
				v-else
				class="card mx-auto w-full overflow-hidden"
			>
				<table class="min-w-full divide-y divide-violet-800">
					<thead class="bg-violet-900">
						<tr>
							<th
								class="px-6 py-3 text-xs font-medium tracking-wider text-left text-violet-300 uppercase"
							>
								Prénom
							</th>
							<th
								class="px-6 py-3 text-xs font-medium tracking-wider text-left text-violet-300 uppercase"
							>
								Nom
							</th>
							<th
								class="px-6 py-3 text-xs font-medium tracking-wider text-left text-violet-300 uppercase"
							>
								Email
							</th>
							<th
								class="px-6 py-3 text-xs font-medium tracking-wider text-left text-violet-300 uppercase"
							>
								Pseudo
							</th>
							<th
								class="px-6 py-3 text-xs font-medium tracking-wider text-right text-violet-300 uppercase"
							>
								Actions
							</th>
						</tr>
					</thead>
					<tbody class="divide-y divide-violet-800">
						<tr
							v-for="u in filteredUsers"
							:key="u.id"
							class="hover:bg-violet-900"
						>
							<td class="px-6 py-4 whitespace-nowrap">
								<div class="text-sm font-medium text-violet-200">
									{{ u.firstName }}
								</div>
							</td>
							<td class="px-6 py-4 whitespace-nowrap">
								<div class="text-sm text-violet-200">{{ u.lastName }}</div>
							</td>
							<td class="px-6 py-4 whitespace-nowrap">
								<div class="text-sm text-violet-200">{{ u.email }}</div>
							</td>
							<td class="px-6 py-4 whitespace-nowrap">
								<div class="text-sm text-violet-200">{{ u.pseudo }}</div>
							</td>
							<td class="px-6 py-4 text-sm font-medium text-right whitespace-nowrap">
								<button
									@click="editUser(u)"
									class="mr-3 link-primary"
								>
									Modifier
								</button>
								<button
									@click="confirmDeleteUser(u)"
									class="text-red-400 hover:text-red-300"
								>
									Supprimer
								</button>
							</td>
						</tr>
					</tbody>
				</table>
			</div>
		</div>

		<!-- Modal d'édition de l'utilisateur -->
		<div
			v-if="showEditModal"
			class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50"
		>
			<div class="w-full max-w-md p-6 card dark:bg-violet-950">
				<h3 class="mb-4 text-xl font-bold text-violet-300">Modifier l'utilisateur</h3>
				<form
					@submit.prevent="updateUser"
					class="space-y-4"
				>
					<div>
						<label class="label-primary dark:text-violet-300">Prénom</label>
						<input
							v-model="userForm.firstName"
							type="text"
							class="input-primary dark:bg-violet-900 dark:text-white dark:border-violet-800"
							required
						/>
					</div>
					<div>
						<label class="label-primary dark:text-violet-300">Nom</label>
						<input
							v-model="userForm.lastName"
							type="text"
							class="input-primary dark:bg-violet-900 dark:text-white dark:border-violet-800"
							required
						/>
					</div>
					<div>
						<label class="label-primary dark:text-violet-300">Email</label>
						<input
							v-model="userForm.email"
							type="email"
							class="input-primary dark:bg-violet-900 dark:text-white dark:border-violet-800"
							required
						/>
					</div>
					<div>
						<label class="label-primary dark:text-violet-300">Pseudo</label>
						<input
							v-model="userForm.pseudo"
							type="text"
							class="input-primary dark:bg-violet-900 dark:text-white dark:border-violet-800"
							required
						/>
					</div>
					<div>
						<label class="label-primary dark:text-violet-300">Rôle</label>
						<select
							v-model="userForm.role"
							class="input-primary dark:bg-violet-900 dark:text-white dark:border-violet-800"
							required
						>
							<option value="USER">Étudiant</option>
							<option value="PROFESSOR">Professeur</option>
							<option value="ADMIN">Administrateur</option>
						</select>
					</div>
					<div class="flex justify-end pt-4 space-x-3">
						<button
							type="button"
							@click="showEditModal = false"
							class="btn-primary bg-violet-800 hover:bg-violet-700"
						>
							Annuler
						</button>
						<button
							type="submit"
							class="btn-primary"
							:disabled="updating"
						>
							{{ updating ? 'Mise à jour...' : 'Mettre à jour' }}
						</button>
					</div>
				</form>
			</div>
		</div>

		<!-- Modal de confirmation de suppression -->
		<div
			v-if="showDeleteModal"
			class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50"
		>
			<div class="w-full max-w-md p-6 card dark:bg-violet-950">
				<h3 class="mb-4 text-xl font-bold text-violet-300">Supprimer l'utilisateur</h3>
				<p class="mb-6 text-violet-200">
					Êtes-vous sûr de vouloir supprimer cet utilisateur ? Cette action est
					irréversible et supprimera toutes les données associées.
				</p>
				<div class="flex justify-end space-x-3">
					<button
						@click="showDeleteModal = false"
						class="btn-primary bg-violet-800 hover:bg-violet-700"
					>
						Annuler
					</button>
					<button
						@click="deleteUser"
						class="btn-primary bg-red-600 hover:bg-red-700"
						:disabled="deleting"
					>
						{{ deleting ? 'Suppression...' : 'Supprimer' }}
					</button>
				</div>
			</div>
		</div>

		<!-- Notification (succès ou erreur) -->
		<div
			v-if="notification"
			:class="[
				'fixed bottom-4 right-4 px-6 py-3 rounded-lg shadow-lg max-w-xs',
				notification.type === 'success'
					? 'bg-green-600 text-white'
					: 'bg-red-600 text-white',
			]"
		>
			{{ notification.message }}
		</div>
	</div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, watch } from 'vue';
import { useQuery, useMutation, provideApolloClient } from '@vue/apollo-composable';
import gql from 'graphql-tag';
import { useNuxtApp } from '#app';
import { useAuth } from '~/composables/useAuth';

definePageMeta({
	middleware: ['auth'],
});

const nuxtApp = useNuxtApp();
provideApolloClient(nuxtApp.$apolloUsers as any);

const { user: currentUser } = useAuth();
const currentUserId = computed(() => currentUser.value?.id || '');

const loading = ref(true);
const updating = ref(false);
const deleting = ref(false);
const users = ref<any[]>([]);
const searchQuery = ref('');
// On filtre uniquement par prénom et nom
const filteredUsers = computed(() => {
	let result = users.value;
	if (searchQuery.value) {
		const query = searchQuery.value.toLowerCase();
		result = result.filter(
			(u) =>
				u.firstName.toLowerCase().includes(query) ||
				u.lastName.toLowerCase().includes(query),
		);
	}
	return result;
});

const notification = ref<{ type: 'success' | 'error'; message: string } | null>(null);
const showEditModal = ref(false);
const showDeleteModal = ref(false);

const userForm = reactive({
	id: '',
	firstName: '',
	lastName: '',
	email: '',
	pseudo: '',
	// Le rôle doit être fourni selon votre schéma (AUTHROLE) – ADMIN, PROFESSOR, USER
	role: '',
	classIds: [] as string[],
});

const selectedUser = ref<any>(null);

const GET_USERS_QUERY = gql`
	query GetUsers {
		getUsers {
			id
			firstName
			lastName
			email
			pseudo
			classIds
		}
	}
`;

// Remarque : le serveur attend que l'objet utilisateur soit de type UpdateUserRequest
const UPDATE_USER_MUTATION = gql`
	mutation UpdateUserById($id: String!, $user: UpdateUserRequest!) {
		updateUserById(id: $id, user: $user) {
			id
			firstName
			lastName
			email
			pseudo
			classIds
		}
	}
`;

const DELETE_USER_MUTATION = gql`
	mutation DeleteUserById($id: String!) {
		deleteUserById(id: $id) {
			id
			firstName
			lastName
			email
			pseudo
			classIds
		}
	}
`;

const UPDATE_ROLE_MUTATION = gql`
	mutation UpdateRole($userId: String!, $role: String!) {
		updateRole(userId: $userId, role: { role: $role }) {
			id
			role
		}
	}
`;

const { result, loading: queryLoading, refetch } = useQuery(GET_USERS_QUERY);

const { mutate: updateUserMutation } = useMutation(UPDATE_USER_MUTATION);
const { mutate: deleteUserMutation } = useMutation(DELETE_USER_MUTATION);
// La mutation updateRole est exécutée via le client Apollo Auth
const { mutate: updateRoleMutation } = useMutation(UPDATE_ROLE_MUTATION, {
	clientId: 'apolloAuth',
});

watch(result, (newResult) => {
	if (newResult?.getUsers) {
		users.value = newResult.getUsers;
	}
});

watch(queryLoading, (isLoading) => {
	loading.value = isLoading;
});

onMounted(async () => {
	await refetch();
});

const editUser = (u: any) => {
	selectedUser.value = u;
	// Copier les données pour le formulaire d'édition
	userForm.id = u.id;
	userForm.firstName = u.firstName;
	userForm.lastName = u.lastName;
	userForm.email = u.email;
	userForm.pseudo = u.pseudo;
	// Le backend ne retourne pas le rôle ; laissez vide ou définissez une valeur par défaut
	userForm.role = '';
	userForm.classIds = u.classIds || [];
	showEditModal.value = true;
};

const confirmDeleteUser = (u: any) => {
	selectedUser.value = u;
	showDeleteModal.value = true;
};

const updateUser = async () => {
	if (!selectedUser.value) return;
	updating.value = true;
	try {
		// On envoie toutes les informations (incluant role)
		await updateUserMutation({
			id: userForm.id,
			user: {
				firstName: userForm.firstName,
				lastName: userForm.lastName,
				email: userForm.email,
				pseudo: userForm.pseudo,
				// Pour le champ role, si aucune valeur n’est saisie, nous utilisons une valeur par défaut ("USER")
				role: userForm.role && userForm.role.trim() !== '' ? userForm.role : 'USER',
				classIds: userForm.classIds,
			},
		});
		showEditModal.value = false;
		await refetch();
		showNotification('Utilisateur mis à jour avec succès', 'success');
	} catch (error) {
		console.error('Error updating user:', error);
		showNotification("Échec de la mise à jour de l'utilisateur", 'error');
	} finally {
		updating.value = false;
	}
};

const deleteUser = async () => {
	if (!selectedUser.value) return;
	if (selectedUser.value.id === currentUserId.value) {
		showNotification(
			'Vous ne pouvez pas supprimer votre propre compte depuis cette page. Utilisez la page de profil à la place.',
			'error',
		);
		showDeleteModal.value = false;
		return;
	}
	deleting.value = true;
	try {
		await deleteUserMutation({ id: selectedUser.value.id });
		showDeleteModal.value = false;
		selectedUser.value = null;
		await refetch();
		showNotification('Utilisateur supprimé avec succès', 'success');
	} catch (error) {
		console.error('Error deleting user:', error);
		showNotification("Échec de la suppression de l'utilisateur", 'error');
	} finally {
		deleting.value = false;
	}
};

const refetchUsers = async () => {
	await refetch();
};

const showNotification = (message: string, type: 'success' | 'error') => {
	notification.value = { message, type };
	setTimeout(() => {
		notification.value = null;
	}, 3000);
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
	@apply text-violet-400 hover:text-violet-300;
}
</style>
