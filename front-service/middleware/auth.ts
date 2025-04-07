// middleware/auth.ts
import { useAuth } from '~/composables/useAuth';

export default defineNuxtRouteMiddleware((to) => {
	const { isAuthenticated, user } = useAuth();

	// Routes accessibles sans authentification
	const publicRoutes = ['/login', '/register', '/', '/classes'];
	// Vérifier également si c'est une page de détail de classe (qui est aussi publique)
	const isPublicClassDetail = to.path.startsWith('/classes/');

	// Si l'utilisateur n'est pas authentifié et la route n'est pas publique
	if (!isAuthenticated.value && !publicRoutes.includes(to.path) && !isPublicClassDetail) {
		console.log('Middleware: Auth check failed, redirecting to login');
		return navigateTo('/login');
	}

	// Si l'utilisateur est authentifié et essaie d'accéder aux pages de login/register
	if (isAuthenticated.value && (to.path === '/login' || to.path === '/register')) {
		console.log('Middleware: Already authenticated, redirecting to home');
		return navigateTo('/');
	}

	console.log('Middleware: Auth check passed for', to.path);
});
