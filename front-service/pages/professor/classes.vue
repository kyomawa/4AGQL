<template>
	<div class="min-h-screen py-12 px-4 sm:px-6 lg:px-8 dark:to-violet-900">
		<div class="w-[830px] mx-auto">
			<Head>
				<title>Mes classes - SchoolInc</title>
			</Head>
			<div class="flex justify-between items-center mb-6">
				<h2 class="title-primary">Mes classes enseignées</h2>
				<button
					@click="openCreateClassModal"
					class="btn-primary"
				>
					Créer une classe
				</button>
			</div>

			<!-- Barre de recherche stylisée (filtrage côté client) -->
			<div class="mb-6 mx-auto w-full">
				<label class="label-primary dark:text-violet-300"
					>Rechercher dans mes classes</label
				>
				<div class="flex w-full items-center">
					<input
						type="text"
						v-model="nameFilter"
						placeholder="Entrez le nom de la classe"
						class="input-primary dark:bg-violet-900 dark:text-white dark:border-violet-800 flex-1"
					/>
					<button
						@click="filterClasses"
						class="btn-primary ml-3 px-6"
					>
						Rechercher
					</button>
					<button
						v-if="isFiltered"
						@click="clearFilter"
						class="btn-primary ml-2 bg-violet-800 hover:bg-violet-700 px-6"
					>
						Effacer
					</button>
				</div>
			</div>

			<!-- État de chargement -->
			<div
				v-if="loading"
				class="p-6 card text-center"
			>
				<div
					class="w-12 h-12 mx-auto border-b-2 rounded-full border-violet-500 animate-spin"
				></div>
				<p class="mt-4 text-violet-300">Chargement de vos classes...</p>
			</div>

			<!-- Aucun résultat -->
			<div
				v-else-if="filteredClasses.length === 0"
				class="p-6 card text-center"
			>
				<h3 class="mt-4 text-lg font-medium text-violet-300">Aucune classe trouvée</h3>
				<p class="mt-1 text-violet-300">
					{{
						isFiltered
							? 'Essayez de modifier vos critères de recherche.'
							: "Vous n'enseignez aucune classe pour le moment."
					}}
				</p>
				<button
					@click="openCreateClassModal"
					class="btn-primary mt-4"
				>
					Créer votre première classe
				</button>
			</div>

			<!-- Grille des classes -->
			<div
				v-else
				class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6"
			>
				<div
					v-for="classItem in filteredClasses"
					:key="classItem.id"
					class="card p-6 transition duration-300 hover:shadow-lg flex flex-col justify-between"
				>
					<div class="flex justify-between items-start mb-2">
						<h3 class="text-xl font-semibold text-violet-700 dark:text-violet-300">
							{{ classItem.name }}
						</h3>
						<button
							@click="openUpdateClassModal(classItem)"
							class="text-violet-500 hover:text-violet-400"
						>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								width="16"
								height="16"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
								stroke-linecap="round"
								stroke-linejoin="round"
							>
								<path
									d="M17 3a2.85 2.85 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5L17 3z"
								></path>
							</svg>
						</button>
					</div>

					<div class="flex flex-col items-center text-center mt-2">
						<p class="text-gray-600 dark:text-gray-400 text-sm">
							{{ classItem.userIds?.length || 0 }} étudiant(s) inscrit(s)
						</p>
					</div>

					<div class="mt-4 flex flex-col space-y-2">
						<div class="flex justify-between space-x-2">
							<NuxtLink
								to="/professor/grades"
								class="btn-primary text-center text-sm py-1 px-2 flex-1"
							>
								Voir les notes
							</NuxtLink>
							<button
								@click="openStudentManagementModal(classItem)"
								class="btn-secondary text-center text-sm py-1 px-2 flex-1"
							>
								Gérer les étudiants
							</button>
						</div>
						<button
							@click="confirmDeleteClass(classItem)"
							class="btn-primary bg-red-600 hover:bg-red-700 text-center text-sm py-1 px-2"
						>
							Supprimer la classe
						</button>
					</div>
				</div>
			</div>
		</div>

		<!-- Modal de création de classe -->
		<div
			v-if="showCreateClassModal"
			class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50"
		>
			<div class="w-full max-w-md p-6 card dark:bg-violet-950">
				<h3 class="mb-4 text-xl font-bold text-violet-300">Créer une nouvelle classe</h3>
				<form
					@submit.prevent="createClass"
					class="space-y-4"
				>
					<div>
						<label class="label-primary dark:text-violet-300">Nom de la classe</label>
						<input
							v-model="classForm.name"
							type="text"
							placeholder="Ex: Mathématiques 101"
							class="input-primary dark:bg-violet-900 dark:text-white dark:border-violet-800"
							required
						/>
					</div>
					<div class="flex justify-end pt-4 space-x-3">
						<button
							type="button"
							@click="showCreateClassModal = false"
							class="btn-primary bg-violet-800 hover:bg-violet-700"
						>
							Annuler
						</button>
						<button
							type="submit"
							class="btn-primary"
							:disabled="creatingClass"
						>
							{{ creatingClass ? 'Création...' : 'Créer' }}
						</button>
					</div>
				</form>
			</div>
		</div>

		<!-- Modal de mise à jour de classe -->
		<div
			v-if="showUpdateClassModal"
			class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50"
		>
			<div class="w-full max-w-md p-6 card dark:bg-violet-950">
				<h3 class="mb-4 text-xl font-bold text-violet-300">Modifier la classe</h3>
				<form
					@submit.prevent="updateClass"
					class="space-y-4"
				>
					<div>
						<label class="label-primary dark:text-violet-300">Nom de la classe</label>
						<input
							v-model="updateClassForm.name"
							type="text"
							placeholder="Nouveau nom de la classe"
							class="input-primary dark:bg-violet-900 dark:text-white dark:border-violet-800"
							required
						/>
					</div>
					<div class="flex justify-end pt-4 space-x-3">
						<button
							type="button"
							@click="showUpdateClassModal = false"
							class="btn-primary bg-violet-800 hover:bg-violet-700"
						>
							Annuler
						</button>
						<button
							type="submit"
							class="btn-primary"
							:disabled="updatingClass"
						>
							{{ updatingClass ? 'Mise à jour...' : 'Mettre à jour' }}
						</button>
					</div>
				</form>
			</div>
		</div>

		<!-- Modal de gestion des étudiants -->
		<div
			v-if="showStudentManagementModal"
			class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50"
		>
			<div class="w-full max-w-lg p-6 card dark:bg-violet-950 max-h-[80vh] overflow-y-auto">
				<h3 class="mb-4 text-xl font-bold text-violet-300">
					Gestion des étudiants - {{ selectedClass?.name }}
				</h3>

				<!-- Ajouter un étudiant -->
				<div class="mb-6 p-4 bg-violet-900 rounded-lg">
					<h4 class="mb-2 font-semibold text-violet-300">Ajouter un étudiant</h4>
					<div class="flex space-x-2">
						<select
							v-model="selectedStudentId"
							class="input-primary dark:bg-violet-800 dark:text-white dark:border-violet-700 flex-1"
						>
							<option
								value=""
								disabled
							>
								Sélectionnez un étudiant
							</option>
							<option
								v-for="student in availableStudents"
								:key="student.id"
								:value="student.id"
							>
								{{ student.firstName }} {{ student.lastName }} ({{ student.email }})
							</option>
						</select>
						<button
							@click="enrollStudent"
							class="btn-primary"
							:disabled="!selectedStudentId || enrollingStudent"
						>
							{{ enrollingStudent ? '...' : 'Ajouter' }}
						</button>
					</div>
				</div>

				<!-- Liste des étudiants inscrits -->
				<div>
					<h4 class="mb-2 font-semibold text-violet-300">Étudiants inscrits</h4>
					<div
						v-if="!selectedClass?.userIds?.length"
						class="text-center p-4 bg-violet-900 rounded-lg"
					>
						<p class="text-violet-300">Aucun étudiant inscrit dans cette classe</p>
					</div>
					<div
						v-else
						class="space-y-2"
					>
						<div
							v-for="userId in selectedClass.userIds"
							:key="userId"
							class="flex justify-between items-center p-3 bg-violet-900 rounded-lg"
						>
							<div>
								<p class="text-white font-medium">
									{{ getStudentName(userId) }}
								</p>
								<p class="text-sm text-violet-300">
									{{ getStudentEmail(userId) }}
								</p>
							</div>
							<button
								@click="confirmRemoveStudent(userId)"
								class="text-red-400 hover:text-red-300"
								:disabled="removingStudent"
							>
								{{ isRemovingStudent(userId) ? '...' : 'Retirer' }}
							</button>
						</div>
					</div>
				</div>

				<div class="flex justify-end mt-6">
					<button
						@click="showStudentManagementModal = false"
						class="btn-primary bg-violet-800 hover:bg-violet-700"
					>
						Fermer
					</button>
				</div>
			</div>
		</div>

		<!-- Modal de confirmation de suppression d'étudiant -->
		<div
			v-if="showRemoveStudentModal"
			class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50"
		>
			<div class="w-full max-w-md p-6 card dark:bg-violet-950">
				<h3 class="mb-4 text-xl font-bold text-red-500">Retirer l'étudiant</h3>
				<p class="mb-6 text-violet-200">
					Êtes-vous sûr de vouloir retirer cet étudiant de la classe ?
					<br />
					<span class="font-medium">{{ getStudentName(studentToRemove) }}</span>
				</p>
				<div class="flex justify-end space-x-3">
					<button
						@click="showRemoveStudentModal = false"
						class="btn-primary bg-violet-800 hover:bg-violet-700"
					>
						Annuler
					</button>
					<button
						@click="removeStudent"
						class="btn-primary bg-red-600 hover:bg-red-700"
						:disabled="removingStudent"
					>
						{{ removingStudent ? 'Suppression...' : 'Retirer' }}
					</button>
				</div>
			</div>
		</div>

		<!-- Modal de confirmation de suppression de classe -->
		<div
			v-if="showDeleteClassModal"
			class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50"
		>
			<div class="w-full max-w-md p-6 card dark:bg-violet-950">
				<h3 class="mb-4 text-xl font-bold text-red-500">Supprimer la classe</h3>
				<p class="mb-6 text-violet-200">
					Êtes-vous sûr de vouloir supprimer définitivement cette classe ?
					<br />
					<span class="font-medium">{{ classToDelete?.name }}</span>
					<br /><br />
					<span class="text-red-300">
						Cette action est irréversible et supprimera également toutes les notes
						associées.
					</span>
					<br /><br />
					<span class="text-yellow-300 font-medium">
						Important : Vous devez d'abord retirer tous les étudiants inscrits et
						supprimer toutes les notes associées avant de pouvoir supprimer cette
						classe.
					</span>
				</p>
				<div class="flex justify-end space-x-3">
					<button
						@click="showDeleteClassModal = false"
						class="btn-primary bg-violet-800 hover:bg-violet-700"
					>
						Annuler
					</button>
					<button
						@click="deleteClass"
						class="btn-primary bg-red-600 hover:bg-red-700"
						:disabled="
							deletingClass ||
							(classToDelete &&
								classToDelete.userIds &&
								classToDelete.userIds.length > 0)
						"
					>
						{{ deletingClass ? 'Suppression...' : 'Supprimer' }}
					</button>
				</div>
			</div>
		</div>
		<!-- Notification -->
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
import { useRoute } from 'vue-router';
import { provideApolloClient, useQuery, useMutation } from '@vue/apollo-composable';
import gql from 'graphql-tag';
import { useNuxtApp } from '#app';
import { useAuth } from '~/composables/useAuth';

