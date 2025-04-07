// composables/useMockData.ts
import { ref } from 'vue';

export function useMockData() {
	// Donnée des utilisateurs
	const users = ref([
		{
			id: 'user-1',
			email: 'admin@schoolinc.com',
			pseudo: 'Admin',
			role: 'admin',
		},
		{
			id: 'user-2',
			email: 'prof@schoolinc.com',
			pseudo: 'Professor Smith',
			role: 'professor',
		},
		{
			id: 'user-3',
			email: 'student@schoolinc.com',
			pseudo: 'John Doe',
			role: 'student',
		},
		{
			id: 'user-4',
			email: 'student2@schoolinc.com',
			pseudo: 'Jane Smith',
			role: 'student',
		},
		{
			id: 'user-5',
			email: 'prof2@schoolinc.com',
			pseudo: 'Professor Johnson',
			role: 'professor',
		},
	]);

	// Données des classes
	const classes = ref([
		{
			id: 'class-1',
			name: 'Mathematics 101',
			students: ['user-3', 'user-4'],
			professors: ['user-2'],
		},
		{
			id: 'class-2',
			name: 'Physics 101',
			students: ['user-3'],
			professors: ['user-5'],
		},
		{
			id: 'class-3',
			name: 'Computer Science 101',
			students: ['user-4'],
			professors: ['user-2', 'user-5'],
		},
	]);

	// Données des notes
	const grades = ref([
		{
			id: 'grade-1',
			user_id: 'user-3',
			course: 'Mathematics 101',
			value: 15.5,
			professor_id: 'user-2',
		},
		{
			id: 'grade-2',
			user_id: 'user-3',
			course: 'Physics 101',
			value: 17.0,
			professor_id: 'user-5',
		},
		{
			id: 'grade-3',
			user_id: 'user-4',
			course: 'Mathematics 101',
			value: 16.0,
			professor_id: 'user-2',
		},
		{
			id: 'grade-4',
			user_id: 'user-4',
			course: 'Computer Science 101',
			value: 18.5,
			professor_id: 'user-5',
		},
		{
			id: 'grade-5',
			user_id: 'user-3',
			course: 'Computer Science 101',
			value: 14.0,
			professor_id: 'user-2',
		},
	]);

	return {
		users,
		classes,
		grades,
	};
}
