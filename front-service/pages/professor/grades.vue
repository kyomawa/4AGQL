<template>
	<div class="min-h-screen py-12">
		<h1 class="page-title">Gestion des Notes</h1>

		<div v-if="loading" class="py-8 text-center">
			<div class="w-12 h-12 mx-auto border-b-2 rounded-full border-violet-500 animate-spin"></div>
			<p class="mt-4 text-violet-300">Chargement des notes...</p>
		</div>

		<div v-else class="max-w-5xl mx-auto">
			<!-- Search/Filter -->
			<div class="filter-container">
				<label class="filter-label">Filtrer par Matière</label>
				<div class="flex items-center space-x-3">
					<div class="relative flex-1">
						<select v-model="courseFilter" @change="isFiltered = !!courseFilter"
							class="filter-input w-full appearance-none pr-8">
							<option value="">Toutes les matières</option>
							<option v-for="course in professorCourses" :key="course" :value="course">
								{{ course }}
							</option>
						</select>
						<div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-3 text-white">
							<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
							</svg>
						</div>
					</div>
					<button v-if="isFiltered" @click="clearFilter" class="clear-button">
						Réinitialiser
					</button>
				</div>
			</div>

			<!-- Notes Grid -->
			<div v-if="typedGrades.length === 0" class="py-8 text-center shadow-md bg-violet-950 rounded-2xl">
				<svg xmlns="http://www.w3.org/2000/svg" class="w-16 h-16 mx-auto text-violet-500" fill="none"
					viewBox="0 0 24 24" stroke="currentColor">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
						d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
				</svg>
				<h3 class="mt-4 text-lg font-medium text-violet-200">Aucune note trouvée</h3>
				<p class="mt-1 text-violet-300">
					{{ isFiltered ? "Essayez de modifier vos critères de recherche." : "Aucune note attribuée pour le moment." }}
				</p>
			</div>

			<div v-else class="space-y-4">
				<div v-for="grade in typedGrades" :key="grade.id"
					class="overflow-hidden transition shadow-md bg-violet-950 rounded-2xl hover:shadow-lg">
					<div class="p-5 flex justify-between items-center">
						<div class="flex flex-col md:flex-row md:items-center gap-3">
							<span class="text-white font-medium">{{ getStudentName(grade.user_id) }}</span>
							<span class="text-violet-300">{{ grade.course }}</span>
							<span class="px-3 py-1 bg-green-100 text-green-700 rounded-full font-medium inline-block">
								Note : {{ grade.value }}/20
							</span>
						</div>
						<div class="flex space-x-2">
							<button @click="editGrade(grade)"
								class="p-2 bg-blue-600 hover:bg-blue-700 text-white rounded-full transition">
								<svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
									<path d="M17.414 2.586a2 2 0 00-2.828 0L7 10.172V13h2.828l7.586-7.586a2 2 0 000-2.828z" />
									<path fill-rule="evenodd"
										d="M2 16V4a2 2 0 012-2h4a1 1 0 010 2H4v12h12v-4a1 1 0 112 0v4a2 2 0 01-2 2H4a2 2 0 01-2-2z"
										clip-rule="evenodd" />
								</svg>
							</button>
							<button @click="confirmDeleteGrade(grade)"
								class="p-2 bg-red-600 hover:bg-red-700 text-white rounded-full transition">
								<svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
									<path fill-rule="evenodd"
										d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v6a1 1 0 102 0V8a1 1 0 00-1-1z"
										clip-rule="evenodd" />
								</svg>
							</button>
						</div>
					</div>
				</div>
			</div>

			<!-- Floating Add Button -->
			<button @click="openAddNoteModal" aria-label="Ajouter une note"
				class="fixed bottom-10 right-10 w-14 h-14 bg-violet-600 hover:bg-violet-500 text-white rounded-full shadow-lg flex items-center justify-center transition-colors duration-200">
				<svg xmlns="http://www.w3.org/2000/svg" class="h-7 w-7" fill="none" viewBox="0 0 24 24" stroke="currentColor">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
				</svg>
			</button>
		</div>

		<!-- Modal Ajouter/Éditer Note -->
		<div v-if="showCreateModal || showEditModal"
			class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
			<div class="bg-white dark:bg-violet-900 rounded-2xl shadow-xl w-full max-w-md p-6">
				<h2 class="text-xl font-semibold text-violet-700 dark:text-violet-200 mb-6">
					{{ showEditModal ? 'Modifier' : 'Ajouter' }} une Note
				</h2>

				<form @submit.prevent="submitGradeForm" class="space-y-4">
					<div>
						<label class="block text-sm font-medium text-gray-700 dark:text-violet-300 mb-2">
							Cours
						</label>
						<select v-model="gradeForm.course" required
							class="w-full px-4 py-2 border border-gray-300 dark:border-violet-800 rounded-lg focus:outline-none focus:ring-2 focus:ring-violet-500 dark:bg-violet-950 text-gray-900 dark:text-white">
							<option value="" disabled selected>Sélectionnez un cours</option>
							<option v-for="course in professorCourses" :key="course" :value="course">
								{{ course }}
							</option>
						</select>
					</div>

					<div>
						<label class="block text-sm font-medium text-gray-700 dark:text-violet-300 mb-2">
							Étudiant à noter
						</label>
						<select v-model="gradeForm.user_id" required
							class="w-full px-4 py-2 border border-gray-300 dark:border-violet-800 rounded-lg focus:outline-none focus:ring-2 focus:ring-violet-500 dark:bg-violet-950 text-gray-900 dark:text-white">
							<option value="" disabled selected>Sélectionnez un étudiant</option>
							<option v-for="student in filteredStudents" :key="student.id" :value="student.id">
								{{ student.pseudo || student.email }}
							</option>
						</select>
					</div>

					<div>
						<label class="block text-sm font-medium text-gray-700 dark:text-violet-300 mb-2">
							Note
						</label>
						<input v-model.number="gradeForm.value" type="number" min="0" max="20" required
							class="w-full px-4 py-2 border border-gray-300 dark:border-violet-800 rounded-lg focus:outline-none focus:ring-2 focus:ring-violet-500 dark:bg-violet-950 text-gray-900 dark:text-white"
							placeholder="Note sur 20" />
					</div>

					<div class="flex justify-end space-x-4 pt-4">
						<button type="button" @click="cancelGradeForm"
							class="px-4 py-2 bg-gray-200 dark:bg-violet-800 text-gray-700 dark:text-white rounded-lg hover:bg-gray-300 dark:hover:bg-violet-700">
							Annuler
						</button>
						<button type="submit" class="px-4 py-2 bg-violet-600 hover:bg-violet-700 text-white rounded-lg">
							{{ showEditModal ? 'Modifier' : 'Ajouter' }}
						</button>
					</div>
				</form>
			</div>
		</div>

		<!-- Modal de Confirmation de Suppression -->
		<div v-if="showDeleteModal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
			<div class="bg-white dark:bg-violet-900 rounded-2xl shadow-xl w-full max-w-md p-6">
				<h2 class="text-xl font-semibold text-red-600 mb-4">
					Confirmer la Suppression
				</h2>
				<p class="mb-6 text-gray-700 dark:text-violet-200">
					Êtes-vous sûr de vouloir supprimer cette note ?
				</p>
				<div class="flex justify-end space-x-4">
					<button @click="showDeleteModal = false"
						class="px-4 py-2 bg-gray-200 dark:bg-violet-800 text-gray-700 dark:text-white rounded-lg hover:bg-gray-300 dark:hover:bg-violet-700">
						Annuler
					</button>
					<button @click="deleteGrade" class="px-4 py-2 bg-red-600 hover:bg-red-700 text-white rounded-lg">
						Supprimer
					</button>
				</div>
			</div>
		</div>

		<!-- Alert for errors -->
		<Transition name="error-alert">
			<div v-if="globalError"
				class="fixed top-4 left-1/2 transform -translate-x-1/2 z-50 max-w-md w-full bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded"
				role="alert">
				<div class="flex items-center">
					<svg class="w-6 h-6 mr-4 text-red-500" fill="currentColor" viewBox="0 0 20 20">
						<path fill-rule="evenodd"
							d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
							clip-rule="evenodd" />
					</svg>
					<p class="flex-1">{{ globalError }}</p>
					<button @click="globalError = null" class="ml-4 text-red-700 hover:text-red-900">
						<svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
							<path fill-rule="evenodd"
								d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
								clip-rule="evenodd" />
						</svg>
					</button>
				</div>
			</div>
		</Transition>
	</div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, watch } from 'vue';
