<template>
	<div
		class="card p-6 transition duration-300 hover:shadow-lg h-48 flex flex-col justify-between"
	>
		<h3 class="text-center text-xl font-semibold text-violet-700 dark:text-violet-300 mb-2">
			{{ classItem.name }}
		</h3>
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
/* Styles repris du register.vue */
.card {
	@apply bg-white shadow-lg rounded-2xl transition duration-300 dark:bg-violet-950;
}

.btn-primary {
	@apply py-2 px-4 font-medium text-white transition-colors bg-violet-600 rounded-lg hover:bg-violet-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-violet-500 disabled:opacity-50 disabled:cursor-not-allowed;
}

.link-primary {
	@apply text-violet-600 hover:text-violet-800 dark:text-violet-400 dark:hover:text-violet-300;
}
</style>
