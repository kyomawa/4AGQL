<template>
	<header
		class="sticky top-0 z-50 bg-white border-b shadow-md dark:bg-violet-950 dark:border-violet-900"
	>
		<nav class="px-4 py-4 mx-auto max-w-7xl sm:px-6 lg:px-8">
			<div class="flex items-center justify-between">
				<div class="flex items-center">
					<NuxtLink
						to="/"
						class="flex items-center"
					>
						<img
							src="/logo.png"
							alt="SchoolInc Logo"
							class="w-10 h-10 mr-3"
						/>
						<span
							class="text-2xl font-bold text-violet-600 hover:text-violet-800 dark:text-violet-300 dark:hover:text-violet-200"
						>
							SchoolInc
						</span>
					</NuxtLink>
				</div>

				<button
					@click="isOpen = !isOpen"
					class="text-gray-600 dark:text-violet-300 md:hidden"
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="w-6 h-6"
						fill="none"
						viewBox="0 0 24 24"
						stroke="currentColor"
					>
						<path
							v-if="isOpen"
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M6 18L18 6M6 6l12 12"
						/>
						<path
							v-else
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M4 6h16M4 12h16M4 18h16"
						/>
					</svg>
				</button>

				<div class="items-center hidden space-x-4 md:flex">
					<NuxtLink
						class="nav-item"
						:class="{ 'nav-active': isActiveRoute('/') }"
						to="/"
					>
						Accueil
					</NuxtLink>
					<NuxtLink
						class="nav-item"
						:class="{ 'nav-active': isActiveRoute('/classes') }"
						to="/classes"
					>
						Classes
					</NuxtLink>
					<!-- Bouton "Mes Classes" visible uniquement pour les utilisateurs (student) -->
					<NuxtLink
						v-if="isAuthenticated && (userRole === 'student' || userRole === 'USER')"
						class="nav-item"
						:class="{ 'nav-active': isActiveRoute('/myclasses') }"
						to="/myclasses"
					>
						Mes Classes
					</NuxtLink>

					<!-- Menu pour utilisateurs non authentifiés -->
					<template v-if="!isAuthenticated">
						<NuxtLink
							class="nav-item"
							:class="{ 'nav-active': isActiveRoute('/login') }"
							to="/login"
						>
							Connexion
						</NuxtLink>
						<NuxtLink
							class="btn-primary"
							to="/register"
						>
							S'inscrire
						</NuxtLink>
					</template>

					<!-- Menu pour utilisateurs authentifiés -->
					<template v-else>
						<!-- Menu étudiant -->
						<NuxtLink
							v-if="userRole === 'student' || userRole === 'USER'"
							class="nav-item"
							:class="{ 'nav-active': isActiveRoute('/mygrades') }"
							to="/mygrades"
						>
							Mes Notes
						</NuxtLink>

						<!-- Menu Gestion pour Professeurs -->
						<div
							v-if="userRole === 'professor' || userRole === 'PROFESSOR'"
							class="relative group"
						>
							<button
								class="inline-flex items-center nav-item"
								:class="{ 'nav-active': isProfessorRoute }"
							>
								Gestion
								<svg
									class="w-4 h-4 ml-1"
									fill="none"
									viewBox="0 0 24 24"
									stroke="currentColor"
								>
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M19 9l-7 7-7-7"
									/>
								</svg>
							</button>
							<div
								class="absolute hidden w-48 bg-white rounded-md shadow-lg dark:bg-violet-900 group-hover:block"
							>
								<!-- Liens accessibles aux profs -->
								<NuxtLink
									class="dropdown-item"
									:class="{
										'dropdown-active': isActiveRoute('/professor/grades'),
									}"
									to="/professor/grades"
								>
									Notes
								</NuxtLink>
								<NuxtLink
									class="dropdown-item"
									:class="{
										'dropdown-active': isActiveRoute('/professor/classes'),
									}"
									to="/professor/classes"
								>
									Classes
								</NuxtLink>
							</div>
						</div>

						<!-- Menu Administration pour Admins uniquement -->
						<div
							v-if="userRole === 'admin' || userRole === 'ADMIN'"
							class="relative group"
						>
							<button
								class="inline-flex items-center nav-item"
								:class="{ 'nav-active': isAdminRoute }"
							>
								Administration
								<svg
									class="w-4 h-4 ml-1"
									fill="none"
									viewBox="0 0 24 24"
									stroke="currentColor"
								>
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M19 9l-7 7-7-7"
									/>
								</svg>
							</button>
							<div
								class="absolute hidden w-48 bg-white rounded-md shadow-lg dark:bg-violet-900 group-hover:block"
							>
								<NuxtLink
									class="dropdown-item"
									:class="{
										'dropdown-active': isActiveRoute('/admin/users'),
									}"
									to="/admin/users"
								>
									Utilisateurs
								</NuxtLink>
								<NuxtLink
									class="dropdown-item"
									:class="{
										'dropdown-active': isActiveRoute('/admin/grades'),
									}"
									to="/admin/grades"
								>
									Grades
								</NuxtLink>
							</div>
						</div>

						<!-- Menu compte utilisateur (commun) -->
						<div class="relative group">
							<button
								class="inline-flex items-center nav-item"
								:class="{ 'nav-active': isActiveRoute('/profile') }"
							>
								Bienvenue {{ user?.firstName }}
								<svg
									class="w-4 h-4 ml-1"
									fill="none"
									viewBox="0 0 24 24"
									stroke="currentColor"
								>
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M19 9l-7 7-7-7"
									/>
								</svg>
							</button>
							<div
								class="absolute hidden w-48 bg-white rounded-md shadow-lg dark:bg-violet-900 group-hover:block"
							>
								<NuxtLink
									class="dropdown-item"
									to="/profile"
								>
									Mon Profil
								</NuxtLink>
								<button
									@click="handleLogout"
									class="w-full px-4 py-2 text-left text-red-600 hover:bg-violet-100 dark:hover:bg-violet-800"
								>
									Déconnexion
								</button>
							</div>
						</div>
					</template>
				</div>

				<!-- Menu Mobile -->
				<div
					v-if="isOpen"
					class="absolute left-0 w-full bg-white top-full dark:bg-violet-950 md:hidden"
				>
					<div class="px-4 pt-2 pb-4 space-y-2">
						<NuxtLink
							class="block px-3 py-2 text-gray-700 dark:text-violet-300 hover:bg-violet-100 dark:hover:bg-violet-800 rounded-[20px]"
							to="/"
							@click="isOpen = false"
						>
							Accueil
						</NuxtLink>
						<NuxtLink
							class="block px-3 py-2 text-gray-700 dark:text-violet-300 hover:bg-violet-100 dark:hover:bg-violet-800 rounded-[20px]"
							to="/classes"
							@click="isOpen = false"
						>
							Classes
						</NuxtLink>
						<!-- Bouton "Mes Classes" pour mobile -->
						<NuxtLink
							v-if="
								isAuthenticated && (userRole === 'student' || userRole === 'USER')
							"
							class="block px-3 py-2 text-gray-700 dark:text-violet-300 hover:bg-violet-100 dark:hover:bg-violet-800 rounded-[20px]"
							to="/myclasses"
							@click="isOpen = false"
						>
							Mes Classes
						</NuxtLink>

						<template v-if="!isAuthenticated">
							<NuxtLink
								class="block px-3 py-2 text-gray-700 dark:text-violet-300 hover:bg-violet-100 dark:hover:bg-violet-800 rounded-[20px]"
								to="/login"
								@click="isOpen = false"
							>
								Connexion
							</NuxtLink>
							<NuxtLink
								class="block px-3 py-2 text-white bg-violet-600 hover:bg-violet-700 rounded-[20px]"
								to="/register"
								@click="isOpen = false"
							>
								S'inscrire
							</NuxtLink>
						</template>

						<template v-else>
							<NuxtLink
								v-if="userRole === 'student' || userRole === 'USER'"
								class="block px-3 py-2 text-gray-700 dark:text-violet-300 hover:bg-violet-100 dark:hover:bg-violet-800 rounded-[20px]"
								to="/mygrades"
								@click="isOpen = false"
							>
								Mes Notes
							</NuxtLink>

							<!-- Menu Professeur (Mobile) -->
							<div v-if="userRole === 'professor' || userRole === 'PROFESSOR'">
								<div class="px-3 py-2 text-gray-700 dark:text-violet-300">
									Gestion
								</div>
								<div class="pl-4 space-y-2">
									<NuxtLink
										class="block px-3 py-2 text-gray-700 dark:text-violet-300 hover:bg-violet-100 dark:hover:bg-violet-800 rounded-[20px]"
										to="/professor/grades"
										@click="isOpen = false"
									>
										Notes
									</NuxtLink>
									<NuxtLink
										class="block px-3 py-2 text-gray-700 dark:text-violet-300 hover:bg-violet-100 dark:hover:bg-violet-800 rounded-[20px]"
										to="/professor/classes"
										@click="isOpen = false"
									>
										Classes
									</NuxtLink>
								</div>
							</div>

							<!-- Menu Admin (Mobile) -->
							<div v-if="userRole === 'admin' || userRole === 'ADMIN'">
								<div class="px-3 py-2 text-gray-700 dark:text-violet-300">
									Administration
								</div>
								<div class="pl-4 space-y-2">
									<NuxtLink
										class="block px-3 py-2 text-gray-700 dark:text-violet-300 hover:bg-violet-100 dark:hover:bg-violet-800 rounded-[20px]"
										to="/admin/users"
										@click="isOpen = false"
									>
										Utilisateurs
									</NuxtLink>
									<NuxtLink
										class="block px-3 py-2 text-gray-700 dark:text-violet-300 hover:bg-violet-100 dark:hover:bg-violet-800 rounded-[20px]"
										to="/admin/grades"
										@click="isOpen = false"
									>
										Grades
									</NuxtLink>
								</div>
							</div>

							<div>
								<div class="px-3 py-2 text-gray-700 dark:text-violet-300">
									Bienvenue {{ user?.firstName }}
								</div>
								<div class="pl-4 space-y-2">
									<NuxtLink
										class="block px-3 py-2 text-gray-700 dark:text-violet-300 hover:bg-violet-100 dark:hover:bg-violet-800 rounded-[20px]"
										to="/profile"
										@click="isOpen = false"
									>
										Mon Profil
									</NuxtLink>
									<button
										@click="handleLogout"
										class="w-full px-3 py-2 text-left text-red-600 hover:bg-violet-100 dark:hover:bg-violet-800 rounded-[20px]"
									>
										Déconnexion
									</button>
								</div>
							</div>
						</template>
					</div>
				</div>
			</div>
		</nav>
	</header>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { useAuth } from '~/composables/useAuth';

