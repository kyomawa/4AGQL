// composables/useCurrentUser.ts
import { computed, onMounted, getCurrentInstance } from 'vue';
import { useAuth } from './useAuth';

export function useCurrentUser() {
	const { user, isAuthenticated, fetchCurrentUser } = useAuth();
	const instance = getCurrentInstance();
	if (instance && process.client) {
		onMounted(async () => {
			await fetchCurrentUser();
		});
	}
	const currentUser = computed(() => user.value);
	const userRole = computed(() => user.value?.role || '');
	const isAdmin = computed(() => userRole.value === 'admin');
	const isProfessor = computed(() => userRole.value === 'professor');
	const isStudent = computed(() => userRole.value === 'student');

	return {
		currentUser,
		isAuthenticated,
		fetchCurrentUser,
		userRole,
		isAdmin,
		isProfessor,
		isStudent,
	};
}
