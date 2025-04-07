// composables/useCurrentUser.ts
import { computed, onMounted, getCurrentInstance } from 'vue';
import { useAuth } from './useAuth';

export function useCurrentUser() {
	const { user, isAuthenticated, fetchCurrentUser } = useAuth();

	// S'assurer que les données sont toujours disponibles
	const instance = getCurrentInstance();

	if (instance && process.client) {
		// Charger les données au montage du composant
		onMounted(() => {
			fetchCurrentUser();
		});
	}

	const currentUser = computed(() => user.value);

	return {
		currentUser,
		isAuthenticated,
		fetchCurrentUser,
	};
}
