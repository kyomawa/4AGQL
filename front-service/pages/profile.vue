<template>
  <div class="min-h-screen py-12">
    <div class="max-w-3xl mx-auto">

      <Head>
        <title>Profil - SchoolInc</title>
      </Head>

      <h1 class="page-title">Votre profil</h1>

      <div class="p-8 shadow-md bg-violet-950 rounded-2xl">
        <div class="space-y-6">
          <div class="profile-item">
            <span class="profile-label">Nom :</span>
            <span class="profile-value">{{ userProfile.pseudo }}</span>
          </div>

          <div class="profile-item">
            <span class="profile-label">Email :</span>
            <span class="profile-value">{{ userProfile.email }}</span>
          </div>

          <div class="profile-item">
            <span class="profile-label">Rôle :</span>
            <span :class="['px-2 py-1 rounded-full text-sm', roleColorClass]">
              {{ formattedRole }}
            </span>
          </div>

          <div class="flex mt-8 space-x-4">
            <button @click="startEdit"
              class="btn-primary px-6 py-2 rounded-full bg-violet-600 hover:bg-violet-700 text-white font-semibold transition">
              Modifier le profil
            </button>
            <button @click="showDeleteConfirm = true" class="btn-delete">
              Supprimer le compte
            </button>
          </div>
        </div>
      </div>

      <!-- Modal de confirmation de suppression -->
      <div v-if="showDeleteConfirm" class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50">
        <div class="w-full max-w-md p-8 bg-violet-950 rounded-2xl">
          <h3 class="mb-4 text-xl font-bold text-violet-200">Confirmer la suppression</h3>
          <p class="mb-6 text-violet-300">Êtes-vous sûr de vouloir supprimer votre compte ? Cette action est
            irréversible.</p>
          <div class="flex space-x-4">
            <button @click="deleteAccount" :disabled="deleting"
              class="flex-1 px-4 py-2 font-semibold text-white transition bg-red-600 rounded-full hover:bg-red-700">
              {{ deleting ? 'Suppression...' : 'Confirmer la suppression' }}
            </button>
            <button @click="showDeleteConfirm = false"
              class="flex-1 px-4 py-2 font-semibold transition rounded-full bg-violet-800 hover:bg-violet-700 text-violet-200">
              Annuler
            </button>
          </div>
        </div>
      </div>

      <!-- Formulaire d'édition -->
      <div v-if="isEditing" class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50">
        <div class="w-full max-w-md p-8 bg-violet-950 rounded-2xl">
          <h3 class="mb-6 text-xl font-bold text-violet-200">Modifier le profil</h3>
          <div class="space-y-4">
            <div>
              <label class="block mb-2 text-violet-300">Nom</label>
              <input v-model="userProfile.pseudo" type="text"
                class="w-full px-4 py-2 text-white rounded-full bg-violet-900 focus:outline-none focus:ring-2 focus:ring-violet-600" />
            </div>
            <div class="flex space-x-4">
              <button @click="saveProfile" :disabled="loading"
                class="flex-1 px-4 py-2 font-semibold text-white transition rounded-full bg-violet-600 hover:bg-violet-700">
                {{ loading ? 'Sauvegarde...' : 'Enregistrer' }}
              </button>
              <button @click="cancelEdit"
                class="flex-1 px-4 py-2 font-semibold transition rounded-full bg-violet-800 hover:bg-violet-700 text-violet-200">
                Annuler
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Gestion des erreurs -->
      <div v-if="error" class="fixed px-4 py-2 text-white bg-red-600 rounded-lg shadow-md top-4 right-4">
        {{ error }}
        <button @click="error = null" class="ml-4 text-red-100 hover:text-white">×</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import { useRouter } from 'vue-router';
import gql from 'graphql-tag';
import { useNuxtApp } from '#app';
import { useAuth } from '~/composables/useAuth';

definePageMeta({
  middleware: ['auth'],
});

interface UserProfile {
  id: string;
  email: string;
  pseudo: string;
  role: string;
}

const router = useRouter();
const { user, isAuthenticated, logout } = useAuth();
const nuxtApp = useNuxtApp();

const loading = ref(false);
const error = ref<string | null>(null);
const isEditing = ref(false);
const showDeleteConfirm = ref(false);
const deleting = ref(false);
const originalProfile = ref<UserProfile | null>(null);
const userProfile = ref<UserProfile>({
  id: '',
  email: '',
  pseudo: '',
  role: ''
});

// GraphQL queries
const GET_CURRENT_USER = gql`
  query {
    get_current_user {
      id
      email
      pseudo
      role
    }
  }
`;

const UPDATE_USER = gql`
  mutation UpdateUser($input: UserInput!) {
    update_current_user(input: $input) {
      id
      email
      pseudo
      role
    }
  }
`;

const DELETE_USER = gql`
  mutation {
    delete_current_user {
      id
    }
  }
`;