import { useQuery, useMutation } from '@vue/apollo-composable';
import gql from 'graphql-tag';
import { useCurrentUser } from '~/composables/useCurrentUser';
import { useMockData } from '~/composables/useMockData';
import type { Grade } from '~/types/user';

// Types
interface ExtendedUser {
	id: string;
	email: string;
	pseudo: string;
	role: string;
	user_id?: string;
}

interface Student {
	id: string;
	email: string;
	pseudo: string;
	role: string;
}

// Setup
definePageMeta({
	middleware: ['auth', 'role'],
	role: 'professor',
});

// États
const { currentUser } = useCurrentUser();
const { users, classes, grades: mockGrades } = useMockData();
const loading = ref(true);
const grades = ref<Grade[]>([]);
const courseFilter = ref('');
const isFiltered = ref(false);
const globalError = ref<string | null>(null);

// Modals
const showCreateModal = ref(false);
const showEditModal = ref(false);
const showDeleteModal = ref(false);

// Formulaire
const gradeForm = reactive<Grade>({
	id: '',
	user_id: '',
	course: '',
	value: 0,
	professor_id: '',
});

const selectedGrade = ref<Grade | null>(null);

// Computed
const typedGrades = computed(() => {
	const filtered = grades.value.filter(grade =>
		!isFiltered.value || grade.course === courseFilter.value
	);
	return filtered;
});

