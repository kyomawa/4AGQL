<template>
	<Head>
		<link
			rel="icon"
			type="image/png"
			href="/logo.png"
		/>
	</Head>
	<NuxtLayout>
		<NuxtPage />
	</NuxtLayout>
</template>

<script setup>
import { onMounted } from 'vue';
import { useAuth } from '~/composables/useAuth';

// Initialiser l'authentification au chargement de l'application
const { fetchCurrentUser } = useAuth();

onMounted(async () => {
	try {
		// Cette ligne est CRUCIALE - elle charge l'utilisateur au démarrage de l'application
		await fetchCurrentUser(true);
	} catch (error) {
		console.error('App.vue - Auth initialization error:', error);
	}
});
</script>
