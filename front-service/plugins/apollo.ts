import { defineNuxtPlugin } from '#app';
import { ApolloClient, InMemoryCache, createHttpLink } from '@apollo/client/core';
import { setContext } from '@apollo/client/link/context';
import { onError } from '@apollo/client/link/error';

export default defineNuxtPlugin((nuxtApp) => {
	// Configuration des liens pour chaque service avec les URLs correctes
	const authHttpLink = createHttpLink({
		uri: 'http://localhost/api/auth/graphql',
		credentials: 'include',
		fetch: (uri, options) => {
			return fetch(uri, {
				...options,
				mode: 'no-cors',
				headers: {
					...options?.headers,
					Origin: 'http://localhost',
				},
			});
		},
	});

	const usersHttpLink = createHttpLink({
		uri: 'http://localhost/api/users/graphql',
		credentials: 'include',
		fetch: (uri, options) => {
			return fetch(uri, {
				...options,
				mode: 'no-cors',
				headers: {
					...options?.headers,
					Origin: 'http://localhost',
				},
			});
		},
	});

	const gradesHttpLink = createHttpLink({
		uri: 'http://localhost/api/grades/graphql',
		credentials: 'include',
		fetch: (uri, options) => {
			return fetch(uri, {
				...options,
				mode: 'no-cors',
				headers: {
					...options?.headers,
					Origin: 'http://localhost',
				},
			});
		},
	});

	const classesHttpLink = createHttpLink({
		uri: 'http://localhost/api/classes/graphql',
		credentials: 'include',
		fetch: (uri, options) => {
			return fetch(uri, {
				...options,
				mode: 'no-cors',
				headers: {
					...options?.headers,
					Origin: 'http://localhost',
				},
			});
		},
	});

	// Gestion des erreurs
	const errorLink = onError(({ graphQLErrors, networkError }) => {
		if (graphQLErrors) {
			graphQLErrors.forEach(({ message, locations, path }) => {
				console.error(
					`[GraphQL error]: Message: ${message}, Location: ${locations}, Path: ${path}`,
				);
			});
		}
		if (networkError) {
			console.error(`[Network error]: ${networkError}`);
		}
	});

	// Ajout des headers pour l'authentification si nécessaire
	const authLink = setContext((_, { headers }) => {
		return {
			headers: {
				...headers,
				Origin: 'http://localhost',
			},
		};
	});

	// Création des clients Apollo pour chaque service
	const authClient = new ApolloClient({
		link: authLink.concat(errorLink).concat(authHttpLink),
		cache: new InMemoryCache(),
	});

	const usersClient = new ApolloClient({
		link: authLink.concat(errorLink).concat(usersHttpLink),
		cache: new InMemoryCache(),
	});

	const gradesClient = new ApolloClient({
		link: authLink.concat(errorLink).concat(gradesHttpLink),
		cache: new InMemoryCache(),
	});

	const classesClient = new ApolloClient({
		link: authLink.concat(errorLink).concat(classesHttpLink),
		cache: new InMemoryCache(),
	});

	// Exposer les clients dans l'application
	nuxtApp.provide('apolloAuth', authClient);
	nuxtApp.provide('apolloUsers', usersClient);
	nuxtApp.provide('apolloGrades', gradesClient);
	nuxtApp.provide('apolloClasses', classesClient);
});