// Récupérer les clients Apollo
const nuxtApp = useNuxtApp();

// Récupérer l'identifiant de l'utilisateur connecté (professeur)
const { user } = useAuth();
const currentUserId = computed(() => user.value?.id || '');

// Variables pour la recherche (filtrage côté client)
const nameFilter = ref('');
const isFiltered = ref(false);

// États de chargement et stockage des classes récupérées
const loading = ref(true);
const classes = ref<any[]>([]);
const allUsers = ref<any[]>([]);

// États pour les modals
const showCreateClassModal = ref(false);
const showStudentManagementModal = ref(false);
const showRemoveStudentModal = ref(false);
const showUpdateClassModal = ref(false);
const showDeleteClassModal = ref(false);

// États pour les actions
const creatingClass = ref(false);
const selectedClass = ref<any>(null);
const classToDelete = ref<any>(null);
const selectedStudentId = ref('');
const studentToRemove = ref<string>('');
const enrollingStudent = ref(false);
const removingStudent = ref(false);
const removingStudentIds = ref<string[]>([]);
const updatingClass = ref(false);
const deletingClass = ref(false);

// État pour les notifications
const notification = ref<{ type: 'success' | 'error'; message: string } | null>(null);

// Formulaire pour créer une classe
const classForm = reactive({
	name: '',
});

