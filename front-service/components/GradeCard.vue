<template>
	<div class="card p-6 transition duration-300 hover:shadow-lg h-48 flex flex-col justify-start">
		<!-- Titre du cours aligné à gauche -->
		<h3 class="text-left text-xl font-semibold text-violet-700 dark:text-violet-300 mb-2">
			{{ grade.course }}
		</h3>
		<!-- Affichage du nom de la classe -->
		<p class="text-left text-sm text-gray-700 dark:text-gray-300 mb-2">
			Classe :
			<span class="font-bold text-violet-600 dark:text-violet-400">{{
				grade.className
			}}</span>
		</p>
		<!-- Bloc d'informations sur la note, aligné à gauche -->
		<div class="flex flex-col items-start space-y-2">
			<p class="text-left text-gray-700 dark:text-gray-300">
				Commentaire :
				<span class="font-bold text-violet-600 dark:text-violet-400">{{ grade.note }}</span>
			</p>
			<p class="text-left text-gray-700 dark:text-gray-300">
				Score :
				<span class="font-bold text-violet-600 dark:text-violet-400">{{
					grade.grade
				}}</span>
			</p>
		</div>
	</div>
</template>

<script setup lang="ts">
defineProps({
	grade: {
		type: Object,
		required: true,
	},
});
</script>

<style scoped>
.card {
	@apply bg-white shadow-lg rounded-2xl transition duration-300 dark:bg-violet-950;
}

.link-primary {
	@apply text-violet-600 hover:text-violet-800 dark:text-violet-400 dark:hover:text-violet-300;
}
</style>
