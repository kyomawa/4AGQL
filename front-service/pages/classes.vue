<template>
	<div class="min-h-screen py-12 px-4 sm:px-6 lg:px-8 dark:to-violet-900">
		<div class="w-[830px] mx-auto">
			<Head>
				<title>Classes - SchoolInc</title>
			</Head>
			<h2 class="text-center title-primary">Classes</h2>

			<!-- Barre de recherche stylisée -->
			<div class="mb-6 mx-auto w-full">
				<label class="label-primary dark:text-violet-300">Rechercher des classes</label>
				<div class="flex w-full items-center">
					<input
						type="text"
						v-model="nameFilter"
						@keyup.enter="applyFilter"
						placeholder="Entrez le nom de la classe"
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
				<p class="mt-4 text-violet-300">Chargement des classes...</p>
			</div>

			<!-- Aucun résultat -->
			<div
				v-else-if="classes.length === 0"
				class="p-6 card text-center"
			>
				<h3 class="mt-4 text-lg font-medium text-violet-300">Aucune classe trouvée</h3>
				<p class="mt-1 text-violet-300">
					{{
						isFiltered
							? 'Essayez de modifier vos critères de recherche.'
							: "Aucune classe n'est disponible pour le moment."
					}}
				</p>
			</div>

			<!-- Grille des classes -->
			<div
				v-else
				class="grid grid-cols-3 gap-6"
			>
				<ClassCard
					v-for="classItem in classes"
					:key="classItem.id"
					:classItem="classItem"
				/>
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue';
import { useRoute } from 'vue-router';
import { provideApolloClient, useQuery } from '@vue/apollo-composable';
import gql from 'graphql-tag';
import { useNuxtApp } from '#app';
import ClassCard from '~/components/ClassCard.vue';

// Fournir le client Apollo destiné au service Classes
const nuxtApp = useNuxtApp();
provideApolloClient(nuxtApp.$apolloClasses as any);

const route = useRoute();
const nameFilter = ref('');
const isFiltered = ref(false);
const loading = ref(true);
const classes = ref<any[]>([]);

// Correction de la requête pour qu'elle corresponde à votre backend
const GET_CLASSES_QUERY = gql`
	query GetClasses($name: String) {
		getClasses(name: $name) {
			id
			name
			creatorId
			creatorNames
			userIds
		}
	}
`;

const {
	result,
	loading: queryLoading,
	refetch,
	onError,
} = useQuery(GET_CLASSES_QUERY, () => ({ name: isFiltered.value ? nameFilter.value : null }), {
	fetchPolicy: 'network-only',
});

onError((error) => {
	console.error('Error fetching classes:', error);
});

watch(result, (newResult) => {
	if (newResult && newResult.getClasses) {
		classes.value = newResult.getClasses;
	}
});

watch(queryLoading, (isLoading) => {
	loading.value = isLoading;
});

// Fonction pour déboguer le résultat
const debugResult = () => {
	console.log('Résultat de la requête:', result.value);
};

onMounted(async () => {
	await refetch();
	// Débogage après le chargement
	setTimeout(debugResult, 1000);
});

watch(
	() => route.path,
	async (newPath) => {
		if (newPath === '/classes') {
			await refetch();
		}
	},
);

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
/* Styles repris du style de register.vue */
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