// Get professor's courses
const professorCourses = computed(() => {
	if (!currentUser.value) return [];

	// Récupérer les classes où le professeur enseigne
	const professorClasses = classes.value.filter(cls =>
		cls.professors.includes(currentUser.value?.id || '')
	);

	// Extraire les noms des cours uniques
	const courseNames = professorClasses.map(cls => cls.name);
	return [...new Set(courseNames)];
});

// Get students filtered by selected course
const filteredStudents = computed(() => {
	if (!gradeForm.course) return [];

	// Trouver la classe correspondant au cours sélectionné
	const classInfo = classes.value.find(cls => cls.name === gradeForm.course);
	if (!classInfo) return [];

	// Récupérer les informations des étudiants de cette classe
	return users.value.filter(user =>
		user.role === 'student' && classInfo.students.includes(user.id)
	);
});

// Methods
const getStudentName = (userId: string) => {
	const student = users.value.find(user => user.id === userId);
	return student ? student.pseudo : userId;
};

// Get professor's grades
const loadProfessorGrades = () => {
	if (!currentUser.value) return;

	// Filtrer les notes par professeur
	grades.value = mockGrades.value.filter(grade =>
		grade.professor_id === currentUser.value?.id
	);
};

// Filtres
const clearFilter = () => {
	courseFilter.value = '';
	isFiltered.value = false;
};

// Gestion des notes
const openAddNoteModal = () => {
	resetForm();
	showCreateModal.value = true;
};

const editGrade = (grade: Grade) => {
	selectedGrade.value = grade;
	Object.assign(gradeForm, grade);
	showEditModal.value = true;
};

const confirmDeleteGrade = (grade: Grade) => {
	selectedGrade.value = grade;
	showDeleteModal.value = true;
};

const cancelGradeForm = () => {
	showCreateModal.value = false;
	showEditModal.value = false;
	resetForm();
};

const resetForm = () => {
	gradeForm.id = '';
	gradeForm.user_id = '';
	gradeForm.course = '';
	gradeForm.value = 0;
	gradeForm.professor_id = '';
	selectedGrade.value = null;
};

// Submit form
const submitGradeForm = async () => {
	try {
		globalError.value = null;

		if (showEditModal.value && selectedGrade.value) {
			// Edit existing grade
			const index = grades.value.findIndex(g => g.id === selectedGrade.value?.id);
			if (index !== -1) {
				grades.value[index] = {
					...selectedGrade.value,
					course: gradeForm.course,
					value: gradeForm.value
				};
			}
		} else {
			// Add new grade
			const newGrade: Grade = {
				id: `grade-${Date.now()}`,
				user_id: gradeForm.user_id,
				course: gradeForm.course,
				value: gradeForm.value,
				professor_id: currentUser.value?.id || '',
			};
			grades.value.push(newGrade);
		}

		showCreateModal.value = false;
		showEditModal.value = false;
		resetForm();
	} catch (error) {
		console.error('Error saving grade:', error);
		globalError.value = 'Échec de l\'enregistrement de la note.';
	}
};

const deleteGrade = async () => {
	if (!selectedGrade.value) return;

	try {
		globalError.value = null;

		// Remove grade from list
		grades.value = grades.value.filter(g => g.id !== selectedGrade.value?.id);

		showDeleteModal.value = false;
		selectedGrade.value = null;
	} catch (error) {
		console.error('Error deleting grade:', error);
		globalError.value = 'Échec de la suppression de la note.';
	}
};

// Lifecycle
onMounted(() => {
	// Load data
	loading.value = true;

	try {
		loadProfessorGrades();
	} catch (error) {
		console.error('Error loading data:', error);
		globalError.value = 'Une erreur est survenue lors du chargement des données.';
	} finally {
		loading.value = false;
	}
});
</script>

<style scoped>
.page-title {
	@apply text-4xl font-bold text-white mb-8 text-center;
}

.filter-container {
	@apply p-6 bg-violet-950 rounded-2xl shadow-md mb-8;
}

.filter-label {
	@apply block mb-2 text-violet-200;
}

.filter-input {
	@apply flex-1 px-4 py-2 rounded-full bg-violet-900 text-white placeholder-violet-300 outline-none;
}

.filter-button {
	@apply bg-violet-600 hover:bg-violet-700 text-white font-semibold px-6 py-2 rounded-full transition;
}

.clear-button {
	@apply bg-violet-800 hover:bg-violet-700 text-violet-200 font-semibold px-6 py-2 rounded-full transition;
}

.error-alert-enter-active,
.error-alert-leave-active {
	transition: all 0.3s ease;
}

.error-alert-enter-from,
.error-alert-leave-to {
	opacity: 0;
	transform: translateY(-20px);
}
</style>