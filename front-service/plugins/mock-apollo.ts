// plugins/mock-apollo.ts
import { defineNuxtPlugin } from '#app';
import { ApolloClient, InMemoryCache } from '@apollo/client/core';
import { DefaultApolloClient } from '@vue/apollo-composable';
import { makeExecutableSchema } from '@graphql-tools/schema';
import { SchemaLink } from '@apollo/client/link/schema';
import gql from 'graphql-tag';
import { useMockData } from '~/composables/useMockData';

let currentUser: { user_id: string; role: string } | null = null;

export default defineNuxtPlugin((nuxtApp) => {
	console.log('[Mock Apollo Plugin] Loaded');

	// Tenter de restaurer l'état d'authentification depuis localStorage
	if (process.client) {
		try {
			const authData = localStorage.getItem('auth');
			if (authData) {
				const parsedData = JSON.parse(authData);
				if (parsedData.user && parsedData.timestamp) {
					// Vérifier que les données ne sont pas trop anciennes (24h max)
					if (Date.now() - parsedData.timestamp < 86400000) {
						currentUser = {
							user_id: parsedData.user.id,
							role: parsedData.user.role,
						};
						console.log(
							'[Mock Apollo] Restored auth state from localStorage:',
							currentUser,
						);
					} else {
						localStorage.removeItem('auth');
					}
				}
			}
		} catch (e) {
			console.error('[Mock Apollo] Error parsing auth from localStorage:', e);
			localStorage.removeItem('auth');
		}
	}

	// Récupérer les données mockées
	const mockData = useMockData();

	// Accéder correctement aux tableaux réactifs en utilisant .value
	let usersArray = [...mockData.users.value];
	let gradesArray = [...mockData.grades.value];
	let classesArray = [...mockData.classes.value];

	console.log('Mock data loaded:', { usersArray, gradesArray, classesArray });

	const typeDefs = gql`
		type User {
			id: ID!
			email: String!
			pseudo: String!
			role: String!
		}

		type ExternalClaim {
			user_id: ID!
			role: String!
		}

		type Grade {
			id: ID!
			user_id: ID!
			course: String!
			value: Float!
			professor_id: ID!
		}

		type Class {
			id: ID!
			name: String!
			students: [ID!]!
			professors: [ID!]!
		}

		type Query {
			healthCheck: String!
			get_current_auth: ExternalClaim
			get_users: [User]
			get_user_by_id(id: ID!): User
			get_current_user: User
			get_grades(course: String): [Grade]
			get_grade_by_id(id: ID!): Grade
			get_classes(name: String): [Class]
			get_class_by_id(id: ID!): Class
			get_users_by_class_id(class_id: String): [User]
		}

		input LoginRequest {
			email: String!
			password: String!
		}

		type LoginResponse {
			user: User!
		}

		input RegisterRequest {
			email: String!
			pseudo: String!
			password: String!
			role: String
		}

		input UserInput {
			pseudo: String
			role: String
		}

		input CreateGradeRequest {
			user_id: ID!
			course: String!
			value: Float!
			professor_id: ID!
		}

		input UpdateGradeRequest {
			course: String
			value: Float
		}

		input CreateClassRequest {
			name: String!
			professors: [ID!]
		}

		input UpdateClassRequest {
			name: String
		}

		type Mutation {
			login(auth: LoginRequest!): LoginResponse!
			logout: Boolean!
			register(user: RegisterRequest!): User
			update_current_user(input: UserInput!): User
			update_user_by_id(id: ID!, input: UserInput!): User
			delete_current_user: User
			delete_user_by_id(id: ID!): User
			create_grade(grade: CreateGradeRequest!): Grade
			update_grade_by_id(id: ID!, input: UpdateGradeRequest!): Grade
			delete_grade_by_id(id: ID!): Grade
			create_class(class: CreateClassRequest!): Class
			update_class_by_id(id: ID!, class: UpdateClassRequest!): Class
			delete_class_by_id(id: ID!): Class
			enroll_student(classId: String!, studentId: String!): Class
			remove_student(classId: String!, studentId: String!): Class
		}
	`;

	const resolvers = {
		Query: {
			healthCheck: () => 'Service is Alive',
			get_current_auth: () => currentUser,
			get_users: () => usersArray,
			get_user_by_id: (_: any, { id }: { id: string }) => usersArray.find((u) => u.id === id),
			get_current_user: () => {
				if (!currentUser) return null;
				return usersArray.find((u) => u.id === currentUser?.user_id);
			},
			get_grades: (_: any, { course }: { course?: string }) => {
				let filtered = gradesArray;
				if (course) {
					filtered = gradesArray.filter((g) =>
						g.course.toLowerCase().includes(course.toLowerCase()),
					);
				}
				if (currentUser?.role === 'student') {
					return filtered.filter((g) => g.user_id === currentUser?.user_id);
				}
				return filtered;
			},
			get_grade_by_id: (_: any, { id }: { id: string }) =>
				gradesArray.find((g) => g.id === id),
			get_classes: (_: any, { name }: { name?: string }) =>
				name
					? classesArray.filter((c) => c.name.toLowerCase().includes(name.toLowerCase()))
					: classesArray,
			get_class_by_id: (_: any, { id }: { id: string }) =>
				classesArray.find((c) => c.id === id),
			get_users_by_class_id: (_: any, { class_id }: { class_id: string }) => {
				const classItem = classesArray.find((c) => c.id === class_id);
				if (!classItem) return [];
				return usersArray.filter((u) => classItem.students.includes(u.id));
			},
		},
		Mutation: {
			login: (_: any, { auth }: { auth: { email: string; password: string } }) => {
				console.log('Login attempt:', auth);
				console.log('Available users:', usersArray);

				const user = usersArray.find((u) => u.email === auth.email);
				if (!user || auth.password !== 'password') {
					console.log('Login failed: Invalid credentials');
					throw new Error('Invalid credentials');
				}

				currentUser = {
					user_id: user.id,
					role: user.role,
				};

				// Sauvegarder dans localStorage pour persister l'authentification
				if (process.client) {
					localStorage.setItem(
						'auth',
						JSON.stringify({
							user,
							timestamp: Date.now(),
						}),
					);
				}

				console.log('Login successful:', currentUser);
				return { user };
			},
			logout: () => {
				currentUser = null;

				// Nettoyer localStorage à la déconnexion
				if (process.client) {
					localStorage.removeItem('auth');
				}

				return true;
			},
			register: (
				_: any,
				{
					user,
				}: { user: { email: string; pseudo: string; password: string; role?: string } },
			) => {
				// Vérifier si l'email existe déjà
				if (usersArray.some((u) => u.email === user.email)) {
					throw new Error('Email already exists');
				}

				// Créer un nouvel utilisateur
				const newUser = {
					id: `user-${usersArray.length + 1}`,
					email: user.email,
					pseudo: user.pseudo,
					role: user.role || 'student',
				};

				usersArray.push(newUser);
				// Mettre à jour le ref
				mockData.users.value = [...usersArray];

				return newUser;
			},
			update_current_user: (
				_: any,
				{ input }: { input: { pseudo?: string; role?: string } },
			) => {
				if (!currentUser) throw new Error('Not authenticated');

				const userIndex = usersArray.findIndex((u) => u.id === currentUser?.user_id);
				if (userIndex === -1) throw new Error('User not found');

				// Mettre à jour les champs fournis
				if (input.pseudo !== undefined) {
					usersArray[userIndex].pseudo = input.pseudo;
				}

				// Seul un admin peut changer son propre rôle
				if (input.role !== undefined && currentUser.role === 'admin') {
					usersArray[userIndex].role = input.role;
				}

				// Mettre à jour le ref
				mockData.users.value = [...usersArray];

				// Mettre à jour localStorage si l'utilisateur connecté a été modifié
				if (process.client) {
					const authData = localStorage.getItem('auth');
					if (authData) {
						const parsedData = JSON.parse(authData);
						if (parsedData.user && parsedData.user.id === usersArray[userIndex].id) {
							parsedData.user = usersArray[userIndex];
							localStorage.setItem('auth', JSON.stringify(parsedData));
						}
					}
				}

				return usersArray[userIndex];
			},
			update_user_by_id: (
				_: any,
				{ id, input }: { id: string; input: { pseudo?: string; role?: string } },
			) => {
				// Vérifier si l'utilisateur courant est admin
				if (currentUser?.role !== 'admin') throw new Error('Unauthorized');

				const userIndex = usersArray.findIndex((u) => u.id === id);
				if (userIndex === -1) throw new Error('User not found');

				// Mettre à jour les champs fournis
				if (input.pseudo !== undefined) {
					usersArray[userIndex].pseudo = input.pseudo;
				}
				if (input.role !== undefined) {
					usersArray[userIndex].role = input.role;
				}

				// Mettre à jour le ref
				mockData.users.value = [...usersArray];

				// Mettre à jour localStorage si l'utilisateur connecté a été modifié
				if (process.client) {
					const authData = localStorage.getItem('auth');
					if (authData) {
						const parsedData = JSON.parse(authData);
						if (parsedData.user && parsedData.user.id === id) {
							parsedData.user = usersArray[userIndex];
							localStorage.setItem('auth', JSON.stringify(parsedData));
						}
					}
				}

				return usersArray[userIndex];
			},
			delete_current_user: () => {
				if (!currentUser) throw new Error('Not authenticated');

				const userIndex = usersArray.findIndex((u) => u.id === currentUser?.user_id);
				if (userIndex === -1) throw new Error('User not found');

				// Supprimer l'utilisateur
				const deletedUser = usersArray.splice(userIndex, 1)[0];

				// Supprimer les grades associés
				gradesArray = gradesArray.filter((g) => g.user_id !== deletedUser.id);

				// Supprimer l'utilisateur des classes
				classesArray.forEach((c) => {
					c.students = c.students.filter((studentId) => studentId !== deletedUser.id);
					c.professors = c.professors.filter((profId) => profId !== deletedUser.id);
				});

				// Mettre à jour les refs
				mockData.users.value = [...usersArray];
				mockData.grades.value = [...gradesArray];
				mockData.classes.value = [...classesArray];

				// Déconnecter l'utilisateur
				currentUser = null;

				// Nettoyer localStorage
				if (process.client) {
					localStorage.removeItem('auth');
				}

				return deletedUser;
			},
			delete_user_by_id: (_: any, { id }: { id: string }) => {
				// Vérifier si l'utilisateur courant est admin
				if (currentUser?.role !== 'admin') throw new Error('Unauthorized');

				const userIndex = usersArray.findIndex((u) => u.id === id);
				if (userIndex === -1) throw new Error('User not found');

				// Supprimer l'utilisateur
				const deletedUser = usersArray.splice(userIndex, 1)[0];

				// Supprimer les grades associés
				gradesArray = gradesArray.filter((g) => g.user_id !== deletedUser.id);

				// Supprimer l'utilisateur des classes
				classesArray.forEach((c) => {
					c.students = c.students.filter((studentId) => studentId !== deletedUser.id);
					c.professors = c.professors.filter((profId) => profId !== deletedUser.id);
				});

				// Mettre à jour les refs
				mockData.users.value = [...usersArray];
				mockData.grades.value = [...gradesArray];
				mockData.classes.value = [...classesArray];

				// Si l'utilisateur supprimé était celui connecté, le déconnecter
				if (currentUser && currentUser.user_id === id) {
					currentUser = null;
					if (process.client) {
						localStorage.removeItem('auth');
					}
				}

				return deletedUser;
			},
			create_grade: (
				_: any,
				{
					grade,
				}: {
					grade: { user_id: string; course: string; value: number; professor_id: string };
				},
			) => {
				// Vérifier si l'utilisateur courant est professeur ou admin
				if (currentUser?.role !== 'professor' && currentUser?.role !== 'admin') {
					throw new Error('Unauthorized');
				}

				// Vérifier si l'utilisateur existe
				if (!usersArray.some((u) => u.id === grade.user_id)) {
					throw new Error('Student not found');
				}

				// Vérifier si le professeur utilise son propre ID (sauf pour admin)
				if (
					currentUser?.role === 'professor' &&
					grade.professor_id !== currentUser?.user_id
				) {
					throw new Error('Professors can only create grades with their own ID');
				}

				// Créer une nouvelle note
				const newGrade = {
					id: `grade-${gradesArray.length + 1}`,
					user_id: grade.user_id,
					course: grade.course,
					value: grade.value,
					professor_id: grade.professor_id,
				};

				gradesArray.push(newGrade);
				// Mettre à jour le ref
				mockData.grades.value = [...gradesArray];

				return newGrade;
			},
			update_grade_by_id: (
				_: any,
				{ id, input }: { id: string; input: { course?: string; value?: number } },
			) => {
				// Vérifier si l'utilisateur courant est professeur ou admin
				if (currentUser?.role !== 'professor' && currentUser?.role !== 'admin') {
					throw new Error('Unauthorized');
				}

				const gradeIndex = gradesArray.findIndex((g) => g.id === id);
				if (gradeIndex === -1) throw new Error('Grade not found');

				// Vérifier si c'est le professeur qui a créé la note (sauf pour admin)
				if (
					currentUser?.role === 'professor' &&
					gradesArray[gradeIndex].professor_id !== currentUser?.user_id
				) {
					throw new Error('Professors can only update their own grades');
				}

				// Mettre à jour les champs fournis
				if (input.course !== undefined) {
					gradesArray[gradeIndex].course = input.course;
				}
				if (input.value !== undefined) {
					gradesArray[gradeIndex].value = input.value;
				}

				// Mettre à jour le ref
				mockData.grades.value = [...gradesArray];

				return gradesArray[gradeIndex];
			},
			delete_grade_by_id: (_: any, { id }: { id: string }) => {
				// Vérifier si l'utilisateur courant est professeur ou admin
				if (currentUser?.role !== 'professor' && currentUser?.role !== 'admin') {
					throw new Error('Unauthorized');
				}

				const gradeIndex = gradesArray.findIndex((g) => g.id === id);
				if (gradeIndex === -1) throw new Error('Grade not found');

				// Vérifier si c'est le professeur qui a créé la note (sauf pour admin)
				if (
					currentUser?.role === 'professor' &&
					gradesArray[gradeIndex].professor_id !== currentUser?.user_id
				) {
					throw new Error('Professors can only delete their own grades');
				}

				// Supprimer la note
				const deletedGrade = gradesArray.splice(gradeIndex, 1)[0];
				// Mettre à jour le ref
				mockData.grades.value = [...gradesArray];

				return deletedGrade;
			},
			create_class: (
				_: any,
				{ class: classData }: { class: { name: string; professors?: string[] } },
			) => {
				// Vérifier si l'utilisateur courant est professeur ou admin
				if (currentUser?.role !== 'professor' && currentUser?.role !== 'admin') {
					throw new Error('Unauthorized');
				}

				// Créer une nouvelle classe
				const newClass = {
					id: `class-${classesArray.length + 1}`,
					name: classData.name,
					students: [],
					professors: classData.professors || [],
				};

				// Ajouter l'ID du professeur courant s'il n'est pas déjà inclus
				if (
					currentUser?.role === 'professor' &&
					!newClass.professors.includes(currentUser?.user_id)
				) {
					newClass.professors.push(currentUser?.user_id);
				}

				classesArray.push(newClass);
				// Mettre à jour le ref
				mockData.classes.value = [...classesArray];

				return newClass;
			},
			update_class_by_id: (
				_: any,
				{ id, class: classData }: { id: string; class: { name?: string } },
			) => {
				// Vérifier si l'utilisateur courant est professeur ou admin
				if (currentUser?.role !== 'professor' && currentUser?.role !== 'admin') {
					throw new Error('Unauthorized');
				}

				const classIndex = classesArray.findIndex((c) => c.id === id);
				if (classIndex === -1) throw new Error('Class not found');

				// Vérifier si le professeur est associé à cette classe (sauf pour admin)
				if (
					currentUser?.role === 'professor' &&
					!classesArray[classIndex].professors.includes(currentUser?.user_id)
				) {
					throw new Error('Professors can only update their own classes');
				}

				// Mettre à jour les champs fournis
				if (classData.name !== undefined) {
					classesArray[classIndex].name = classData.name;
				}

				// Mettre à jour le ref
				mockData.classes.value = [...classesArray];

				return classesArray[classIndex];
			},
			delete_class_by_id: (_: any, { id }: { id: string }) => {
				// Vérifier si l'utilisateur courant est professeur ou admin
				if (currentUser?.role !== 'professor' && currentUser?.role !== 'admin') {
					throw new Error('Unauthorized');
				}

				const classIndex = classesArray.findIndex((c) => c.id === id);
				if (classIndex === -1) throw new Error('Class not found');

				// Vérifier si le professeur est associé à cette classe (sauf pour admin)
				if (
					currentUser?.role === 'professor' &&
					!classesArray[classIndex].professors.includes(currentUser?.user_id)
				) {
					throw new Error('Professors can only delete their own classes');
				}

				// Vérifier si la classe a des étudiants
				if (classesArray[classIndex].students.length > 0) {
					throw new Error('Cannot delete a class with enrolled students');
				}

				// Supprimer la classe
				const deletedClass = classesArray.splice(classIndex, 1)[0];
				// Mettre à jour le ref
				mockData.classes.value = [...classesArray];

				return deletedClass;
			},
			enroll_student: (
				_: any,
				{ classId, studentId }: { classId: string; studentId: string },
			) => {
				// Vérifier si l'utilisateur courant est professeur ou admin
				if (currentUser?.role !== 'professor' && currentUser?.role !== 'admin') {
					throw new Error('Unauthorized');
				}

				const classIndex = classesArray.findIndex((c) => c.id === classId);
				if (classIndex === -1) throw new Error('Class not found');

				// Vérifier si le professeur est associé à cette classe (sauf pour admin)
				if (
					currentUser?.role === 'professor' &&
					!classesArray[classIndex].professors.includes(currentUser?.user_id)
				) {
					throw new Error('Professors can only enroll students to their own classes');
				}

				// Vérifier si l'étudiant existe
				if (!usersArray.some((u) => u.id === studentId && u.role === 'student')) {
					throw new Error('Student not found');
				}

				// Vérifier si l'étudiant est déjà inscrit
				if (classesArray[classIndex].students.includes(studentId)) {
					throw new Error('Student already enrolled');
				}

				// Ajouter l'étudiant
				classesArray[classIndex].students.push(studentId);
				// Mettre à jour le ref
				mockData.classes.value = [...classesArray];

				return classesArray[classIndex];
			},
			remove_student: (
				_: any,
				{ classId, studentId }: { classId: string; studentId: string },
			) => {
				// Vérifier si l'utilisateur courant est professeur ou admin
				if (currentUser?.role !== 'professor' && currentUser?.role !== 'admin') {
					throw new Error('Unauthorized');
				}

				const classIndex = classesArray.findIndex((c) => c.id === classId);
				if (classIndex === -1) throw new Error('Class not found');

				// Vérifier si le professeur est associé à cette classe (sauf pour admin)
				if (
					currentUser?.role === 'professor' &&
					!classesArray[classIndex].professors.includes(currentUser?.user_id)
				) {
					throw new Error('Professors can only remove students from their own classes');
				}

				// Vérifier si l'étudiant est inscrit
				if (!classesArray[classIndex].students.includes(studentId)) {
					throw new Error('Student not enrolled');
				}

				// Supprimer l'étudiant
				classesArray[classIndex].students = classesArray[classIndex].students.filter(
					(id) => id !== studentId,
				);
				// Mettre à jour le ref
				mockData.classes.value = [...classesArray];

				return classesArray[classIndex];
			},
		},
	};

	const schema = makeExecutableSchema({ typeDefs, resolvers });

	const apolloClient = new ApolloClient({
		cache: new InMemoryCache(),
		link: new SchemaLink({ schema }),
	});

	nuxtApp.vueApp.provide(DefaultApolloClient, apolloClient);

	return {
		provide: {
			apollo: apolloClient,
		},
	};
});