const isOpen = ref(false);
const router = useRouter();
const route = useRoute();

// Utiliser directement les états partagés de auth
const { user, isAuthenticated, logout, fetchCurrentUser } = useAuth();

// Récupérer le rôle de l'utilisateur
const userRole = computed(() => {
	return user.value?.role || '';
});

// Initialiser l'état au montage
onMounted(async () => {
	console.log('Navbar mounted, ensuring auth state');
	try {
		// Rafraîchir l'authentification au montage du composant
		await fetchCurrentUser();
		console.log('Auth state refreshed, role:', userRole.value);
	} catch (error) {
		console.error('Error checking auth in navbar:', error);
	}
});

// Active route detection
const isActiveRoute = (path: string) => {
	return route.path === path;
};

// Check if any professor route is active
const isProfessorRoute = computed(() => {
	return route.path.startsWith('/professor/');
});

// Check if any admin route is active
const isAdminRoute = computed(() => {
	return route.path.startsWith('/admin/');
});

// Logout handler
const handleLogout = async () => {
	console.log('Logout requested');
	try {
		await logout();
		isOpen.value = false;
		router.push('/login');
	} catch (error) {
		console.error('Error during logout:', error);
		// En cas d'erreur, quand même rediriger
		router.push('/login');
	}
};
</script>

<style scoped>
.nav-item {
	@apply px-3 py-2 text-gray-700 dark:text-violet-300 hover:text-violet-600 dark:hover:text-violet-100 rounded-[20px] transition-colors;
}

.nav-active {
	@apply text-violet-600 dark:text-violet-100 bg-violet-50 dark:bg-violet-800;
}

.btn-primary {
	@apply px-3 py-2 text-white bg-violet-600 hover:bg-violet-700 rounded-[20px] transition-colors;
}

.dropdown-item {
	@apply block px-4 py-2 text-sm text-gray-700 dark:text-violet-200 hover:bg-violet-50 dark:hover:bg-violet-800;
}
</style>