// Formulaire pour mettre à jour une classe
const updateClassForm = reactive({
	id: '',
	name: '',
});

// Query pour récupérer les classes du professeur
const GET_CLASSES_BY_USERID_QUERY = gql`
	query GetClassesByUserId($userId: String!) {
		getClassesByUserId(userId: $userId) {
			id
			name
			creatorId
			userIds
		}
	}
`;

// Query pour récupérer tous les utilisateurs
const GET_USERS_QUERY = gql`
	query GetUsers {
		getUsers {
			id
			firstName
			lastName
			pseudo
			email
			classIds
		}
	}
`;

// Mutation pour créer une classe
const CREATE_CLASS_MUTATION = gql`
	mutation CreateClass($name: String!, $creatorId: String!) {
		createClass(class: { name: $name, creatorId: $creatorId }) {
			id
			name
			creatorId
			userIds
		}
	}
`;

// Mutation pour mettre à jour une classe
const UPDATE_CLASS_MUTATION = gql`
	mutation UpdateClassById($id: String!, $name: String!) {
		updateClassById(id: $id, class: { name: $name }) {
			id
			name
			creatorId
			userIds
		}
	}
`;

// Mutation pour supprimer une classe
const DELETE_CLASS_MUTATION = gql`
	mutation DeleteClassById($id: String!) {
		deleteClassById(id: $id) {
			id
			name
			creatorId
			userIds
		}
	}
`;

