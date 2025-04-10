// middleware/auth.ts
import { useAuth } from '~/composables/useAuth';

export default defineNuxtRouteMiddleware((to) => {
	const { isAuthenticated } = useAuth();

	// Routes accessibles sans authentification
	const publicRoutes = ['/login', '/register', '/', '/classes'];
	// La page de détail d'une classe (ex.: /classes/12345) est également publique
	const isPublicClassDetail = to.path.startsWith('/classes/');

	// Si l'utilisateur n'est pas authentifié et la route n'est pas publique, rediriger vers la page d'accueil
	if (!isAuthenticated.value && !publicRoutes.includes(to.path) && !isPublicClassDetail) {
		console.log('Middleware: Auth check failed, redirecting to home');
		return navigateTo('/');
	}

	// Si l'utilisateur est authentifié et essaie d'accéder aux pages de login/register, rediriger vers la page d'accueil
	if (isAuthenticated.value && (to.path === '/login' || to.path === '/register')) {
		console.log('Middleware: Already authenticated, redirecting to home');
		return navigateTo('/');
	}

	console.log('Middleware: Auth check passed for', to.path);
});
