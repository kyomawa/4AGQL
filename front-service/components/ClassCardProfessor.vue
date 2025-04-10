<template>
	<div
		class="card p-6 transition duration-300 hover:shadow-lg h-48 flex flex-col justify-between"
	>
		<h3 class="text-center text-xl font-semibold text-violet-700 dark:text-violet-300 mb-2">
			{{ classItem.name }}
		</h3>

		<div class="flex flex-col items-center text-center">
			<p class="text-gray-600 dark:text-gray-400 text-sm">
				{{ classItem.userIds?.length || 0 }} étudiant(s) inscrits
			</p>
		</div>

		<div class="mt-4 flex justify-center space-x-2">
			<NuxtLink
				:to="`/professor/grades?classId=${classItem.id}`"
				class="btn-secondary"
			>
				Voir les notes
			</NuxtLink>
			<NuxtLink
				:to="`/professor/students?classId=${classItem.id}`"
				class="link-primary"
			>
				Voir les étudiants
			</NuxtLink>
		</div>
	</div>
</template>

<script setup lang="ts">
defineProps({
	classItem: {
		type: Object,
		required: true,
	},
});
</script>

<style scoped>
.card {
	@apply bg-white shadow-lg rounded-2xl transition duration-300 dark:bg-violet-950;
}

.btn-secondary {
	@apply px-3 py-1 text-white bg-violet-600 hover:bg-violet-700 rounded-lg transition-colors;
}

.link-primary {
	@apply text-violet-600 hover:text-violet-800 dark:text-violet-400 dark:hover:text-violet-300;
}
</style>
