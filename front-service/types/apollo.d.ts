// types/apollo.d.ts
import { ApolloClient, NormalizedCacheObject } from '@apollo/client/core';

declare module '#app' {
	interface NuxtApp {
		$apolloAuth: ApolloClient<NormalizedCacheObject>;
		$apolloUsers: ApolloClient<NormalizedCacheObject>;
		$apolloGrades: ApolloClient<NormalizedCacheObject>;
		$apolloClasses: ApolloClient<NormalizedCacheObject>;
	}
}

// Pour éviter l'erreur "The module is a namespace, not a type"
export {};
