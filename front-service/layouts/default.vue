<template>
	<div
		class="flex flex-col min-h-screen font-sans transition duration-300 bg-gradient-to-tr from-violet-50 to-white dark:from-violet-950 dark:to-violet-900">
		<!-- Navbar toujours visible -->
		<Navbar />

		<main class="flex-1 px-4 py-8 mx-auto max-w-7xl sm:px-6 lg:px-8">
			<slot />
		</main>

		<footer
			class="py-4 mt-auto text-sm text-center text-gray-600 transition duration-300 shadow-inner bg-violet-100 rounded-t-2xl dark:bg-violet-900 dark:text-violet-200">
			© {{ new Date().getFullYear() }} SchoolInc. Tous droits réservés.
		</footer>
	</div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import Navbar from '~/components/Navbar.vue'
import { useAuth } from '~/composables/useAuth'

const { fetchCurrentUser } = useAuth()

onMounted(() => {
	console.log('Layout mounted, checking auth state')
	fetchCurrentUser().catch(error => {
		console.error('Error checking auth in layout:', error)
	})
})
</script>

<style scoped>
@import url('https://fonts.googleapis.com/css2?family=Poppins:wght@400;500;600;700&display=swap');

html {
	font-family: 'Poppins', sans-serif;
}
</style>