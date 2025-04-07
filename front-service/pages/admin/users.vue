<template>
	<div>
		<h1 class="mb-6 text-3xl font-bold">Manage Users</h1>

		<div
			v-if="loading"
			class="py-8 text-center"
		>
			<div
				class="w-12 h-12 mx-auto border-b-2 border-blue-500 rounded-full animate-spin"
			></div>
			<p class="mt-4 text-gray-600">Loading users...</p>
		</div>

		<div v-else>
			<!-- Controls -->
			<div class="p-4 mb-6 bg-white rounded-lg shadow">
				<div class="flex flex-wrap gap-3">
					<div class="flex-1">
						<label class="block mb-2 text-sm font-medium text-gray-700"
							>Search Users</label
						>
						<input
							type="text"
							v-model="searchQuery"
							placeholder="Search by email, pseudo or ID"
							class="w-full px-4 py-2 border rounded-md focus:ring-blue-500 focus:border-blue-500"
						/>
					</div>
					<div>
						<label class="block mb-2 text-sm font-medium text-gray-700"
							>Filter by Role</label
						>
						<select
							v-model="roleFilter"
							class="w-full px-4 py-2 border rounded-md focus:ring-blue-500 focus:border-blue-500"
						>
							<option value="">All Roles</option>
							<option value="student">Student</option>
							<option value="professor">Professor</option>
							<option value="admin">Admin</option>
						</select>
					</div>
				</div>
			</div>

			<!-- Users Table -->
			<div
				v-if="filteredUsers.length === 0"
				class="py-8 text-center bg-white rounded-lg shadow"
			>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					class="w-16 h-16 mx-auto text-gray-400"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z"
					/>
				</svg>
				<h3 class="mt-4 text-lg font-medium text-gray-900">No users found</h3>
				<p class="mt-1 text-gray-500">Try changing your search or filter criteria.</p>
			</div>

			<div
				v-else
				class="overflow-hidden bg-white rounded-lg shadow"
			>
				<table class="min-w-full divide-y divide-gray-200">
					<thead class="bg-gray-50">
						<tr>
							<th
								class="px-6 py-3 text-xs font-medium tracking-wider text-left text-gray-500 uppercase"
							>
								ID
							</th>
							<th
								class="px-6 py-3 text-xs font-medium tracking-wider text-left text-gray-500 uppercase"
							>
								Email
							</th>
							<th
								class="px-6 py-3 text-xs font-medium tracking-wider text-left text-gray-500 uppercase"
							>
								Pseudo
							</th>
							<th
								class="px-6 py-3 text-xs font-medium tracking-wider text-left text-gray-500 uppercase"
							>
								Role
							</th>
							<th
								class="px-6 py-3 text-xs font-medium tracking-wider text-right text-gray-500 uppercase"
							>
								Actions
							</th>
						</tr>
					</thead>
					<tbody class="bg-white divide-y divide-gray-200">
						<tr
							v-for="user in filteredUsers"
							:key="user.id"
							class="hover:bg-gray-50"
						>
							<td class="px-6 py-4 whitespace-nowrap">
								<div class="text-sm font-medium text-gray-900">{{ user.id }}</div>
							</td>
							<td class="px-6 py-4 whitespace-nowrap">
								<div class="text-sm text-gray-900">{{ user.email }}</div>
							</td>
							<td class="px-6 py-4 whitespace-nowrap">
								<div class="text-sm text-gray-900">{{ user.pseudo }}</div>
							</td>
							<td class="px-6 py-4 whitespace-nowrap">
								<span
									class="inline-flex px-2 text-xs font-semibold leading-5 rounded-full"
									:class="roleColorClass(user.role)"
								>
									{{ user.role }}
								</span>
							</td>
							<td class="px-6 py-4 text-sm font-medium text-right whitespace-nowrap">
								<button
									@click="editUser(user)"
									class="mr-3 text-indigo-600 hover:text-indigo-900"
								>
									Edit
								</button>
								<button
									@click="confirmDeleteUser(user)"
									class="text-red-600 hover:text-red-900"
								>
									Delete
								</button>
							</td>
						</tr>
					</tbody>
				</table>
			</div>
		</div>

		<!-- Edit User Modal -->
		<div
			v-if="showEditModal"
			class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50"
		>
			<div class="w-full max-w-md p-6 bg-white rounded-lg">
				<h2 class="mb-4 text-xl font-bold">Edit User</h2>

				<form
					@submit.prevent="updateUser"
					class="space-y-4"
				>
					<div>
						<label class="block mb-1 text-sm font-medium text-gray-700">Email</label>
						<input
							v-model="userForm.email"
							type="email"
							class="w-full px-4 py-2 border rounded-md focus:ring-blue-500 focus:border-blue-500"
							readonly
						/>
						<p class="mt-1 text-xs text-gray-500">Email cannot be changed</p>
					</div>

					<div>
						<label class="block mb-1 text-sm font-medium text-gray-700">Pseudo</label>
						<input
							v-model="userForm.pseudo"
							type="text"
							class="w-full px-4 py-2 border rounded-md focus:ring-blue-500 focus:border-blue-500"
							required
						/>
					</div>

					<div>
						<label class="block mb-1 text-sm font-medium text-gray-700">Role</label>
						<select
							v-model="userForm.role"
							class="w-full px-4 py-2 border rounded-md focus:ring-blue-500 focus:border-blue-500"
							required
						>
							<option value="student">Student</option>
							<option value="professor">Professor</option>
							<option value="admin">Admin</option>
						</select>
					</div>

					<div class="flex justify-end pt-4 space-x-3">
						<button
							type="button"
							@click="showEditModal = false"
							class="px-4 py-2 transition border rounded-md hover:bg-gray-100"
						>
							Cancel
						</button>
						<button
							type="submit"
							class="px-4 py-2 text-white transition bg-blue-600 rounded-md hover:bg-blue-700"
						>
							Update
						</button>
					</div>
				</form>
			</div>
		</div>

		<!-- Delete User Confirmation Modal -->
		<div
			v-if="showDeleteModal"
			class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50"
		>
			<div class="w-full max-w-md p-6 bg-white rounded-lg">
				<h2 class="mb-4 text-xl font-bold">Delete User</h2>
				<p class="mb-6 text-gray-700">
					Are you sure you want to delete this user? This action cannot be undone and will
					remove all associated data.
				</p>
				<div class="flex justify-end space-x-3">
					<button
						@click="showDeleteModal = false"
						class="px-4 py-2 transition border rounded-md hover:bg-gray-100"
					>
						Cancel
					</button>
					<button
						@click="deleteUser"
						class="px-4 py-2 text-white transition bg-red-600 rounded-md hover:bg-red-700"
					>
						Delete
					</button>
				</div>
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue';
import { useQuery, useMutation } from '@vue/apollo-composable';
import gql from 'graphql-tag';
import { useCurrentUser } from '~/composables/useCurrentUser';

