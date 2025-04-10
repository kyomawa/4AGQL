// tailwind.config.ts
import type { Config } from 'tailwindcss';

const config: Config = {
	darkMode: 'media',
	content: [
		'./components/**/*.{vue,js,ts}',
		'./layouts/**/*.{vue,js,ts}',
		'./pages/**/*.{vue,js,ts}',
		'./app.vue',
	],
	theme: {
		extend: {
			fontFamily: {
				sans: ['Poppins', 'sans-serif'],
			},
			colors: {
				violet: {
					50: '#f4f4fe',
					100: '#eceafd',
					200: '#dad8fc',
					300: '#beb8fa',
					400: '#9d90f5',
					500: '#7d63ef',
					600: '#6b42e5',
					700: '#5e33d2',
					800: '#4c28af',
					900: '#402290',
					950: '#261461',
				},
			},
			borderRadius: {
				xl: '1rem',
				'2xl': '1.5rem',
				full: '9999px',
				'20px': '20px', // arrondi personnalisé précis
			},
			boxShadow: {
				md: '0 4px 10px rgba(0,0,0,0.05)', // ombre douce
			},
		},
	},
	plugins: [],
};

export default config;
