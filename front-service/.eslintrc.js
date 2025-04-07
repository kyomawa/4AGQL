module.exports = {
	root: true,
	env: {
		browser: true,
		node: true,
	},
	extends: ['@nuxtjs/eslint-config-typescript', 'plugin:nuxt/recommended'],
	rules: {
		// Désactiver les erreurs de variables non définies pour les variables __VLS_*
		'no-undef': 'off',
		// Désactiver les erreurs de types pour les variables qui ont implictement un type 'any'
		'@typescript-eslint/no-explicit-any': 'off',
		'@typescript-eslint/ban-types': 'off',
		// Autres règles qui peuvent être utiles
		'vue/multi-word-component-names': 'off',
		'vue/no-v-html': 'off',
	},
};
