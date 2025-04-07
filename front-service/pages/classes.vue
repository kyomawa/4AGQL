<template>
	<div class="min-h-screen py-12">
		<h1 class="page-title">Classes</h1>

		<div v-if="loading" class="py-8 text-center">
			<div class="w-12 h-12 mx-auto border-b-2 rounded-full border-violet-500 animate-spin"></div>
			<p class="mt-4 text-violet-300">Chargement des classes...</p>
		</div>

		<div v-else class="max-w-5xl mx-auto">
			<!-- Search/Filter -->
			<div class="filter-container">
				<label class="filter-label">Rechercher des classes</label>
				<div class="flex items-center space-x-3">
					<input type="text" v-model="nameFilter" placeholder="Entrez le nom de la classe" class="filter-input" />
					<button @click="applyFilter" class="filter-button">
						Rechercher
					</button>
					<button v-if="isFiltered" @click="clearFilter" class="clear-button">
						Effacer
					</button>
				</div>
			</div>

			<!-- Classes Grid -->
			<div v-if="classes.length === 0" class="py-8 text-center shadow-md bg-violet-950 rounded-2xl">
				<svg xmlns="http://www.w3.org/2000/svg" class="w-16 h-16 mx-auto text-violet-500" fill="none"
					viewBox="0 0 24 24" stroke="currentColor">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
						d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
				</svg>
				<h3 class="mt-4 text-lg font-medium text-violet-200">Aucune classe trouvée</h3>
				<p class="mt-1 text-violet-300">
					{{
						isFiltered
							? 'Essayez de modifier vos critères de recherche.'
							: 'Aucune classe n\'est disponible pour le moment.'
					}}
				</p>
			</div>

			<div v-else class="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
				<div v-for="classItem in classes" :key="classItem.id"
					class="overflow-hidden transition shadow-md bg-violet-950 rounded-2xl hover:shadow-lg">
					<div class="p-5 border-b border-violet-800">
						<h3 class="text-xl font-semibold text-violet-200">{{ classItem.name }}</h3>
					</div>
					<div class="p-5">
						<div class="flex items-center mb-3">
							<svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5 mr-2 text-violet-500" fill="none"
								viewBox="0 0 24 24" stroke="currentColor">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
									d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z" />
							</svg>
							<span class="text-violet-300">
								{{ classItem.students ? classItem.students.length : 0 }} Étudiants
							</span>
						</div>
						<div class="flex items-center mb-4">
							<svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5 mr-2 text-violet-500" fill="none"
								viewBox="0 0 24 24" stroke="currentColor">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
									d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
							</svg>
							<span class="text-violet-300">
								{{ classItem.professors ? classItem.professors.length : 0 }} Professeurs
							</span>
						</div>
						<NuxtLink :to="`/classes/${classItem.id}`"
							class="block py-2 text-center text-white transition rounded-full bg-violet-600 hover:bg-violet-700">
							Voir les détails
						</NuxtLink>
					</div>
				</div>
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue';
import { useQuery } from '@vue/apollo-composable';
import { useRoute } from 'vue-router';
import gql from 'graphql-tag';
import { useAuth } from '~/composables/useAuth';

// Cette page est publique, pas besoin de middleware role spécifique
definePageMeta({
	middleware: ['auth'], // auth middleware ne bloquera pas pour les routes publiques
});

const route = useRoute();
const { isAuthenticated, fetchCurrentUser } = useAuth();
const loading = ref(true);
const classes = ref<any[]>([]);
const nameFilter = ref('');
const isFiltered = ref(false);

const GET_CLASSES_QUERY = gql`
	query GetClasses($name: String) {
		get_classes(name: $name) {
			id
			name
			students
			professors
		}
	}
`;

const {
	result,
	loading: queryLoading,
	refetch,
	onError
} = useQuery(GET_CLASSES_QUERY, () => ({ name: isFiltered.value ? nameFilter.value : null }), {
	// Toujours recharger depuis le réseau et non le cache
	fetchPolicy: 'network-only',
});

// Gérer les erreurs
onError((error) => {
	console.error('Error fetching classes:', error);
});

// Watch for query result and update classes
watch(result, (newResult) => {
	if (newResult && newResult.get_classes) {
		classes.value = newResult.get_classes;
	}
});

// Watch for loading state and update local loading state
watch(queryLoading, (isLoading) => {
	loading.value = isLoading;
});

// S'assurer que l'utilisateur est chargé et recharger les données à chaque montage
onMounted(async () => {
	// Vérifier l'authentification sans bloquer l'affichage
	await fetchCurrentUser();

	// Recharger les données
	await refetch();
});

// Surveiller les changements de route pour recharger les données quand on revient sur cette page
watch(() => route.path, async (newPath) => {
	if (newPath === '/classes') {
		await refetch();
	}
});

const applyFilter = async () => {
	if (nameFilter.value.trim()) {
		isFiltered.value = true;
		await refetch();
	}
};

const clearFilter = async () => {
	nameFilter.value = '';
	isFiltered.value = false;
	await refetch();
};
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
</style>