// Mutation pour inscrire un étudiant
const ENROLL_STUDENT_MUTATION = gql`
	mutation EnrollStudent($classId: String!, $studentId: String!) {
		enrollStudent(classId: $classId, studentId: $studentId) {
			id
			name
			creatorId
			userIds
		}
	}
`;

// Mutation pour retirer un étudiant
const REMOVE_STUDENT_MUTATION = gql`
	mutation RemoveStudent($classId: String!, $studentId: String!) {
		removeStudent(classId: $classId, studentId: $studentId) {
			id
			name
			creatorId
			userIds
		}
	}
`;

// Fournir le client Apollo pour les classes
provideApolloClient(nuxtApp.$apolloClasses as any);

// Récupérer les classes
const {
	result: classesResult,
	loading: classesLoading,
	refetch: refetchClasses,
} = useQuery(
	GET_CLASSES_BY_USERID_QUERY,
	() => ({ userId: currentUserId.value }),
	() => ({ enabled: !!currentUserId.value, fetchPolicy: 'network-only' }),
);

// Setup des mutations
const { mutate: createClassMutation } = useMutation(CREATE_CLASS_MUTATION);
const { mutate: updateClassMutation } = useMutation(UPDATE_CLASS_MUTATION);
const { mutate: deleteClassMutation } = useMutation(DELETE_CLASS_MUTATION);
const { mutate: enrollStudentMutation } = useMutation(ENROLL_STUDENT_MUTATION);
const { mutate: removeStudentMutation } = useMutation(REMOVE_STUDENT_MUTATION);

const route = useRoute();

// Observer les changements dans les classes
watch(classesResult, (newResult) => {
	if (newResult?.getClassesByUserId) {
		classes.value = newResult.getClassesByUserId;
	}
});

watch(classesLoading, (isLoading) => {
	loading.value = isLoading;
});

// Filtrage côté client par nom de classe
const filteredClasses = computed(() => {
	if (!nameFilter.value.trim() || !isFiltered.value) {
		return classes.value;
	}

	const filter = nameFilter.value.toLowerCase();
	return classes.value.filter((classItem) => classItem.name.toLowerCase().includes(filter));
});

// Étudiants disponibles (ceux qui ne sont pas déjà dans la classe)
const availableStudents = computed(() => {
	if (!selectedClass.value || !allUsers.value.length) return [];

	// Filtrer pour exclure ceux déjà inscrits dans la classe
	return allUsers.value.filter((student) => !selectedClass.value.userIds.includes(student.id));
});

// Vérifier si un étudiant est en cours de suppression
const isRemovingStudent = (studentId: string) => {
	return removingStudentIds.value.includes(studentId);
};

