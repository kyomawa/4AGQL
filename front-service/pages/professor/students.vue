<template>
	<div class="min-h-screen py-12">
		<div class="max-w-3xl mx-auto">
			<h1 class="page-title">Gestion des Étudiants</h1>

			<div v-if="loading" class="py-8 text-center">
				<div class="w-12 h-12 mx-auto border-b-2 rounded-full border-violet-500 animate-spin"></div>
				<p class="mt-4 text-violet-300">Chargement des données...</p>
			</div>

			<div v-else-if="!selectedClass" class="mb-8">
				<div class="p-6 bg-violet-950 rounded-2xl shadow-md">
					<h2 class="mb-4 text-xl font-semibold text-violet-200 text-center">Sélectionner une Classe</h2>

					<div v-if="professorClasses.length === 0" class="py-8 text-center rounded-2xl bg-violet-900">
						<p class="text-violet-300">Vous n'avez pas encore de classes.</p>
						<NuxtLink to="/professor/classes" class="inline-block mt-2 text-violet-300 hover:text-violet-200">
							Créer une classe
						</NuxtLink>
					</div>

					<div v-else class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
						<button v-for="classItem in professorClasses" :key="classItem.id" @click="selectClass(classItem)"
							class="p-4 text-left transition border rounded-lg border-violet-800 bg-violet-900 hover:bg-violet-800">
							<h3 class="font-medium text-violet-200">{{ classItem.name }}</h3>
							<p class="mt-1 text-sm text-violet-300">
								{{ classItem.students.length }} étudiants
							</p>
						</button>
					</div>
				</div>
			</div>

			<div v-else>
				<div
					class="flex flex-col sm:flex-row flex-wrap items-center justify-between p-6 mb-6 bg-violet-950 rounded-2xl shadow-md">
					<div>
						<h2 class="text-xl font-semibold text-violet-200">{{ selectedClass.name }}</h2>
						<p class="text-sm text-violet-300">
							{{ selectedClass.students.length }} étudiants inscrits
						</p>
					</div>
					<div class="flex flex-col sm:flex-row mt-4 sm:mt-0 space-y-3 sm:space-y-0 sm:space-x-3 w-full sm:w-auto">
						<button @click="selectedClass = null"
							class="flex items-center justify-center px-4 py-2 transition bg-violet-800 hover:bg-violet-700 text-violet-200 rounded-full">
							<svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5 mr-1" fill="none" viewBox="0 0 24 24"
								stroke="currentColor">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
							</svg>
							Retour aux Classes
						</button>
						<button @click="showAddModal = true"
							class="flex items-center justify-center px-4 py-2 text-white transition bg-violet-600 rounded-full hover:bg-violet-700">
							<svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5 mr-1" fill="none" viewBox="0 0 24 24"
								stroke="currentColor">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
							</svg>
							Ajouter un Étudiant
						</button>
					</div>
				</div>

				<div v-if="selectedClass.students.length === 0" class="py-8 text-center bg-violet-950 rounded-2xl shadow-md">
					<svg xmlns="http://www.w3.org/2000/svg" class="w-16 h-16 mx-auto text-violet-500" fill="none"
						viewBox="0 0 24 24" stroke="currentColor">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
							d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z" />
					</svg>
					<h3 class="mt-4 text-lg font-medium text-violet-200">Aucun étudiant inscrit</h3>
					<p class="mt-1 text-violet-300">Cette classe n'a pas encore d'étudiants.</p>
				</div>

				<div v-else class="overflow-hidden bg-violet-950 rounded-2xl shadow-md">
					<div class="p-4 border-b border-violet-800 bg-violet-900">
						<input type="text" v-model="searchQuery" placeholder="Rechercher des étudiants..."
							class="w-full px-4 py-2 rounded-full bg-violet-800 text-white placeholder-violet-300 outline-none" />
					</div>
					<div class="overflow-x-auto">
						<table class="min-w-full divide-y divide-violet-800">
							<thead class="bg-violet-900">
								<tr>
									<th class="px-6 py-3 text-xs font-medium tracking-wider text-left text-violet-300 uppercase">ID
										Étudiant
									</th>
									<th class="px-6 py-3 text-xs font-medium tracking-wider text-left text-violet-300 uppercase">Email
									</th>
									<th class="px-6 py-3 text-xs font-medium tracking-wider text-left text-violet-300 uppercase">Pseudo
									</th>
									<th class="px-6 py-3 text-xs font-medium tracking-wider text-right text-violet-300 uppercase">Actions
									</th>
								</tr>
							</thead>
							<tbody class="bg-violet-950 divide-y divide-violet-800">
								<tr v-for="student in getFilteredStudents()" :key="student.id" class="hover:bg-violet-900">
									<td class="px-6 py-4 text-sm font-medium text-violet-200 whitespace-nowrap">{{ student.id }}</td>
									<td class="px-6 py-4 text-sm text-violet-200 whitespace-nowrap">{{ student.email }}</td>
									<td class="px-6 py-4 text-sm text-violet-200 whitespace-nowrap">{{ student.pseudo }}</td>
									<td class="px-6 py-4 text-sm font-medium text-right whitespace-nowrap">
										<NuxtLink :to="`/professor/grades?student=${student.id}`"
											class="mr-3 text-violet-300 hover:text-violet-200">Voir les Notes</NuxtLink>
										<button @click="confirmRemoveStudent(student)"
											class="text-red-400 hover:text-red-300">Retirer</button>
									</td>
								</tr>
							</tbody>
						</table>
					</div>
				</div>
			</div>

			<!-- Modal Ajout -->
			<div v-if="showAddModal" class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50">
				<div class="w-full max-w-md p-6 bg-violet-950 rounded-2xl shadow-md">
					<h2 class="mb-4 text-xl font-bold text-violet-200 text-center">Ajouter un Étudiant à la Classe</h2>
					<form @submit.prevent="addStudent" class="space-y-4">
						<div>
							<label class="block mb-2 text-sm font-medium text-violet-300">Sélectionner un étudiant</label>
							<div class="relative">
								<input v-model="searchStudentQuery" type="text"
									class="w-full px-4 py-2 rounded-full bg-violet-900 text-white placeholder-violet-300 outline-none"
									placeholder="Rechercher un étudiant par nom ou email..." />
								<div v-if="filteredAvailableStudents.length > 0"
									class="absolute z-10 w-full mt-1 bg-violet-900 rounded-lg shadow-lg max-h-60 overflow-auto">
									<div v-for="student in filteredAvailableStudents" :key="student.id"
										@click="selectStudentToAdd(student)"
										class="px-4 py-2 cursor-pointer hover:bg-violet-800 border-b border-violet-800 last:border-b-0">
										<div class="font-medium text-violet-200">{{ student.pseudo }}</div>
										<div class="text-sm text-violet-300">{{ student.email }}</div>
									</div>
								</div>
							</div>
							<div v-if="selectedStudentToAdd" class="mt-3 p-3 bg-violet-900 rounded-lg">
								<div class="flex justify-between items-center">
									<div>
										<div class="font-medium text-violet-200">{{ selectedStudentToAdd.pseudo }}</div>
										<div class="text-sm text-violet-300">{{ selectedStudentToAdd.email }}</div>
									</div>
									<button type="button" @click="selectedStudentToAdd = null"
										class="text-violet-300 hover:text-violet-200">
										<svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
											<path fill-rule="evenodd"
												d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
												clip-rule="evenodd" />
										</svg>
									</button>
								</div>
							</div>
						</div>
						<div class="flex justify-end pt-4 space-x-3">
							<button type="button" @click="showAddModal = false"
								class="px-4 py-2 transition rounded-full bg-violet-800 hover:bg-violet-700 text-violet-200">Annuler</button>
							<button type="submit" :disabled="!selectedStudentToAdd"
								class="px-4 py-2 text-white transition rounded-full disabled:opacity-50 disabled:cursor-not-allowed"
								:class="selectedStudentToAdd ? 'bg-violet-600 hover:bg-violet-700' : 'bg-violet-800'">
								Ajouter
							</button>
						</div>
					</form>
				</div>
			</div>

			<!-- Modal Retrait -->
			<div v-if="showRemoveModal" class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50">
				<div class="w-full max-w-md p-6 bg-violet-950 rounded-2xl shadow-md">
					<h2 class="mb-4 text-xl font-bold text-violet-200">Retirer l'Étudiant</h2>
					<p class="mb-6 text-violet-300">
						Êtes-vous sûr de vouloir retirer cet étudiant de la classe ? Cela ne supprimera pas son compte.
					</p>
					<div class="flex justify-end space-x-3">
						<button @click="showRemoveModal = false"
							class="px-4 py-2 transition rounded-full bg-violet-800 hover:bg-violet-700 text-violet-200">Annuler</button>
						<button @click="removeStudent"
							class="px-4 py-2 text-white transition bg-red-600 rounded-full hover:bg-red-700">Retirer</button>
					</div>
				</div>
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import { useRoute } from 'vue-router';
import { useQuery, useMutation } from '@vue/apollo-composable';
import gql from 'graphql-tag';
import { useCurrentUser } from '~/composables/useCurrentUser';
import type { User } from '~/types/user';

