import { useHead } from '#imports';

export function useDarkMode() {
	const isDark = useState('darkMode', () => false);

	const toggleDark = () => {
		isDark.value = !isDark.value;
		if (process.client) {
			document.documentElement.classList.toggle('dark', isDark.value);
		}
	};

	// Si besoin, change la balise meta ou autre
	useHead({
		htmlAttrs: {
			class: isDark.value ? 'dark' : '',
		},
	});

	return { isDark, toggleDark };
}
