// pages/classes/[id].vue
<template>
	<div>
		<div v-if="loading" class="py-8 text-center">
			<div class="w-12 h-12 mx-auto border-b-2 border-blue-500 rounded-full animate-spin"></div>
			<p class="mt-4 text-gray-600">Loading class details...</p>
		</div>

		<div v-else-if="error" class="py-8 text-center rounded-lg shadow bg-red-50">
			<svg xmlns="http://www.w3.org/2000/svg" class="w-16 h-16 mx-auto text-red-500" fill="none" viewBox="0 0 24 24"
				stroke="currentColor">
				<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
					d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
			</svg>
			<h3 class="mt-4 text-lg font-medium text-red-800">{{ error }}</h3>
			<div class="mt-4">
				<NuxtLink to="/classes" class="text-blue-600 underline hover:text-blue-800">
					Back to Classes
				</NuxtLink>
			</div>
		</div>

		<div v-else-if="classData">
			<!-- Back Link -->
			<div class="mb-6">
				<NuxtLink to="/classes" class="inline-flex items-center text-blue-600 hover:text-blue-800">
					<svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5 mr-1" fill="none" viewBox="0 0 24 24"
						stroke="currentColor">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
					</svg>
					Back to Classes
				</NuxtLink>
			</div>

			<!-- Class Header -->
			<div class="p-6 mb-6 bg-white rounded-lg shadow">
				<h1 class="text-3xl font-bold text-blue-700">{{ classData.name }}</h1>
				<div class="flex flex-wrap gap-4 mt-4">
					<div class="inline-flex items-center px-4 py-2 rounded-md bg-blue-50">
						<svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5 mr-2 text-blue-600" fill="none" viewBox="0 0 24 24"
							stroke="currentColor">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
								d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z" />
						</svg>
						<span class="font-medium">
							{{ classData.students ? classData.students.length : 0 }} Students
						</span>
					</div>
					<div class="inline-flex items-center px-4 py-2 rounded-md bg-green-50">
						<svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5 mr-2 text-green-600" fill="none" viewBox="0 0 24 24"
							stroke="currentColor">
							<path d="M12 14l9-5-9-5-9 5 9 5z" />
							<path
								d="M12 14l6.16-3.422a12.083 12.083 0 01.665 6.479A11.952 11.952 0 0012 20.055a11.952 11.952 0 00-6.824-2.998 12.078 12.078 0 01.665-6.479L12 14z" />
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
								d="M12 14l9-5-9-5-9 5 9 5zm0 0l6.16-3.422a12.083 12.083 0 01.665 6.479A11.952 11.952 0 0012 20.055a11.952 11.952 0 00-6.824-2.998 12.078 12.078 0 01.665-6.479L12 14zm-4 6v-7.5l4-2.222" />
						</svg>
						<span class="font-medium">
							{{ classData.professors ? classData.professors.length : 0 }} Professors
						</span>
					</div>
				</div>
			</div>

			<!-- Students Section -->
			<div class="p-6 mb-6 bg-white rounded-lg shadow">
				<h2 class="mb-4 text-xl font-semibold">Students</h2>

				<div v-if="!classData.students || classData.students.length === 0"
					class="py-6 text-center rounded-lg bg-gray-50">
					<p class="text-gray-500">No students enrolled in this class yet.</p>
				</div>

				<div v-else class="overflow-hidden border rounded-lg">
					<table class="min-w-full divide-y divide-gray-200">
						<thead class="bg-gray-50">
							<tr>
								<th class="px-6 py-3 text-xs font-medium tracking-wider text-left text-gray-500 uppercase">
									Student ID
								</th>
								<th class="px-6 py-3 text-xs font-medium tracking-wider text-left text-gray-500 uppercase">
									Actions
								</th>
							</tr>
						</thead>
						<tbody class="bg-white divide-y divide-gray-200">
							<tr v-for="studentId in classData.students" :key="studentId">
								<td class="px-6 py-4 whitespace-nowrap">
									<div class="text-sm font-medium text-gray-900">
										{{ studentId }}
									</div>
								</td>
							</tr>
						</tbody>
					</table>
				</div>
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRoute } from 'vue-router';
import { useNuxtApp } from '#app';
import type { Class } from '~/types/user';
import gql from 'graphql-tag';

// Définir les refs avec des types explicites
const loading = ref<boolean>(true);
const error = ref<string | null>(null);
const classData = ref<Class | null>(null);

// Récupérer l'ID de la classe depuis la route
const route = useRoute();
const classId = route.params.id as string;

// Mutation GraphQL pour récupérer les détails de la classe
const GET_CLASS_DETAILS = gql`
  query GetClassById($id: ID!) {
    get_class_by_id(id: $id) {
      id
      name
      students
      professors
    }
  }
`;

onMounted(async () => {
	try {
		const { $apollo } = useNuxtApp();

		const result = await $apollo.query({
			query: GET_CLASS_DETAILS,
			variables: { id: classId }
		});

		if (result.data?.get_class_by_id) {
			classData.value = result.data.get_class_by_id;
		} else {
			error.value = 'Class not found';
		}
	} catch (err) {
		console.error('Error fetching class details:', err);
		error.value = 'Failed to load class details';
	} finally {
		loading.value = false;
	}
});
</script>