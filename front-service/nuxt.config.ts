// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
	compatibilityDate: '2024-08-12',
	devtools: { enabled: false },
	typescript: {
		strict: true,
		typeCheck: true,
	},
	modules: ['@nuxt/eslint', '@nuxtjs/tailwindcss'],
	postcss: {
		plugins: {
			tailwindcss: {},
			autoprefixer: {},
		},
	},

	css: ['~/assets/css/tailwind.css'], // Import global CSS

	// Ajouts pour améliorer la persistance de l'authentification
	app: {
		keepalive: true,
		pageTransition: { name: 'page', mode: 'out-in' },
		head: {
			link: [
				{
					rel: 'stylesheet',
					href: 'https://fonts.googleapis.com/css2?family=Poppins:wght@400;500;600;700&display=swap',
				},
			],
		},
	},

	// Désactiver le SSR pour éviter les problèmes d'hydratation avec l'authentification
	ssr: false,

	// Options pour une meilleure gestion du state
	experimental: {
		payloadExtraction: false,
	},

	// Configuration de Vite pour optimiser les performances
	vite: {
		optimizeDeps: {
			include: ['vue', '@vue/apollo-composable'],
		},
	},
});
