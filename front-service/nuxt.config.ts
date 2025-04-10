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
	css: ['~/assets/css/tailwind.css'],
	app: {
		keepalive: true,
		pageTransition: { name: 'page', mode: 'out-in' },
		head: {
			title: 'SchoolInc',
			meta: [
				{ charset: 'utf-8' },
				{ name: 'viewport', content: 'width=device-width, initial-scale=1' },
			],
			link: [
				{
					rel: 'stylesheet',
					href: 'https://fonts.googleapis.com/css2?family=Poppins:wght@400;500;600;700&display=swap',
				},
				{ rel: 'icon', type: 'image/png', href: '/logo.png' },
			],
		},
	},
	ssr: false,
	experimental: {
		payloadExtraction: false,
	},
	vite: {
		optimizeDeps: {
			include: ['vue', '@vue/apollo-composable'],
		},
		server: {
			proxy: {
				'/api/auth/graphql': {
					target: 'http://auth-service:8080',
					changeOrigin: true,
					rewrite: (path) => path.replace(/^\/api\/auth\/graphql/, '/api/auth/graphql'),
				},
				'/api/users/graphql': {
					target: 'http://users-service:8080',
					changeOrigin: true,
					rewrite: (path) => path.replace(/^\/api\/users\/graphql/, '/api/users/graphql'),
				},
				'/api/grades/graphql': {
					target: 'http://grades-service:8080',
					changeOrigin: true,
					rewrite: (path) =>
						path.replace(/^\/api\/grades\/graphql/, '/api/grades/graphql'),
				},
				'/api/classes/graphql': {
					target: 'http://classes-service:8080',
					changeOrigin: true,
					rewrite: (path) =>
						path.replace(/^\/api\/classes\/graphql/, '/api/classes/graphql'),
				},
			},
		},
	},
});
