<template>
	<div
		class="flex flex-col min-h-screen font-sans transition duration-300 bg-gradient-to-tr from-violet-50 to-white dark:from-violet-950 dark:to-violet-900"
	>
		<!-- Navbar toujours visible -->
		<Navbar />

		<!-- Main sans contrainte de largeur fixe pour s'adapter aux composants enfants -->
		<main
			class="flex-1 py-8 mx-auto w-full"
			style="padding-top: 24px"
		>
			<slot />
		</main>

		<footer
			class="py-4 mt-auto text-sm text-center text-gray-600 transition duration-300 shadow-inner bg-violet-100 rounded-t-2xl dark:bg-violet-900 dark:text-violet-200"
		>
			© {{ new Date().getFullYear() }} SchoolInc. Tous droits réservés.
		</footer>
	</div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import Navbar from '~/components/Navbar.vue';
import { useAuth } from '~/composables/useAuth';

const { fetchCurrentUser } = useAuth();
const authInitialized = ref(false);

onMounted(async () => {
	try {
		await fetchCurrentUser();
		authInitialized.value = true;
	} catch (error) {
		console.error('Error checking auth in layout:', error);
	}
});
</script>

<style scoped>
@import url('https://fonts.googleapis.com/css2?family=Poppins:wght@400;500;600;700&display=swap');
html {
	font-family: 'Poppins', sans-serif;
}
</style>