definePageMeta({
	middleware: ['auth'],
});

const { currentUser } = useCurrentUser();
const loading = ref(true);
const users = ref<any[]>([]);
const searchQuery = ref('');
const roleFilter = ref('');

// Modal states
const showEditModal = ref(false);
const showDeleteModal = ref(false);

// Form data
const userForm = reactive({
	id: '',
	email: '',
	pseudo: '',
	role: '',
});

// Currently selected user for edit/delete
const selectedUser = ref<any>(null);

// Filter users based on search query and role filter
const filteredUsers = computed(() => {
	let result = users.value;

	if (roleFilter.value) {
		result = result.filter((user) => user.role === roleFilter.value);
	}

	if (searchQuery.value) {
		const query = searchQuery.value.toLowerCase();
		result = result.filter(
			(user) =>
				user.id.toLowerCase().includes(query) ||
				user.email.toLowerCase().includes(query) ||
				user.pseudo.toLowerCase().includes(query),
		);
	}

	return result;
});

// GraphQL Queries and Mutations
const GET_USERS_QUERY = gql`
	query GetUsers {
		get_users {
			id
			email
			pseudo
			role
		}
	}
`;

const UPDATE_USER_MUTATION = gql`
	mutation UpdateUser($id: String!, $input: UserInput!) {
		update_user_by_id(id: $id, input: $input) {
			id
			email
			pseudo
			role
		}
	}
`;

const DELETE_USER_MUTATION = gql`
	mutation DeleteUser($id: String!) {
		delete_user_by_id(id: $id) {
			id
		}
	}
`;

// Execute Query
const { result, loading: queryLoading, refetch } = useQuery(GET_USERS_QUERY);

// Setup Mutations
const { mutate: updateUserMutation } = useMutation(UPDATE_USER_MUTATION);
const { mutate: deleteUserMutation } = useMutation(DELETE_USER_MUTATION);

// Watch for query result and update users
watch(result, (newResult) => {
	if (newResult?.get_users) {
		users.value = newResult.get_users;
	}
});

// Watch for loading state and update local loading state
watch(queryLoading, (isLoading) => {
	loading.value = isLoading;
});

onMounted(async () => {
	await refetch();
});

// Methods
const editUser = (user: any) => {
	selectedUser.value = user;

	// Copy user data to form
	userForm.id = user.id;
	userForm.email = user.email;
	userForm.pseudo = user.pseudo;
	userForm.role = user.role;

	showEditModal.value = true;
};

const confirmDeleteUser = (user: any) => {
	selectedUser.value = user;
	showDeleteModal.value = true;
};

const updateUser = async () => {
	if (!selectedUser.value) return;

	try {
		await updateUserMutation({
			id: userForm.id,
			input: {
				pseudo: userForm.pseudo,
				role: userForm.role,
			},
		});

		showEditModal.value = false;
		await refetch();
	} catch (error) {
		console.error('Error updating user:', error);
		alert('Failed to update user.');
	}
};

const deleteUser = async () => {
	if (!selectedUser.value) return;

	// Prevent deleting yourself
	if (selectedUser.value.id === currentUser.value?.user_id) {
		alert('You cannot delete your own account from this page. Use the profile page instead.');
		showDeleteModal.value = false;
		return;
	}

	try {
		await deleteUserMutation({
			id: selectedUser.value.id,
		});

		showDeleteModal.value = false;
		selectedUser.value = null;
		await refetch();
	} catch (error) {
		console.error('Error deleting user:', error);
		alert('Failed to delete user.');
	}
};

// Helper functions
const roleColorClass = (role: string) => {
	switch (role) {
		case 'admin':
			return 'bg-purple-100 text-purple-800';
		case 'professor':
			return 'bg-green-100 text-green-800';
		case 'student':
			return 'bg-blue-100 text-blue-800';
		default:
			return 'bg-gray-100 text-gray-800';
	}
};
</script>
