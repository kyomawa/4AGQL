// middleware/role.ts
import { useAuth } from '~/composables/useAuth';

export default defineNuxtRouteMiddleware((to) => {
	const { user, isAuthenticated } = useAuth();

	const requiredRole = to.meta?.role as string | undefined;
	const userRole = user.value?.role;

	console.log('[Role Middleware] Route:', to.path);
	console.log('[Role Middleware] Required role:', requiredRole);
	console.log('[Role Middleware] User role:', userRole);

	// Si pas de rôle requis sur cette page, on continue
	if (!requiredRole) return;

	// Si on n'est pas authentifié, rediriger vers la connexion
	if (!isAuthenticated.value || !userRole) {
		console.log('[Role Middleware] Not authenticated, redirecting to login');
		return navigateTo('/login');
	}

	// Vérifier si l'utilisateur a le rôle requis (avec hiérarchie)
	const hasAccess =
		(requiredRole === 'admin' && userRole === 'admin') ||
		(requiredRole === 'professor' && (userRole === 'professor' || userRole === 'admin')) ||
		(requiredRole === 'student' &&
			(userRole === 'student' || userRole === 'professor' || userRole === 'admin'));

	// Si l'utilisateur n'a pas l'autorisation, rediriger vers l'accueil
	if (!hasAccess) {
		console.log('[Role Middleware] Access denied. Redirecting to /');
		return navigateTo('/');
	}
});
