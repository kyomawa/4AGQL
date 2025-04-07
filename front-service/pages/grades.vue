<template>
	<div class="min-h-screen py-12">

		<Head>
			<title>Mes Notes - SchoolInc</title>
		</Head>

		<div class="max-w-5xl mx-auto">
			<h1 class="page-title">Mes Notes</h1>

			<div class="filter-container">
				<label class="filter-label">Filtrer par matière</label>
				<div class="flex items-center space-x-3">
					<input type="text" v-model="courseFilter" placeholder="Entrez le nom de la matière" class="filter-input" />
					<button @click="applyFilter" class="filter-button">Filtrer</button>
					<button v-if="isFiltered" @click="clearFilter" class="clear-button">Effacer</button>
				</div>
			</div>

			<div class="table-container">
				<table class="grades-table">
					<thead>
						<tr>
							<th>Matière</th>
							<th>Note</th>
							<th>Professeur</th>
						</tr>
					</thead>
					<tbody>
						<tr v-for="grade in grades" :key="grade.id" class="border-b border-violet-800">
							<td class="bg-violet-900">{{ grade.course }}</td>
							<td :class="getGradeStyles(grade.value)">{{ grade.value }}/20</td>
							<td class="bg-violet-900">{{ getProfessorName(grade.professor_id) }}</td>
						</tr>
					</tbody>
				</table>
			</div>

			<div class="stats-container">
				<div class="stat-card">
					<h3>Moyenne Générale</h3>
					<p :class="getStatGradeStyle(averageGrade)">{{ averageGrade.toFixed(2) }}/20</p>
				</div>
				<div class="stat-card">
					<h3>Note la plus élevée</h3>
					<p :class="getStatGradeStyle(highestGrade)">{{ highestGrade.toFixed(2) }}/20</p>
				</div>
				<div class="stat-card">
					<h3>Note la plus basse</h3>
					<p :class="getStatGradeStyle(lowestGrade)">{{ lowestGrade.toFixed(2) }}/20</p>
				</div>
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import { useQuery } from '@vue/apollo-composable';
import { useRoute } from 'vue-router';
import gql from 'graphql-tag';
import { useAuth } from '~/composables/useAuth';

// Cette page nécessite le rôle 'student' minimum
definePageMeta({
	middleware: ['auth', 'role'],
	role: 'student'
});

const route = useRoute();
const { fetchCurrentUser } = useAuth();
const loading = ref(true);
const grades = ref<any[]>([]);
const professors = ref<any[]>([]);
const courseFilter = ref('');
const isFiltered = ref(false);

// Query pour récupérer les notes
const GET_GRADES_QUERY = gql`
	query GetGrades($course: String) {
		get_grades(course: $course) {
			id
			user_id
			course
			value
			professor_id
		}
	}
`;

// Query pour récupérer les professeurs
const GET_USERS_QUERY = gql`
	query {
		get_users {
			id
			pseudo
			role
		}
	}
`;

const {
	result: gradesResult,
	loading: gradesLoading,
	refetch: refetchGrades,
	onError: onGradesError
} = useQuery(GET_GRADES_QUERY, () => ({
	course: isFiltered.value ? courseFilter.value : null
}), {
	fetchPolicy: 'network-only',
});

const {
	result: usersResult,
	refetch: refetchUsers,
	onError: onUsersError
} = useQuery(GET_USERS_QUERY, null, {
	fetchPolicy: 'network-only',
});

// Gérer les erreurs
onGradesError((error) => {
	console.error('Error fetching grades:', error);
});

onUsersError((error) => {
	console.error('Error fetching users:', error);
});

// Observer les résultats des requêtes
watch(gradesResult, (newResult) => {
	if (newResult && newResult.get_grades) {
		grades.value = newResult.get_grades;
	}
});

watch(usersResult, (newResult) => {
	if (newResult && newResult.get_users) {
		professors.value = newResult.get_users.filter(user =>
			user.role === 'professor' || user.role === 'admin'
		);
	}
});

// Observer l'état de chargement
watch(gradesLoading, (isLoading) => {
	loading.value = isLoading;
});

// Fonction pour obtenir le nom du professeur
const getProfessorName = (professorId) => {
	const professor = professors.value.find(p => p.id === professorId);
	return professor ? professor.pseudo : `Professor ID: ${professorId}`;
};

// Initialiser les données
onMounted(async () => {
	// S'assurer que l'authentification est à jour
	await fetchCurrentUser();

	// Charger les données
	await Promise.all([
		refetchGrades(),
		refetchUsers()
	]);
});

// Surveiller les changements de route pour recharger les données
watch(() => route.path, async (newPath) => {
	if (newPath === '/grades') {
		await Promise.all([
			refetchGrades(),
			refetchUsers()
		]);
	}
});

const applyFilter = async () => {
	if (courseFilter.value.trim()) {
		isFiltered.value = true;
		await refetchGrades();
	}
};

const clearFilter = async () => {
	courseFilter.value = '';
	isFiltered.value = false;
	await refetchGrades();
};

// Computed statistics
const averageGrade = computed(() => {
	if (grades.value.length === 0) return 0;
	const sum = grades.value.reduce((acc, grade) => acc + grade.value, 0);
	return sum / grades.value.length;
});

const highestGrade = computed(() => {
	if (grades.value.length === 0) return 0;
	return Math.max(...grades.value.map((grade) => grade.value));
});

const lowestGrade = computed(() => {
	if (grades.value.length === 0) return 0;
	return Math.min(...grades.value.map((grade) => grade.value));
});

// Helper functions pour les styles des notes
const getGradeStyles = (grade: number) => {
	if (grade >= 16) return 'text-green-400 bg-violet-900';
	if (grade >= 12) return 'text-blue-400 bg-violet-900';
	if (grade >= 10) return 'text-yellow-400 bg-violet-900';
	return 'text-red-400 bg-violet-900';
};

// Helper function pour les styles des stats
const getStatGradeStyle = (grade: number) => {
	if (grade >= 16) return 'text-green-400 text-xl font-bold';
	if (grade >= 12) return 'text-blue-400 text-xl font-bold';
	if (grade >= 10) return 'text-yellow-400 text-xl font-bold';
	return 'text-red-400 text-xl font-bold';
};
</script>

<style scoped>
.page-title {
	@apply text-4xl font-bold text-white mb-8;
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

.table-container {
	@apply rounded-2xl shadow-md overflow-hidden;
}

.grades-table {
	@apply min-w-full text-white;
}

.grades-table th {
	@apply px-6 py-4 text-left bg-violet-900;
}

.grades-table td {
	@apply px-6 py-4;
}

.stats-container {
	@apply grid grid-cols-3 gap-4 mt-8;
}

.stat-card {
	@apply bg-violet-950 rounded-2xl shadow-md p-4;
}

.stat-card h3 {
	@apply text-violet-200;
}
</style>