definePageMeta({
	middleware: ['auth', 'role'],
	role: 'professor'
});

const route = useRoute();
const { currentUser } = useCurrentUser();

const loading = ref(true);
const classes = ref<any[]>([]);
const allUsers = ref<User[]>([]);
const selectedClass = ref<any>(null);
const searchQuery = ref('');
const newStudentId = ref('');
const showAddModal = ref(false);
const showRemoveModal = ref(false);
const studentToRemove = ref<any>(null);
const searchStudentQuery = ref('');
const selectedStudentToAdd = ref<User | null>(null);

const GET_CLASSES_QUERY = gql`
	query {
		get_classes {
			id
			name
			students
			professors
		}
	}
`;

const GET_USERS_QUERY = gql`
	query {
		get_users {
			id
			email
			pseudo
			role
		}
	}
`;

const ENROLL_STUDENT_MUTATION = gql`
	mutation($classId: String!, $studentId: String!) {
		enroll_student(classId: $classId, studentId: $studentId) {
			id
			students
		}
	}
`;

const REMOVE_STUDENT_MUTATION = gql`
	mutation($classId: String!, $studentId: String!) {
		remove_student(classId: $classId, studentId: $studentId) {
			id
			students
		}
	}
`;

const { result: classesResult, refetch: refetchClasses } = useQuery(GET_CLASSES_QUERY);
const { result: usersResult, refetch: refetchUsers } = useQuery(GET_USERS_QUERY);
const { mutate: enrollStudent } = useMutation(ENROLL_STUDENT_MUTATION);
const { mutate: removeStudentMutation } = useMutation(REMOVE_STUDENT_MUTATION);