// Obtenir le nom d'un étudiant à partir de son ID
const getStudentName = (studentId: string) => {
	const student = allUsers.value.find((u) => u.id === studentId);
	if (student) {
		return `${student.firstName} ${student.lastName}`;
	}
	return studentId;
};

// Obtenir l'email d'un étudiant à partir de son ID
const getStudentEmail = (studentId: string) => {
	const student = allUsers.value.find((u) => u.id === studentId);
	if (student) {
		return student.email;
	}
	return '';
};

// Récupérer les utilisateurs depuis Apollo Users
const fetchUsers = async () => {
	try {
		provideApolloClient(nuxtApp.$apolloUsers as any);

		const result = await nuxtApp.$apolloUsers.query({
			query: GET_USERS_QUERY,
			fetchPolicy: 'network-only',
		});

		if (result.data && result.data.getUsers) {
			allUsers.value = result.data.getUsers;
		}

		// Revenir au client Apollo pour les classes
		provideApolloClient(nuxtApp.$apolloClasses as any);
	} catch (error) {
		console.error('Erreur lors de la récupération des utilisateurs:', error);
		showNotification('Erreur lors du chargement des étudiants', 'error');
	}
};

// Initialisation
onMounted(async () => {
	if (currentUserId.value) {
		// Fournir le client Apollo pour les classes
		provideApolloClient(nuxtApp.$apolloClasses as any);

		// Récupérer les classes du professeur
		await refetchClasses();

		// Récupérer les utilisateurs pour la gestion des étudiants
		await fetchUsers();
	} else {
		loading.value = false;
	}
});

// Observer les changements de route
watch(
	() => route.path,
	async (newPath) => {
		if (newPath === '/professor/classes') {
			provideApolloClient(nuxtApp.$apolloClasses as any);
			await refetchClasses();
		}
	},
);

// Observer les changements d'ID utilisateur
watch(
	() => currentUserId.value,
	async (newId, oldId) => {
		if (newId && newId !== oldId) {
			console.log('ID utilisateur changé, recharger les classes:', newId);
			provideApolloClient(nuxtApp.$apolloClasses as any);
			await refetchClasses();
		}
	},
);

// Filtrage côté client
const filterClasses = () => {
	if (nameFilter.value.trim()) {
		isFiltered.value = true;
	}
};

const clearFilter = () => {
	nameFilter.value = '';
	isFiltered.value = false;
};

// Ouverture du modal de création de classe
const openCreateClassModal = () => {
	classForm.name = '';
	showCreateClassModal.value = true;
};

// Création d'une nouvelle classe
const createClass = async () => {
	if (!classForm.name.trim()) return;

	creatingClass.value = true;

	try {
		provideApolloClient(nuxtApp.$apolloClasses as any);

		await createClassMutation({
			name: classForm.name,
			creatorId: currentUserId.value,
		});

		// Rafraîchir les données
		await refetchClasses();
		showCreateClassModal.value = false;
		showNotification('Classe créée avec succès', 'success');
	} catch (error) {
		console.error('Erreur lors de la création de la classe:', error);
		showNotification('Erreur lors de la création de la classe', 'error');
	} finally {
		creatingClass.value = false;
	}
};

// Ouverture du modal de mise à jour de classe
const openUpdateClassModal = (classItem: any) => {
	updateClassForm.id = classItem.id;
	updateClassForm.name = classItem.name;
	showUpdateClassModal.value = true;
};

// Mise à jour d'une classe
const updateClass = async () => {
	if (!updateClassForm.name.trim()) return;

	updatingClass.value = true;

	try {
		provideApolloClient(nuxtApp.$apolloClasses as any);

		await updateClassMutation({
			id: updateClassForm.id,
			name: updateClassForm.name,
		});

		// Rafraîchir les données
		await refetchClasses();
		showUpdateClassModal.value = false;
		showNotification('Classe mise à jour avec succès', 'success');
	} catch (error) {
		console.error('Erreur lors de la mise à jour de la classe:', error);
		showNotification('Erreur lors de la mise à jour de la classe', 'error');
	} finally {
		updatingClass.value = false;
	}
};

// Ouverture du modal de confirmation de suppression de classe
const confirmDeleteClass = (classItem: any) => {
	classToDelete.value = classItem;
	showDeleteClassModal.value = true;
};

