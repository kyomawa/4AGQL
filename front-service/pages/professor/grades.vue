<template>
	<div class="min-h-screen py-12 px-4 sm:px-6 lg:px-8 dark:to-violet-900">
		<div class="w-[830px] mx-auto">
			<Head>
				<title>Gestion des notes - SchoolInc</title>
			</Head>
			<h2 class="text-center title-primary">Gestion des notes</h2>

			<!-- Filtres -->
			<div class="mb-8 card p-6">
				<h3 class="text-lg font-semibold text-violet-700 dark:text-violet-300 mb-4">
					Filtres
				</h3>
				<div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
					<!-- Filtre par classe -->
					<div>
						<label class="label-primary dark:text-violet-300">Classe</label>
						<select
							v-model="classFilter"
							@change="fetchGradesByClass"
							class="input-primary dark:bg-violet-900 dark:text-white dark:border-violet-800"
						>
							<option value="">Toutes les classes</option>
							<option
								v-for="classObj in myClasses"
								:key="classObj.id"
								:value="classObj.id"
							>
								{{ classObj.name }}
							</option>
						</select>
					</div>
					<!-- Filtre par cours -->
					<div>
						<label class="label-primary dark:text-violet-300">Cours</label>
						<select
							v-model="courseFilter"
							class="input-primary dark:bg-violet-900 dark:text-white dark:border-violet-800"
						>
							<option value="">Tous les cours</option>
							<option
								v-for="course in uniqueCourses"
								:key="course"
								:value="course"
							>
								{{ course }}
							</option>
						</select>
					</div>
				</div>
				<!-- Filtre par étudiant -->
				<div class="mb-4">
					<label class="label-primary dark:text-violet-300">Étudiant</label>
					<select
						v-model="studentFilter"
						class="input-primary dark:bg-violet-900 dark:text-white dark:border-violet-800 w-full"
					>
						<option value="">Tous les étudiants</option>
						<option
							v-for="student in availableStudents"
							:key="student.id"
							:value="student.id"
						>
							{{ `${student.firstName} ${student.lastName}` }}
						</option>
					</select>
				</div>
				<div class="flex justify-end">
					<button
						@click="clearAllFilters"
						class="btn-primary bg-violet-800 hover:bg-violet-700"
					>
						Effacer tous les filtres
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
				<p class="mt-4 text-violet-300">Chargement des notes...</p>
			</div>

			<!-- Aucun résultat -->
			<div
				v-else-if="filteredGrades.length === 0"
				class="p-6 card text-center"
			>
				<h3 class="mt-4 text-lg font-medium text-violet-300">Aucune note trouvée</h3>
				<p class="mt-1 text-violet-300">
					{{
						classFilter || courseFilter
							? 'Essayez de modifier vos critères de recherche.'
							: 'Veuillez sélectionner une classe dans le filtre pour attribuer une note.'
					}}
				</p>
				<div
					v-if="classFilter"
					class="mt-4"
				>
					<button
						@click="openAddGradeModal"
						class="btn-primary"
					>
						Ajouter une note
					</button>
				</div>
			</div>

			<!-- Tableau des notes -->
			<div v-else>
				<div class="flex justify-end mb-4">
					<button
						v-if="classFilter"
						@click="openAddGradeModal"
						class="btn-primary"
					>
						Ajouter une note
					</button>
				</div>
				<div class="overflow-x-auto">
					<table
						class="min-w-full bg-white dark:bg-violet-950 rounded-lg overflow-hidden shadow-md"
					>
						<thead class="bg-violet-100 dark:bg-violet-900">
							<tr>
								<th
									class="py-3 px-4 text-left text-sm font-medium text-violet-800 dark:text-violet-200"
								>
									Étudiant
								</th>
								<th
									class="py-3 px-4 text-left text-sm font-medium text-violet-800 dark:text-violet-200"
								>
									Cours
								</th>
								<th
									class="py-3 px-4 text-left text-sm font-medium text-violet-800 dark:text-violet-200"
								>
									Commentaire
								</th>
								<th
									class="py-3 px-4 text-left text-sm font-medium text-violet-800 dark:text-violet-200"
								>
									Note
								</th>
								<th
									class="py-3 px-4 text-left text-sm font-medium text-violet-800 dark:text-violet-200"
								>
									Actions
								</th>
							</tr>
						</thead>
						<tbody class="divide-y divide-gray-200 dark:divide-violet-800">
							<tr
								v-for="grade in filteredGrades"
								:key="grade.id"
								class="hover:bg-violet-50 dark:hover:bg-violet-900"
							>
								<td class="py-3 px-4 text-sm text-gray-700 dark:text-gray-300">
									{{ grade.userNames || grade.userId }}
								</td>
								<td class="py-3 px-4 text-sm text-gray-700 dark:text-gray-300">
									{{ grade.course }}
								</td>
								<td class="py-3 px-4 text-sm text-gray-700 dark:text-gray-300">
									{{ grade.note }}
								</td>
								<td
									class="py-3 px-4 text-sm font-medium text-violet-600 dark:text-violet-400"
								>
									{{ grade.grade }}/100
								</td>
								<td class="py-3 px-4 text-sm flex space-x-2">
									<button
										@click="openEditGradeModal(grade)"
										class="text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300"
									>
										Modifier
									</button>
									<button
										@click="openDeleteGradeModal(grade)"
										class="text-red-600 hover:text-red-800 dark:text-red-400 dark:hover:text-red-300"
									>
										Supprimer
									</button>
								</td>
							</tr>
						</tbody>
					</table>
				</div>
			</div>
		</div>

		<!-- Modal Ajouter/Modifier une note -->
		<div
			v-if="showGradeModal"
			class="fixed inset-0 z-50 bg-black bg-opacity-50 flex items-center justify-center"
		>
			<div class="bg-white dark:bg-violet-950 rounded-lg shadow-xl p-6 w-full max-w-md">
				<h3 class="text-xl font-semibold text-violet-700 dark:text-violet-300 mb-4">
					{{ editingGrade ? 'Modifier la note' : 'Ajouter une note' }}
				</h3>
				<form @submit.prevent="validateAndSaveGrade">
					<!-- Champ Étudiant -->
					<div class="mb-4">
						<label class="label-primary dark:text-violet-300">Étudiant</label>
						<select
							v-model="gradeForm.userId"
							class="input-primary dark:bg-violet-900 dark:text-white dark:border-violet-800"
							required
						>
							<option
								value=""
								disabled
							>
								Sélectionnez un étudiant
							</option>
							<option
								v-for="student in allUsers"
								:key="student.id"
								:value="student.id"
							>
								{{ `${student.firstName} ${student.lastName}` }}
							</option>
						</select>
						<p
							v-if="studentNotInClassError"
							class="text-red-500 text-sm mt-1"
						>
							Cet étudiant n'est pas inscrit dans cette classe. Veuillez l'ajouter
							d'abord.
						</p>
					</div>
					<!-- Champ Cours -->
					<div class="mb-4">
						<label class="label-primary dark:text-violet-300">Cours</label>
						<input
							v-model="gradeForm.course"
							type="text"
							placeholder="Nom du cours"
							class="input-primary dark:bg-violet-900 dark:text-white dark:border-violet-800"
							required
						/>
					</div>
					<!-- Champ Commentaire -->
					<div class="mb-4">
						<label class="label-primary dark:text-violet-300">Commentaire</label>
						<textarea
							v-model="gradeForm.note"
							placeholder="Commentaire sur la note (min 20 caractères)"
							class="input-primary dark:bg-violet-900 dark:text-white dark:border-violet-800 min-h-[100px]"
							required
						></textarea>
					</div>
					<!-- Champ Note -->
					<div class="mb-6">
						<label class="label-primary dark:text-violet-300">Note (sur 100)</label>
						<input
							v-model="gradeForm.grade"
							type="number"
							min="10"
							max="100"
							step="1"
							placeholder="Note sur 100"
							class="input-primary dark:bg-violet-900 dark:text-white dark:border-violet-800"
							required
						/>
					</div>
					<div class="flex justify-end space-x-3">
						<button
							type="button"
							@click="closeGradeModal"
							class="btn-secondary border border-gray-300 dark:border-violet-700"
						>
							Annuler
						</button>
						<button
							type="submit"
							class="btn-primary"
						>
							{{ editingGrade ? 'Enregistrer' : 'Ajouter' }}
						</button>
					</div>
				</form>
			</div>
		</div>

		<!-- Modal de confirmation de suppression -->
		<div
			v-if="showDeleteModal"
			class="fixed inset-0 z-50 bg-black bg-opacity-50 flex items-center justify-center"
		>
			<div class="bg-white dark:bg-violet-950 rounded-lg shadow-xl p-6 w-full max-w-md">
				<h3 class="text-xl font-semibold text-red-600 dark:text-red-400 mb-4">
					Supprimer la note
				</h3>
				<p class="text-gray-700 dark:text-gray-300 mb-6">
					Êtes-vous sûr de vouloir supprimer cette note ? Cette action est irréversible.
				</p>
				<div class="flex justify-end space-x-3">
					<button
						@click="closeDeleteModal"
						class="btn-secondary border border-gray-300 dark:border-violet-700"
					>
						Annuler
					</button>
					<button
						@click="deleteGrade"
						class="bg-red-600 hover:bg-red-700 text-white px-4 py-2 rounded-lg"
					>
						Supprimer
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
import { ref, computed, onMounted, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { provideApolloClient, useQuery, useMutation } from '@vue/apollo-composable';
import gql from 'graphql-tag';
import { useNuxtApp } from '#app';
import { useAuth } from '~/composables/useAuth';

// Fournir les clients appropriés
const nuxtApp = useNuxtApp();
provideApolloClient(nuxtApp.$apolloClasses as any); // Pour les classes
const apolloGrades = nuxtApp.$apolloGrades as any;
const apolloUsers = nuxtApp.$apolloUsers as any;

// Router et état d'authentification
const route = useRoute();
const router = useRouter();
const { user } = useAuth();
const currentUserId = computed(() => user.value?.id || '');

// États pour les filtres
const classFilter = ref('');
const courseFilter = ref('');
const studentFilter = ref('');

// États pour les données
const loading = ref(true);
const grades = ref<any[]>([]);
const myClasses = ref<any[]>([]);
const allUsers = ref<any[]>([]);

// Pour gérer l'erreur d'étudiant non inscrit
const studentNotInClassError = ref(false);

// États pour les modals
const showGradeModal = ref(false);
const showDeleteModal = ref(false);
const editingGrade = ref<any>(null);
const gradeForm = ref({
	id: '',
	userId: '',
	professorId: '',
	classId: '',
	course: '',
	note: '',
	grade: 0,
});
const gradeToDelete = ref<any>(null);

// État pour les notifications
const notification = ref<{ type: 'success' | 'error'; message: string } | null>(null);

// --- Requêtes et mutations GraphQL ---

// Query pour récupérer les classes du professeur
const GET_MY_CLASSES_QUERY = gql`
	query GetClassById($userId: String!) {
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

// Query pour récupérer les notes d'une classe
const GET_GRADES_BY_CLASS_QUERY = gql`
	query GetGradesByClassId($classId: String!) {
		getGradesByClassId(classId: $classId) {
			id
			course
			note
			grade
			userId
			userNames
			professorNames
			professorId
			classId
			className
		}
	}
`;

// Mutation pour créer une note
const CREATE_GRADE_MUTATION = gql`
	mutation CreateGrade(
		$course: String!
		$note: String
		$grade: Float!
		$userId: String!
		$professorId: String!
		$classId: String!
	) {
		createGrade(
			grade: {
				course: $course
				note: $note
				grade: $grade
				userId: $userId
				professorId: $professorId
				classId: $classId
			}
		) {
			id
			course
			note
			grade
			userId
			professorId
			classId
		}
	}
`;

// Mutation pour mettre à jour une note sans utiliser GradeInput
const UPDATE_GRADE_MUTATION = gql`
	mutation UpdateGradeById($id: String!, $course: String!, $note: String!, $grade: Float!) {
		updateGradeById(id: $id, grade: { course: $course, note: $note, grade: $grade }) {
			id
			course
			note
			grade
			userId
			professorId
			classId
		}
	}
`;

// Mutation pour supprimer une note
const DELETE_GRADE_MUTATION = gql`
	mutation DeleteGradeById($id: String!) {
		deleteGradeById(id: $id) {
			id
			course
			note
			grade
			userId
			professorId
			classId
		}
	}
`;

// Récupérer les classes du professeur
const {
	result: classesResult,
	loading: classesLoading,
	refetch: refetchClasses,
} = useQuery(
	GET_MY_CLASSES_QUERY,
	() => ({ userId: currentUserId.value }),
	() => ({ enabled: !!currentUserId.value }),
);
watch(classesResult, (newResult) => {
	if (newResult?.getClassesByUserId) {
		myClasses.value = newResult.getClassesByUserId;
	}
});

// Récupérer tous les utilisateurs
provideApolloClient(apolloUsers);
const {
	result: usersResult,
	loading: usersLoading,
	refetch: refetchUsers,
} = useQuery(GET_USERS_QUERY);
watch(usersResult, (newResult) => {
	if (newResult?.getUsers) {
		allUsers.value = newResult.getUsers;
	}
});

// Fonction pour récupérer les notes d'une classe
async function fetchGradesByClass() {
	if (!classFilter.value) {
		grades.value = [];
		courseFilter.value = '';
		studentFilter.value = '';
		return;
	}
	loading.value = true;
	try {
		provideApolloClient(apolloGrades);
		const result = await apolloGrades.query({
			query: GET_GRADES_BY_CLASS_QUERY,
			variables: { classId: classFilter.value },
			fetchPolicy: 'network-only',
		});
		if (result.data?.getGradesByClassId) {
			grades.value = result.data.getGradesByClassId;
			// Mettre à jour les utilisateurs
			provideApolloClient(apolloUsers);
			await refetchUsers();
		} else {
			grades.value = [];
		}
	} catch (error) {
		console.error('Erreur lors du chargement des notes:', error);
		showNotification('Erreur lors du chargement des notes', 'error');
		grades.value = [];
	} finally {
		loading.value = false;
	}
}

// Mutations pour les notes
provideApolloClient(apolloGrades);
const { mutate: updateGrade } = useMutation(UPDATE_GRADE_MUTATION);
const { mutate: deleteGradeMutation } = useMutation(DELETE_GRADE_MUTATION);
const { mutate: createGrade } = useMutation(CREATE_GRADE_MUTATION);

// Regrouper l'état de chargement
watch([classesLoading, usersLoading], ([classLoading, userLoading]) => {
	loading.value = classLoading || userLoading;
});

// Cours uniques pour le filtre
const uniqueCourses = computed(() => {
	const courses = new Set<string>();
	grades.value.forEach((grade) => {
		if (grade.course) courses.add(grade.course);
	});
	return Array.from(courses);
});

// Étudiants disponibles pour la classe sélectionnée
const availableStudents = computed(() => {
	if (!classFilter.value) return [];
	const selectedClass = myClasses.value.find((c) => c.id === classFilter.value);
	if (!selectedClass) return [];
	return allUsers.value.filter(
		(user) =>
			grades.value.some((grade) => grade.userId === user.id) ||
			(selectedClass.userIds && selectedClass.userIds.includes(user.id)),
	);
});

// Grades filtrés par cours et par étudiant
const filteredGrades = computed(() => {
	let filtered = grades.value;
	if (courseFilter.value) {
		filtered = filtered.filter(
			(grade) => grade.course.toLowerCase() === courseFilter.value.toLowerCase(),
		);
	}
	if (studentFilter.value) {
		filtered = filtered.filter((grade) => grade.userId === studentFilter.value);
	}
	return filtered;
});

// Initialisation
onMounted(async () => {
	const urlClassId = route.query.classId as string;
	if (urlClassId) {
		classFilter.value = urlClassId;
	}
	provideApolloClient(apolloUsers);
	await refetchUsers();
	if (currentUserId.value) {
		provideApolloClient(nuxtApp.$apolloClasses);
		await refetchClasses();
		if (classFilter.value) {
			await fetchGradesByClass();
		} else {
			loading.value = false;
		}
	} else {
		const unwatch = watch(currentUserId, async (newUserId) => {
			if (newUserId) {
				provideApolloClient(nuxtApp.$apolloClasses);
				await refetchClasses();
				if (classFilter.value) {
					await fetchGradesByClass();
				} else {
					loading.value = false;
				}
				unwatch();
			}
		});
	}
});

// Méthodes de gestion des modals et des notes
function clearAllFilters() {
	classFilter.value = '';
	courseFilter.value = '';
	studentFilter.value = '';
	grades.value = [];
}

async function openAddGradeModal() {
	if (allUsers.value.length === 0) {
		provideApolloClient(apolloUsers);
		await refetchUsers();
	}
	editingGrade.value = null;
	gradeForm.value = {
		id: '',
		userId: '',
		professorId: currentUserId.value,
		classId: classFilter.value,
		course: '',
		note: '',
		grade: 0,
	};
	studentNotInClassError.value = false;
	showGradeModal.value = true;
}

function openEditGradeModal(grade: any) {
	editingGrade.value = grade;
	gradeForm.value = {
		id: grade.id,
		userId: grade.userId,
		professorId: grade.professorId,
		classId: grade.classId,
		course: grade.course,
		note: grade.note,
		grade: grade.grade,
	};
	showGradeModal.value = true;
}

function closeGradeModal() {
	showGradeModal.value = false;
}

function openDeleteGradeModal(grade: any) {
	gradeToDelete.value = grade;
	showDeleteModal.value = true;
}

function closeDeleteModal() {
	showDeleteModal.value = false;
	gradeToDelete.value = null;
}

async function saveGrade() {
	try {
		provideApolloClient(apolloGrades);
		if (editingGrade.value) {
			// Mise à jour d'une note existante
			await updateGrade({
				id: gradeForm.value.id,
				course: gradeForm.value.course,
				note: gradeForm.value.note,
				grade: parseFloat(gradeForm.value.grade.toString()),
			});
			showNotification('Note mise à jour avec succès', 'success');
		} else {
			// Création d'une nouvelle note
			await createGrade({
				course: gradeForm.value.course,
				note: gradeForm.value.note,
				grade: parseFloat(gradeForm.value.grade.toString()),
				userId: gradeForm.value.userId,
				professorId: currentUserId.value,
				classId: classFilter.value,
			});
			showNotification('Note ajoutée avec succès', 'success');
		}
		await fetchGradesByClass();
		courseFilter.value = '';
		studentFilter.value = '';
		provideApolloClient(apolloUsers);
		await refetchUsers();
		closeGradeModal();
	} catch (error) {
		console.error('Erreur lors de la sauvegarde:', error);
		showNotification('Erreur lors de la sauvegarde de la note', 'error');
	}
}

async function deleteGrade() {
	if (!gradeToDelete.value) return;
	try {
		provideApolloClient(apolloGrades);
		await deleteGradeMutation({ id: gradeToDelete.value.id });
		await fetchGradesByClass();
		showNotification('Note supprimée avec succès', 'success');
		closeDeleteModal();
	} catch (error) {
		console.error('Erreur lors de la suppression:', error);
		showNotification('Erreur lors de la suppression de la note', 'error');
	}
}

// Gestion des notifications
function showNotification(message: string, type: 'success' | 'error') {
	notification.value = { message, type };
	setTimeout(() => {
		notification.value = null;
	}, 3000);
}

// Vérifier si l'étudiant est inscrit dans la classe
function isStudentInClass(studentId: string): boolean {
	const selectedClass = myClasses.value.find((c) => c.id === classFilter.value);
	if (!selectedClass) return false;
	return selectedClass.userIds && selectedClass.userIds.includes(studentId);
}

// Validation et sauvegarde de note
async function validateAndSaveGrade() {
	studentNotInClassError.value = false;
	if (editingGrade.value) {
		await saveGrade();
		return;
	}
	if (!isStudentInClass(gradeForm.value.userId)) {
		studentNotInClassError.value = true;
		return;
	}
	if (!gradeForm.value.note || gradeForm.value.note.trim().length < 20) {
		showNotification('Le commentaire doit contenir au moins 20 caractères.', 'error');
		return;
	}
	const numericGrade = parseFloat(gradeForm.value.grade.toString());
	if (numericGrade < 10 || numericGrade > 100) {
		showNotification('La note doit être comprise entre 10 et 100.', 'error');
		return;
	}
	await saveGrade();
}

// Observateur pour les changements d'URL
watch(
	() => route.query,
	(newQuery) => {
		const urlClassId = newQuery.classId as string;
		if (urlClassId && urlClassId !== classFilter.value) {
			classFilter.value = urlClassId;
			fetchGradesByClass();
		}
	},
);
</script>

<style scoped>
.card {
	@apply bg-white shadow-lg rounded-2xl transition duration-300 dark:bg-violet-950;
}
.title-primary {
	@apply mb-6 text-2xl font-bold text-violet-700 dark:text-violet-300;
}
.label-primary {
	@apply block mb-1 text-sm font-medium text-gray-700 dark:text-violet-300;
}
.input-primary {
	@apply w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-violet-500 focus:border-transparent;
}
.btn-primary {
	@apply py-2 px-4 font-medium text-white transition-colors bg-violet-600 rounded-lg hover:bg-violet-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-violet-500 disabled:opacity-50 disabled:cursor-not-allowed;
}
.btn-secondary {
	@apply py-2 px-4 font-medium text-gray-700 transition-colors bg-white rounded-lg hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-violet-500 dark:text-gray-300 dark:bg-violet-900 dark:hover:bg-violet-800;
}
.link-primary {
	@apply text-violet-600 hover:text-violet-800 dark:text-violet-400 dark:hover:text-violet-300;
}
</style>