watch([classesResult, usersResult], ([newClasses, newUsers]) => {
	if (newClasses?.get_classes) {
		classes.value = newClasses.get_classes;
		if (route.query.class) {
			const found = classes.value.find((c) => c.id === route.query.class);
			if (found) selectedClass.value = found;
		}
	}
	if (newUsers?.get_users) {
		allUsers.value = newUsers.get_users.filter((u: User) => u.role === 'student');
	}
	loading.value = false;
});

const professorClasses = computed(() => {
	const user = currentUser.value;
	if (!user) return [];
	return classes.value.filter((cls) => cls.professors.includes(user.id));
});
const getFilteredStudents = computed(() => {
	if (!selectedClass.value) return [];

	const students = selectedClass.value.students.map((studentId: string) =>
		allUsers.value.find((u) => u.id === studentId) || { id: studentId, email: 'Inconnu', pseudo: 'Inconnu' }
	);

	if (!searchQuery.value) return students;

	const query = searchQuery.value.toLowerCase();
	return students.filter(
		(s: any) =>
			s.id.toLowerCase().includes(query) ||
			s.email.toLowerCase().includes(query) ||
			s.pseudo.toLowerCase().includes(query)
	);
});


const filteredAvailableStudents = computed(() => {
	if (!selectedClass.value) return [];

	// Filtrer les étudiants qui ne sont pas déjà dans la classe
	const availableStudents = allUsers.value
		.filter((user) =>
			user.role === 'student' &&
			!selectedClass.value.students.includes(user.id)
		);

	if (!searchStudentQuery.value.trim()) return availableStudents;

	const query = searchStudentQuery.value.toLowerCase();
	return availableStudents.filter(
		(student) =>
			student.pseudo.toLowerCase().includes(query) ||
			student.email.toLowerCase().includes(query) ||
			student.id.toLowerCase().includes(query)
	);
});

const selectStudentToAdd = (student: User) => {
	selectedStudentToAdd.value = student;
	searchStudentQuery.value = '';
};

const selectClass = (cls: any) => {
	selectedClass.value = cls;
	searchQuery.value = '';
};

const addStudent = async () => {
	if (!selectedClass.value || !selectedStudentToAdd.value) return;

	await enrollStudent({
		classId: selectedClass.value.id,
		studentId: selectedStudentToAdd.value.id
	});

	selectedClass.value.students.push(selectedStudentToAdd.value.id);
	selectedStudentToAdd.value = null;
	showAddModal.value = false;
};

const confirmRemoveStudent = (student: any) => {
	studentToRemove.value = student;
	showRemoveModal.value = true;
};

const removeStudent = async () => {
	if (!selectedClass.value || !studentToRemove.value) return;
	await removeStudentMutation({ classId: selectedClass.value.id, studentId: studentToRemove.value.id });
	selectedClass.value.students = selectedClass.value.students.filter((id: string) => id !== studentToRemove.value.id);
	showRemoveModal.value = false;
	studentToRemove.value = null;
};

onMounted(async () => {
	await Promise.all([refetchClasses(), refetchUsers()]);
});
</script>

<style scoped>
.page-title {
	@apply text-4xl font-bold text-white mb-8 text-center;
}
</style>