// Suppression d'une classe
// Suppression d'une classe
const deleteClass = async () => {
	if (!classToDelete.value) return;

	deletingClass.value = true;

	try {
		provideApolloClient(nuxtApp.$apolloClasses as any);

		await deleteClassMutation({
			id: classToDelete.value.id,
		});

		// Rafraîchir les données
		await refetchClasses();
		showDeleteClassModal.value = false;
		showNotification('Classe supprimée avec succès', 'success');
	} catch (error: any) {
		// Typer l'erreur comme 'any'
		console.error('Erreur lors de la suppression de la classe:', error);

		// Vérifier si l'erreur est due à des étudiants inscrits
		if (
			error.message &&
			typeof error.message === 'string' &&
			error.message.includes('Cannot delete a class with enrolled students')
		) {
			showNotification(
				"Impossible de supprimer cette classe. Veuillez d'abord retirer tous les étudiants inscrits et toutes les notes associées.",
				'error',
			);
		} else {
			showNotification('Erreur lors de la suppression de la classe', 'error');
		}
	} finally {
		deletingClass.value = false;
		classToDelete.value = null;
	}
};
// Ouverture du modal de gestion des étudiants
const openStudentManagementModal = async (classItem: any) => {
	selectedClass.value = classItem;
	selectedStudentId.value = '';

	// Rafraîchir la liste des utilisateurs avant d'ouvrir le modal
	await fetchUsers();

	showStudentManagementModal.value = true;
};

// Inscription d'un étudiant
const enrollStudent = async () => {
	if (!selectedClass.value || !selectedStudentId.value) return;

	enrollingStudent.value = true;

	try {
		provideApolloClient(nuxtApp.$apolloClasses as any);

		await enrollStudentMutation({
			classId: selectedClass.value.id,
			studentId: selectedStudentId.value,
		});

		// Mettre à jour la classe sélectionnée et rafraîchir la liste
		await refetchClasses();

		// Mettre à jour le selectedClass avec les données actualisées
		if (classesResult.value?.getClassesByUserId) {
			selectedClass.value = classesResult.value.getClassesByUserId.find(
				(c: any) => c.id === selectedClass.value.id,
			);
		}

		selectedStudentId.value = '';
		showNotification('Étudiant inscrit avec succès', 'success');
	} catch (error) {
		console.error("Erreur lors de l'inscription de l'étudiant:", error);
		showNotification("Erreur lors de l'inscription de l'étudiant", 'error');
	} finally {
		enrollingStudent.value = false;
	}
};

// Confirmation de suppression d'un étudiant
const confirmRemoveStudent = (userId: string) => {
	studentToRemove.value = userId;
	showRemoveStudentModal.value = true;
};

// Retrait d'un étudiant
const removeStudent = async () => {
	if (!selectedClass.value || !studentToRemove.value) return;

	removingStudent.value = true;
	removingStudentIds.value.push(studentToRemove.value);

	try {
		provideApolloClient(nuxtApp.$apolloClasses as any);

		await removeStudentMutation({
			classId: selectedClass.value.id,
			studentId: studentToRemove.value,
		});

		// Mettre à jour la classe sélectionnée et rafraîchir la liste
		await refetchClasses();

		// Mettre à jour le selectedClass avec les données actualisées
		if (classesResult.value?.getClassesByUserId) {
			selectedClass.value = classesResult.value.getClassesByUserId.find(
				(c: any) => c.id === selectedClass.value.id,
			);
		}

		showRemoveStudentModal.value = false;
		studentToRemove.value = '';
		showNotification('Étudiant retiré avec succès', 'success');
	} catch (error) {
		console.error("Erreur lors du retrait de l'étudiant:", error);
		showNotification("Erreur lors du retrait de l'étudiant", 'error');
	} finally {
		removingStudent.value = false;
		removingStudentIds.value = removingStudentIds.value.filter(
			(id) => id !== studentToRemove.value,
		);
	}
};
// Afficher une notification
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
	@apply text-2xl font-bold text-violet-700 dark:text-violet-300;
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

.btn-secondary {
	@apply py-2 px-4 font-medium text-gray-700 transition-colors bg-white rounded-lg hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-violet-500 dark:text-gray-300 dark:bg-violet-900 dark:hover:bg-violet-800 disabled:opacity-50 disabled:cursor-not-allowed;
}

.link-primary {
	@apply text-violet-600 hover:text-violet-800 dark:text-violet-400 dark:hover:text-violet-300;
}
</style>
