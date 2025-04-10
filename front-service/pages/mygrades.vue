<template>
	<div class="min-h-screen py-12 px-4 sm:px-6 lg:px-8 dark:to-violet-900">
		<div class="w-[830px] mx-auto">
			<Head>
				<title>Mes notes - SchoolInc</title>
			</Head>
			<h2 class="text-center title-primary">Mes notes</h2>

			<!-- Barre de recherche stylisée pour filtrer par nom de classe -->
			<div class="mb-6 mx-auto w-full">
				<label class="label-primary dark:text-violet-300">Rechercher par classe</label>
				<div class="flex w-full items-center">
					<input
						type="text"
						v-model="classFilter"
						@keyup.enter="applyFilter"
						placeholder="Nom de la classe"
						class="input-primary dark:bg-violet-900 dark:text-white dark:border-violet-800 flex-1"
					/>
					<button
						@click="applyFilter"
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
						isFiltered
							? 'Essayez de modifier vos critères de recherche.'
							: "Aucune note n'est disponible pour le moment."
					}}
				</p>
			</div>

			<!-- Liste des notes (une note par ligne) -->
			<div
				v-else
				class="space-y-4"
			>
				<GradeCard
					v-for="grade in filteredGrades"
					:key="grade.id"
					:grade="grade"
					class="w-full"
				/>
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import { useNuxtApp } from '#app';
import { provideApolloClient, useQuery } from '@vue/apollo-composable';
import gql from 'graphql-tag';
import GradeCard from '~/components/GradeCard.vue';
import { useAuth } from '~/composables/useAuth';

// Fournir le client Apollo destiné au service Grades
const nuxtApp = useNuxtApp();
provideApolloClient(nuxtApp.$apolloGrades as any);

// Récupérer le user connecté via useAuth et définir currentUserId
const { user } = useAuth();
const currentUserId = computed(() => user.value?.id || '');

// Barre de recherche pour filtrer par nom de classe
const classFilter = ref('');
const isFiltered = ref(false);
const loading = ref(true);
const grades = ref<any[]>([]);

// Définition de la query avec variable userId
const GET_GRADES_QUERY = gql`
	query GetGradesByUserId($userId: String!) {
		getGradesByUserId(userId: $userId) {
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

// Utilisation de useQuery en passant currentUserId
const {
	result,
	loading: queryLoading,
	refetch,
	onError,
} = useQuery(GET_GRADES_QUERY, () => ({ userId: currentUserId.value }), {
	fetchPolicy: 'network-only',
});

onError((error) => {
	console.error('Error fetching grades:', error);
});

watch(result, (newResult) => {
	if (newResult && newResult.getGradesByUserId) {
		grades.value = newResult.getGradesByUserId;
	}
});

watch(queryLoading, (isLoading) => {
	loading.value = isLoading;
});

onMounted(async () => {
	if (currentUserId.value) {
		await refetch();
	}
});

// Filtrer les notes par le nom de la classe (en minuscule)
const filteredGrades = computed(() => {
	if (!classFilter.value.trim()) {
		return grades.value;
	}
	const filter = classFilter.value.toLowerCase();
	return grades.value.filter(
		(g: any) => g.className && g.className.toLowerCase().includes(filter),
	);
});

const applyFilter = async () => {
	if (classFilter.value.trim()) {
		isFiltered.value = true;
		await refetch();
	}
};

const clearFilter = async () => {
	classFilter.value = '';
	isFiltered.value = false;
	await refetch();
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
</style>