// Load user data directly with Apollo
const loadUserData = async () => {
  loading.value = true;
  error.value = null;

  try {
    // Vérifier si nous avons déjà les données utilisateur de l'authentification
    if (user.value) {
      userProfile.value = { ...user.value } as UserProfile;
      originalProfile.value = { ...user.value } as UserProfile;
      loading.value = false;
      return;
    }

    // Vérifier si Apollo est disponible
    if (!nuxtApp.$apollo) {
      console.error("Apollo n'est pas disponible, on réessaiera plus tard");
      error.value = "Le service n'est pas encore disponible. Veuillez réessayer.";
      loading.value = false;
      return;
    }

    // Sinon, charger depuis l'API
    const result = await nuxtApp.$apollo.query({
      query: GET_CURRENT_USER,
      fetchPolicy: 'network-only'
    });

    if (result.data?.get_current_user) {
      userProfile.value = { ...result.data.get_current_user };
      originalProfile.value = { ...result.data.get_current_user };
    } else {
      error.value = "Impossible de récupérer vos informations de profil.";
    }
  } catch (err) {
    console.error('Error loading user profile:', err);
    error.value = "Impossible de charger votre profil. Veuillez réessayer.";
  } finally {
    loading.value = false;
  }
};

// Start editing profile
const startEdit = () => {
  if (originalProfile.value) {
    originalProfile.value = { ...userProfile.value };
  } else {
    originalProfile.value = { ...userProfile.value };
  }
  isEditing.value = true;
};

// Cancel editing
const cancelEdit = () => {
  if (originalProfile.value) {
    userProfile.value = { ...originalProfile.value };
  }
  isEditing.value = false;
};

// Save profile changes
const saveProfile = async () => {
  loading.value = true;

  try {
    if (!nuxtApp.$apollo) {
      error.value = "Le service n'est pas encore disponible. Veuillez réessayer.";
      loading.value = false;
      return;
    }

    const result = await nuxtApp.$apollo.mutate({
      mutation: UPDATE_USER,
      variables: {
        input: {
          pseudo: userProfile.value.pseudo
        }
      }
    });

    if (result.data?.update_current_user) {
      userProfile.value = { ...result.data.update_current_user };
      originalProfile.value = { ...result.data.update_current_user };
      isEditing.value = false;
    } else {
      error.value = "Erreur lors de la mise à jour du profil.";
    }
  } catch (err) {
    console.error('Error updating profile:', err);
    error.value = "Impossible de mettre à jour votre profil. Veuillez réessayer.";
  } finally {
    loading.value = false;
  }
};

// Delete account
const deleteAccount = async () => {
  deleting.value = true;

  try {
    if (!nuxtApp.$apollo) {
      error.value = "Le service n'est pas encore disponible. Veuillez réessayer.";
      deleting.value = false;
      return;
    }

    await nuxtApp.$apollo.mutate({
      mutation: DELETE_USER
    });

    await logout();
    router.push('/login');
  } catch (err) {
    console.error('Error deleting account:', err);
    error.value = "Impossible de supprimer votre compte. Veuillez réessayer.";
    showDeleteConfirm.value = false;
  } finally {
    deleting.value = false;
  }
};

// Computed properties
const formattedRole = computed(() => {
  const role = userProfile.value.role;
  if (role === 'admin') return 'Administrateur';
  if (role === 'professor') return 'Professeur';
  if (role === 'student') return 'Étudiant';
  return role;
});

const roleColorClass = computed(() => {
  const role = userProfile.value.role;
  if (role === 'admin') return 'bg-purple-600 text-white';
  if (role === 'professor') return 'bg-blue-600 text-white';
  if (role === 'student') return 'bg-green-600 text-white';
  return 'bg-gray-600 text-white';
});

// Réagir aux changements de l'état d'authentification
watch(user, () => {
  if (user.value && !userProfile.value.id) {
    userProfile.value = { ...user.value } as UserProfile;
    originalProfile.value = { ...user.value } as UserProfile;
  }
});

// Charger les données au montage du composant
onMounted(() => {
  // Cette fonction ne contient AUCUNE référence à refetch
  loadUserData();

  // Charger les données après un petit délai si Apollo n'était pas prêt
  if (!nuxtApp.$apollo) {
    setTimeout(() => {
      loadUserData();
    }, 1000);
  }
});
</script>

<style scoped>
.page-title {
  @apply text-4xl font-bold text-white mb-8 text-center;
}

.profile-item {
  @apply flex justify-between py-4 border-b border-violet-800;
}

.profile-label {
  @apply font-semibold text-violet-300;
}

.profile-value {
  @apply text-violet-100;
}

.btn-delete {
  @apply bg-red-600 hover:bg-red-700 text-white font-semibold px-6 py-2 rounded-full transition;
}
</style>