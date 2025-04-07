// plugins/auth.client.ts
// Ce plugin ne s'exécute que côté client
import { defineNuxtPlugin } from '#app';
import { useAuth } from '~/composables/useAuth';

export default defineNuxtPlugin(async (nuxtApp) => {
	const { fetchCurrentUser } = useAuth();

	// Attendre que Apollo soit disponible avant d'initialiser l'authentification
	nuxtApp.hook('app:created', () => {
		console.log('Auth plugin: App created, will initialize auth soon');

		// On utilise setTimeout pour s'assurer que tous les plugins sont chargés
		setTimeout(async () => {
			try {
				console.log('Auth plugin: Initializing auth state');
				await fetchCurrentUser(true);
			} catch (error) {
				console.error('Auth plugin: Error initializing auth', error);
			}
		}, 100);
	});

	// Vérifier l'authentification à chaque changement de route
	nuxtApp.hook('page:start', () => {
		console.log('Auth plugin: Page navigation detected');
		// On vérifie que Apollo est disponible avant d'appeler
		if (nuxtApp.$apollo) {
			// N'utilise pas await ici pour éviter de bloquer la navigation
			fetchCurrentUser().catch((err) => {
				console.error('Auth plugin: Error refreshing auth during navigation', err);
			});
		}
	});

	nuxtApp.hook('app:error', (err) => {
		console.log('Auth plugin: App error detected');
		// Vérifier que Apollo est disponible
		if (nuxtApp.$apollo) {
			console.log('Auth plugin: Refreshing auth after app error');
			// Rafraîchir l'authentification en cas d'erreur d'application
			fetchCurrentUser(true).catch((authErr) => {
				console.error('Auth plugin: Error refreshing auth after app error', authErr);
			});
		}
	});
});
