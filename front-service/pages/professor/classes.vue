<template>
	<div class="min-h-screen py-12">
		<h1 class="page-title">Gestion des Classes</h1>

		<div v-if="loading" class="py-8 text-center">
			<div class="w-12 h-12 mx-auto border-b-2 rounded-full border-violet-500 animate-spin"></div>
			<p class="mt-4 text-violet-300">Chargement des classes...</p>
		</div>

		<div v-else class="max-w-5xl mx-auto">
			<!-- Search/Filter et Ajouter -->
			<div class="flex flex-col gap-4 mb-8 sm:flex-row">
				<div class="flex-1 filter-container">
					<label class="filter-label">Rechercher des Classes</label>
					<div class="flex gap-3">
						<input type="text" v-model="nameFilter" placeholder="Nom de la classe" class="filter-input" />
						<button @click="applyFilter" class="filter-button">
							Rechercher
						</button>
						<button v-if="isFiltered" @click="clearFilter" class="clear-button">
							Effacer
						</button>
					</div>
				</div>

				<button @click="showCreateModal = true"
					class="p-6 bg-violet-700 hover:bg-violet-600 text-white font-semibold rounded-2xl transition flex items-center justify-center shadow-md">
					<div class="flex items-center">
						<svg xmlns="http://www.w3.org/2000/svg" class="w-6 h-6 mr-2" fill="none" viewBox="0 0 24 24"
							stroke="currentColor">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
						</svg>
						<span class="text-lg font-bold">Ajouter</span>
					</div>
				</button>
			</div>

			<div v-if="professorClasses.length === 0" class="py-8 text-center shadow-md bg-violet-950 rounded-2xl">
				<svg xmlns="http://www.w3.org/2000/svg" class="w-16 h-16 mx-auto text-violet-500" fill="none"
					viewBox="0 0 24 24" stroke="currentColor">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
						d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
				</svg>
				<h3 class="mt-4 text-lg font-medium text-violet-200">Aucune classe trouvée</h3>
				<p class="mt-1 text-violet-300">
					{{ isFiltered ? 'Essayez de modifier vos critères de recherche.' : 'Vous n\'avez pas encore créé de classes.'
					}}
				</p>
			</div>

			<div v-else class="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
				<div v-for="classe in professorClasses" :key="classe.id"
					class="overflow-hidden transition shadow-md bg-violet-950 rounded-2xl hover:shadow-lg">
					<div class="p-5 border-b border-violet-800">
						<h3 class="text-xl font-semibold text-violet-200">{{ classe.name }}</h3>
					</div>
					<div class="p-5">
						<p class="text-violet-300 mb-4">Étudiants inscrits : {{ classe.students.length }}</p>
						<div class="flex justify-end gap-2">
							<button @click="editClass(classe)"
								class="px-3 py-1 text-sm bg-blue-600 hover:bg-blue-700 text-white rounded-full transition">Modifier</button>
							<button @click="confirmDeleteClass(classe)"
								class="px-3 py-1 text-sm bg-red-600 hover:bg-red-700 text-white rounded-full transition">Supprimer</button>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, watch } from 'vue';
import { useQuery, useMutation } from '@vue/apollo-composable';
import gql from 'graphql-tag';
import { useCurrentUser } from '~/composables/useCurrentUser';
import type { User } from '~/types/user';

definePageMeta({
	middleware: ['auth', 'role'],
	role: 'professor',
});

const { currentUser } = useCurrentUser();
const loading = ref(true);
const classes = ref<any[]>([]);
const nameFilter = ref('');
const isFiltered = ref(false);

const showCreateModal = ref(false);
const showEditModal = ref(false);
const showDeleteModal = ref(false);

const classForm = reactive({
	id: '',
	name: '',
	students: [] as string[],
	professors: [] as string[],
});

const selectedClass = ref<any>(null);

const professorClasses = computed(() => {
	const user = currentUser.value as User | null;
	if (!user) return [];
	return classes.value.filter((cls) => cls.professors.includes(user.id || ''));
});

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

const CREATE_CLASS_MUTATION = gql`
  mutation CreateClass($class: CreateClassRequest!) {
    create_class(class: $class) {
      id
      name
    }
  }
`;

const UPDATE_CLASS_MUTATION = gql`
  mutation UpdateClass($id: ID!, $class: UpdateClassRequest!) {
    update_class_by_id(id: $id, class: $class) {
      id
      name
    }
  }
`;

const DELETE_CLASS_MUTATION = gql`
  mutation DeleteClass($id: ID!) {
    delete_class_by_id(id: $id) {
      id
    }
  }
`;

const { result, loading: queryLoading, refetch } = useQuery(GET_CLASSES_QUERY, () => ({
	name: isFiltered.value ? nameFilter.value : null,
}));

const { mutate: createClass } = useMutation(CREATE_CLASS_MUTATION);
const { mutate: updateClass } = useMutation(UPDATE_CLASS_MUTATION);
const { mutate: deleteClassById } = useMutation(DELETE_CLASS_MUTATION);

watch(result, (newResult) => {
	if (newResult && newResult.get_classes) {
		classes.value = newResult.get_classes;
	}
});

watch(queryLoading, (isLoading) => {
	loading.value = isLoading;
});

onMounted(async () => {
	await refetch();
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

const editClass = (classItem: any) => {
	selectedClass.value = classItem;
	classForm.id = classItem.id;
	classForm.name = classItem.name;
	classForm.students = [...classItem.students];
	classForm.professors = [...classItem.professors];
	showEditModal.value = true;
};

const confirmDeleteClass = (classItem: any) => {
	selectedClass.value = classItem;
	showDeleteModal.value = true;
};

const cancelClassForm = () => {
	showCreateModal.value = false;
	showEditModal.value = false;
	resetForm();
};

const resetForm = () => {
	classForm.id = '';
	classForm.name = '';
	classForm.students = [];
	classForm.professors = [];
	selectedClass.value = null;
};

const submitClassForm = async () => {
	const user = currentUser.value as User | null;
	if (!user) {
		console.error('Aucun utilisateur connecté');
		return;
	}

	if (showEditModal.value) {
		try {
			await updateClass({
				id: classForm.id,
				class: { name: classForm.name },
			});
			showEditModal.value = false;
			resetForm();
			await refetch();
		} catch (error) {
			console.error('Erreur lors de la mise à jour de la classe :', error);
			alert('Echec de la mise à jour de la classe.');
		}
	} else {
		try {
			await createClass({
				class: {
					name: classForm.name,
					professors: [user.id || ''],
				},
			});
			showCreateModal.value = false;
			resetForm();
			await refetch();
		} catch (error) {
			console.error('Erreur lors de la création de la classe :', error);
			alert('Echec de la création de la classe.');
		}
	}
};

const deleteClass = async () => {
	if (!selectedClass.value) return;
	if (selectedClass.value.students && selectedClass.value.students.length > 0) {
		alert('Impossible de supprimer une classe avec des étudiants inscrits.');
		showDeleteModal.value = false;
		return;
	}
	try {
		await deleteClassById({ id: selectedClass.value.id });
		showDeleteModal.value = false;
		selectedClass.value = null;
		await refetch();
	} catch (error) {
		console.error('Erreur lors de la suppression de la classe :', error);
		alert('Echec de la suppression de la classe.');
	}
};
</script>

<style scoped>
.page-title {
	@apply text-4xl font-bold text-white mb-8 text-center;
}

.filter-container {
	@apply p-6 bg-violet-950 rounded-2xl shadow-md;
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